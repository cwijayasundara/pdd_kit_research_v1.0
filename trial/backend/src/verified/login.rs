//! SPEC FIRST — the contract of `POST /api/auth/login`, written and approved
//! BEFORE any proof body exists (hash gate in intent/decisions.md).
//!
//! The approved artifact is the eight numbered clauses on `login_flow` below.
//! Everything else here either defines ghost vocabulary the clauses use, or
//! wraps trusted code with the contract `login_flow` may rely on.
//!
//! Verify (from trial/):
//!   PATH="$PWD/verus-release:$PATH" cargo verus verify -p backend
//! Plain `cargo build` / `cargo test` still work: the verus! macro erases
//! proof code, and plain code outside verus!{} is NOT proof-checked — the
//! trusted ring (trusted.rs, handlers, models, seed) is pinned behaviorally
//! by tests/trusted_boundary.rs instead (see intent/decisions.md, D6).
//!
//! Until the proof stage, every proved fn carries #[verifier::external_body]
//! with an unimplemented!() body: its contract is TYPE-checked but ASSUMED,
//! not proved. Removing the attribute turns on the real proof obligation.

use crate::models::{Admin, AppError, Claims, Config, Customer, Expiry};
use crate::trusted::{
    check_password, expiry_from_now, generate_token, lookup_admin, lookup_customer,
    PoolHandle,
};
use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// SPEC VISIBILITY FOR PLAIN/FOREIGN TYPES
// These proxies let verus-visible code use the runtime types: our own
// models (declared outside verus!) and sqlx's pool/error.
// ---------------------------------------------------------------------------
#[verifier::external_type_specification]
pub struct ExCustomer(Customer);

#[verifier::external_type_specification]
pub struct ExAdmin(Admin);

#[verifier::external_type_specification]
pub struct ExClaims(Claims);

#[verifier::external_type_specification]
pub struct ExConfig(Config);

#[verifier::external_type_specification]
pub struct ExExpiry(Expiry);

#[verifier::external_type_specification]
pub struct ExAppError(AppError);

/// The pool is opaque to the proof: an unknown value the trusted lookups read.
/// (Generics must mirror Pool<DB>; sqlx::Error itself cannot be represented —
/// it holds a multi-trait `dyn` — so the trusted wrappers below return
/// AppError instead, folding database_error's contract into theirs.)
/// The pool handle: fully opaque to the proof (external_body stops field
/// analysis, so sqlx's private internals and trait bounds never surface).
/// The handler wraps `state.pool` in a PoolHandle (plain code) and passes it
/// through untouched; the proved flow never destructures it.
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExPoolHandle(#[verifier::reject_recursive_types] crate::trusted::PoolHandle);

// ---------------------------------------------------------------------------
// UNINTERPRETED SPEC VOCABULARY
// The verifier knows nothing about these functions beyond the contracts
// below. They stand for the database's contents and the password/signing
// machinery — the parts this trial explicitly does NOT prove.
// ---------------------------------------------------------------------------

/// The customers table, read by exact email (contents unknown to the proof).
pub uninterp spec fn customer_on_file(email: Seq<char>) -> Option<Customer>;

/// The admins table, read by exact username (contents unknown to the proof).
pub uninterp spec fn admin_on_file(username: Seq<char>) -> Option<Admin>;

/// What bcrypt would say about (password, hash) — None = unreadable hash.
pub uninterp spec fn bcrypt_verdict(password: Seq<char>, hash: Seq<char>) -> Option<bool>;

/// The claims the signer would have embedded in token `t`.
pub uninterp spec fn token_claims(token: Seq<char>, secret: Seq<char>) -> Option<Claims>;

/// A real lookup failure: sqlx itself errored (db down, broken query, ...).
pub uninterp spec fn customer_lookup_fails(email: Seq<char>) -> bool;

/// A real lookup failure on the admins table.
pub uninterp spec fn admin_lookup_fails(username: Seq<char>) -> bool;

/// A real signing failure for these claims (bad secret, jwt error, ...).
pub uninterp spec fn signing_fails(sub: int, role: Seq<char>, secret: Seq<char>) -> bool;

// ---------------------------------------------------------------------------
// GHOST STRUCT + SPEC HELPERS
// ---------------------------------------------------------------------------

/// The row login acts on: its id, its role (the string the JWT stores),
/// and its stored password hash.
pub struct Account {
    pub id: i64,
    pub role: Seq<char>,
    pub password_hash: String,
}

/// Spec view of an `Option<&str>` request field.
pub open spec fn opt_str(e: Option<&str>) -> Option<Seq<char>> {
    match e {
        Some(s) => Some(s@),
        None => None,
    }
}

/// The account the request names: email → customers row (role customer);
/// no email but a username → admins row (role admin). EMAIL WINS — a request
/// that sends both never touches the admins table.
pub open spec fn named_account(
    email: Option<Seq<char>>,
    username: Option<Seq<char>>,
) -> Option<Account> {
    match email {
        Some(e) => match customer_on_file(e) {
            Some(c) => Some(Account {
                id: c.id,
                role: "customer"@,
                password_hash: c.password_hash,
            }),
            None => None,
        },
        None => match username {
            Some(u) => match admin_on_file(u) {
                Some(a) => Some(Account {
                    id: a.id,
                    role: "admin"@,
                    password_hash: a.password_hash,
                }),
                None => None,
            },
            None => None,
        },
    }
}

/// True iff the ONE lookup the request triggers fails (email wins: a request
/// with an email never consults the admins table, and vice versa).
pub open spec fn lookup_fails(
    email: Option<Seq<char>>,
    username: Option<Seq<char>>,
) -> bool {
    match email {
        Some(e) => customer_lookup_fails(e),
        None => match username {
            Some(u) => admin_lookup_fails(u),
            None => false,
        },
    }
}

// ---------------------------------------------------------------------------
// TRUSTED BOUNDARY — contract wrappers around plain trusted code.
// external_body = the body is NOT proof-checked; the contract is ASSUMED.
// The behavior each contract claims is pinned behaviorally by
// tests/trusted_boundary.rs. A false contract here makes every promise of
// login_flow hold only on paper.
// ---------------------------------------------------------------------------

/// The customers-table lookup, exactly as the spec's `customer_on_file` says.
#[verifier::external_body]
pub async fn ex_lookup_customer(
    pool: &PoolHandle,
    email: &str,
) -> (r: Result<Option<Customer>, AppError>)
    ensures
        (r matches Ok(Some(c)) ==> {
            &&& customer_on_file(email@) matches Some(w)
            &&& w.id == c.id
            &&& w.email@ == c.email@
            &&& w.password_hash@ == c.password_hash@
        }),
        (r matches Ok(None) ==> customer_on_file(email@) is None),
        (r matches Err(AppError::Internal(_)) && customer_lookup_fails(email@)),
{
    match lookup_customer(pool.pool(), email).await {
        Ok(found) => Ok(found),
        Err(e) => Err(crate::trusted::database_error(e)),
    }
}

/// The admins-table lookup, exactly as the spec's `admin_on_file` says.
#[verifier::external_body]
pub async fn ex_lookup_admin(
    pool: &PoolHandle,
    username: &str,
) -> (r: Result<Option<Admin>, AppError>)
    ensures
        (r matches Ok(Some(a)) ==> {
            &&& admin_on_file(username@) matches Some(w)
            &&& w.id == a.id
            &&& w.username@ == a.username@
            &&& w.password_hash@ == a.password_hash@
        }),
        (r matches Ok(None) ==> admin_on_file(username@) is None),
        (r matches Err(AppError::Internal(_)) && admin_lookup_fails(username@)),
{
    match lookup_admin(pool.pool(), username).await {
        Ok(found) => Ok(found),
        Err(e) => Err(crate::trusted::database_error(e)),
    }
}

/// The bcrypt verdict: Some(v) = readable hash, v = right/wrong password;
/// Err = unreadable hash (the 500 case).
#[verifier::external_body]
pub fn ex_check_password(password: &str, hash: &str) -> (r: Result<bool, String>)
    ensures
        (r matches Ok(v) ==> bcrypt_verdict(password@, hash@) == Some(v)),
        (r is Err ==> bcrypt_verdict(password@, hash@) is None),
{
    check_password(password, hash)
}

/// The clock: expiry exactly 24 hours after the reading. (A clause change
/// here — e.g. 24h → 1h — is the spec-change test in the trial plan.)
#[verifier::external_body]
pub fn ex_expiry_from_now() -> (r: Expiry)
    ensures
        r.exp as int == r.now as int + 86_400,
{
    expiry_from_now()
}

/// Signs exactly the claims it is given; Err means a real signing failure.
#[verifier::external_body]
pub fn ex_generate_token(claims: &Claims, config: &Config) -> (r: Result<String, String>)
    ensures
        (r matches Ok(t) ==> {
            &&& token_claims(t@, config.jwt_secret@) matches Some(c)
            &&& c.sub == claims.sub
            &&& c.role@ == claims.role@
        }),
        (r is Err ==> signing_fails(claims.sub as int, claims.role@, config.jwt_secret@)),
{
    generate_token(claims, config)
}

// ---------------------------------------------------------------------------
// THE PROVED PIPELINE
// Bodies are trusted placeholders (external_body + unimplemented!()) until
// the proof stage; removing external_body turns on the real obligations.
// Only login_flow's eight clauses are the approved spec; the helper
// contracts are proof-side structure and may be refined with recorded
// decisions (intent/decisions.md, D8).
// ---------------------------------------------------------------------------

/// Decides whether the named account + password is good to log in.
/// Ok passes the row through unchanged; Unauthorized = wrong password;
/// Internal = unreadable stored hash.
#[verifier::external_body]
pub fn decide_login(a: Account, password: &str) -> (r: Result<Account, AppError>)
    ensures
        (r matches Ok(acc) ==> {
            &&& a.id == acc.id
            &&& a.role == acc.role
            &&& a.password_hash@ == acc.password_hash@
            &&& bcrypt_verdict(password@, a.password_hash@) matches Some(true)
        }),
        (r matches Err(AppError::Unauthorized)
            ==> bcrypt_verdict(password@, a.password_hash@) matches Some(false)),
        (r matches Err(AppError::Internal(_))
            ==> bcrypt_verdict(password@, a.password_hash@) is None),
{
    unimplemented!()
}

/// Issues the signed token for this account: claims are sub = the account's
/// id, role = its table's role; Err = a real signing failure.
#[verifier::external_body]
pub fn issue_token(a: Account, config: &Config) -> (r: Result<String, AppError>)
    ensures
        (r matches Ok(t) ==> {
            &&& token_claims(t@, config.jwt_secret@) matches Some(c)
            &&& c.sub == a.id
            &&& c.role@ == a.role
        }),
        (r is Err ==> signing_fails(a.id as int, a.role, config.jwt_secret@)),
{
    unimplemented!()
}

/// THE PROVED ENTRY POINT. After the proof stage the handler awaits this and
/// nothing else. Its eight clauses are the approved spec:
///
/// (1) 400 when neither identity is sent.
/// (2) A token carries exactly the named account's id and its table's role.
/// (3) The right password for a named, findable account yields a token,
///     unless signing itself failed.
/// (4) An unknown account is refused (401), unless the lookup really failed.
/// (5) A wrong password against a readable hash is refused (401), unless the
///     lookup really failed.
/// (6) An unreadable stored hash is a 500, never a silent refusal.
/// (7) An admin token can only name an admins row found by username, and
///     only when no email was sent (email always wins).
/// (8) A login named by email always carries the customer role.
#[verifier::external_body]
pub async fn login_flow(
    pool: &PoolHandle,
    config: &Config,
    email: Option<&str>,
    username: Option<&str>,
    password: &str,
) -> (r: Result<String, AppError>)
    ensures
        // (1) 400 when neither identity is sent.
        (email is None && username is None)
            ==> r matches Err(AppError::BadRequest(_)),

        // (2) A token carries exactly the named account's id and its table's role.
        // (match-expression form: verus matches-bindings do not cross `==>`.)
        (match r {
            Ok(t) => match named_account(opt_str(email), opt_str(username)) {
                Some(a) => match bcrypt_verdict(password@, a.password_hash@) {
                    Some(true) => match token_claims(t@, config.jwt_secret@) {
                        Some(c) => {
                            &&& c.sub == a.id
                            &&& c.role@ == a.role
                        },
                        None => false,
                    },
                    _ => false,
                },
                None => false,
            },
            Err(_) => true,
        }),

        // (3) Right password for a named, findable account ⇒ token, unless
        //     signing itself failed.
        (match named_account(opt_str(email), opt_str(username)) {
            Some(a) => match bcrypt_verdict(password@, a.password_hash@) {
                Some(true) => (!lookup_fails(opt_str(email), opt_str(username)))
                    ==> (r is Ok
                        || signing_fails(a.id as int, a.role, config.jwt_secret@)),
                _ => true,
            },
            None => true,
        }),

        // (4) Unknown account ⇒ refused, unless the lookup really failed.
        ((named_account(opt_str(email), opt_str(username)) is None)
            && !lookup_fails(opt_str(email), opt_str(username)))
            ==> r matches Err(AppError::Unauthorized),

        // (5) Wrong password against a readable hash ⇒ refused, unless the
        //     lookup really failed.
        (match named_account(opt_str(email), opt_str(username)) {
            Some(a) => match bcrypt_verdict(password@, a.password_hash@) {
                Some(false) => (!lookup_fails(opt_str(email), opt_str(username)))
                    ==> r matches Err(AppError::Unauthorized),
                _ => true,
            },
            None => true,
        }),

        // (6) Unreadable stored hash ⇒ 500, never a silent refusal.
        (match named_account(opt_str(email), opt_str(username)) {
            Some(a) => (bcrypt_verdict(password@, a.password_hash@) is None)
                && ((!lookup_fails(opt_str(email), opt_str(username)))
                    ==> r matches Err(AppError::Internal(_))),
            None => true,
        }),

        // (7) Admin tokens: only the admins table, only via username.
        (match r {
            Ok(t) => match token_claims(t@, config.jwt_secret@) {
                Some(c) => (c.role@ == "admin"@)
                    ==> ((email is None)
                        && (match opt_str(username) {
                            Some(us) => match admin_on_file(us) {
                                Some(a2) => a2.id == c.sub,
                                None => false,
                            },
                            None => false,
                        })),
                None => true,
            },
            Err(_) => true,
        }),

        // (8) A login named by email always carries the customer role.
        (match r {
            Ok(t) => match token_claims(t@, config.jwt_secret@) {
                Some(c) => (email is Some) ==> (c.role@ == "customer"@),
                None => true,
            },
            Err(_) => true,
        }),
{
    unimplemented!()
}

} // verus!

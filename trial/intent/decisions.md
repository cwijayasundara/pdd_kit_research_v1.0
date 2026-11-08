# Decision cards — answered by a human, before proof bodies exist

One card per decision. Answers are attributed and dated. The spec in
`backend/src/verified/login.rs` may not leave `unimplemented!()` until every
card here is answered AND the spec-approval hash at the bottom is recorded.

## D1 — Does "you only get a token with the right password" also mean you ALWAYS get one with the right password?

These are two different specs. "Only with the right password" is an
implication (token ⇒ right password); a login that refuses everyone keeps
that promise perfectly. "The right password always works" is the converse;
together they are if-and-only-if.

- [x] **Answered 2026-11-07 (human): Both ways** — matching the published
  choice. Clause 3 of the spec is the "if" direction; 500 allowed only when
  a lookup or signing actually fails, so "a login that answers 500 every
  time does NOT meet the spec."

## D2 — Scope: prove only the decision core, or the async glue too?

The kit's recommended default was: prove `decide_login`, keep the handler
trusted and tested. The published demo widened it: give the login handler
its own async spec (`login_flow`) so the glue — awaited lookups, token
signing — is proved, and the handler shrinks to unpack-and-await. Widening
means MORE trusted wrappers, so it needs a fresh sign-off.

- [ ] **Answered (human, before spec approval):** ______
      Default for this trial if unset: follow the published demo (wide), and
      accept D6's extra trust sign-offs knowingly.

## D3 — Where does the role come from?

Nothing in the request names a role. Answer: the role comes from which
window you walk up to — the customers table when an email is sent (email
wins, even if a username is also sent), otherwise the admins table for the
username. Consequence (the published quiz case): email `ana@example.com` +
username `admin` + the admin's password is a wrong-password CUSTOMER login →
401 and no token. Pinned by `email_wins_over_username_admin_password_is_401`.

- [x] **Answered 2026-11-07 (human):** role from the table; email wins.

## D4 — Do we pin today's refusals?

The two-sentence request does not ask for them, but pinning them keeps the
storefront and existing API tests working: 400 when neither email nor
username is sent; 401 unknown account; 401 wrong password (readable hash);
500 and no token when bcrypt cannot read the stored hash.

- [x] **Answered 2026-11-07 (human): yes** — these are spec clauses (5)-(6)
  and the 400 clause; they were not in the original request, and that is
  expected and recorded.

## D5 — Does the empty string count as "sent"?

The scaffold treats `Some("")` as sent (presence semantics). An alternative
treats blank as absent, which changes the 400 clause.

- [ ] **Answered (human):** ______ Default: presence semantics, and the
  e2e tests only cover real absence.

## D6 — Trust sign-offs, one per piece (accept each on its own)

For each piece: the clauses Verus will assume, what you are accepting, and
which promise "holds only on paper" if the assumption is wrong.

| # | Piece | Accepting | Breaks if false | Test (written before the test exists) |
|---|-------|-----------|-----------------|----------------------------------------|
| 1 | `Customer::find_by_email` | Successful lookup returns exactly the customers row on file for that email, or none | Every promise about which account a token names | `trusted_lookup_returns_row_on_file_or_none` |
| 2 | `Admin::find_by_username` | Same, for admins by username | Same as (1) | same test |
| 3 | `check_password` (bcrypt) | `Ok(true)` means exactly the right password; Err only on unreadable hash | Every password promise | `trusted_check_password_true_false_and_unreadable_hash` |
| 4 | `generate_token` (JWT) | Signs exactly the claims given, under the configured secret; Err = real signing failure | Token-claims clauses (2)(7)(8) | `trusted_generate_token_signs_exactly_the_claims_given` |
| 5 | `expiry_from_now` (clock) | Expiry is exactly now + 86,400s; out-of-range panic not promised | Expiry clause | `trusted_expiry_is_exactly_24h` |
| 6 | `database_error` + handler shell | 500 with sqlx's message; request fields passed through unchanged | Clause (1) and error pinning | e2e 400/401 tests |

- [ ] **Accepted by human, piece by piece (not one blanket yes):** ______

## D7 — Explicitly not promised (sign that you read this)

That the login future ever completes; how fast it answers; rate or timing
of attempts; a panic if the clock is out of range; how OTHER endpoints read
the role; whether an error message helps a human; the UI; anything nobody
wrote down. A green proof says nothing about these. Empirical checks only,
labeled evidence.

- [ ] **Signed (human):** ______

## D-toolchain — the deterministic re-run record

- Verus version/tag: `release/0.2026.11.02.8bf68ee` (z3, cargo-verus, vstd ship in release)
- Installed 2026-11-07 via `scripts/install-verus.sh`; stable path `verus-release/verus`
- Rust toolchain required: `1.99.1-aarch64-apple-darwin` (installed via `rustup toolchain install 1.99.1-aarch64-apple-darwin`)
- Smoke verified: `scripts/verify.sh` → "2 verified, 0 errors"
- Toolchain mutation sanity 2026-11-07: a real bug in the smoke file (`else { a }`) was REJECTED, pointing at the exact violated clause (`r >= b`), exit 1. (A `>=`→`>` condition change verified — correctly, since it still implements max. Recorded as a lesson: surviving a mutant can mean the mutant wasn't a bug.)
- Exact verify command used at crate level (Task 1, 2026-11-07):
  `PATH="$PWD/verus-release:$PATH" cargo verus verify -p backend` (from trial/)
  — via `scripts/verify.sh`. vstd = "=0.0.0-2026-11-01-0251" pinned in Cargo.toml.
- Re-run on a second machine without a model? date + result: ______

## D8 — How the spec was formalized (Task 1 findings, 2026-11-07)

Compiling the draft spec under Verus forced these formalization choices.
None changes a clause's meaning; all were forced by the checker (or the
release's semantics), which is exactly what Task 1 tests.

1. **Trusted-boundary form**: `#[verifier::external_fn_specification]`
   wrappers are impossible for own-crate fns ("duplicate specification").
   The boundary is instead `#[verifier::external_body]` contract wrappers in
   the spec file: body not checked, contract ASSUMED, behavior pinned by
   tests/trusted_boundary.rs.
2. **Spec-first placeholder**: `unimplemented!()` bodies fail verification
   (panic has a `false` precondition). Placeholder = `external_body` +
   `unimplemented!()`; removing `external_body` in the proof stage turns on
   the real obligation. Consequence: mutation checks (M1–M9) are only valid
   after the aimed function's `external_body` is removed.
3. **Binder scope**: verus `matches` bindings do not cross `==>` — clauses
   with binders on both sides are written as match-expressions (arm binders
   scope over the arm). Pure syntax, same meaning.
4. **Role in the spec is the JWT's string form** (`Seq<char>`), not the
   runtime `Role` enum — plain-crate types cannot be field types inside
   verus!{}. The runtime bridge (`Role::as_str().to_string()`) is trusted
   glue; the proof-stage link is `"customer".to_string()@ == "customer"@`.
5. **Pool is an opaque newtype** (`trusted::PoolHandle`, handler wraps
   `state.pool.clone()`): `sqlx::Error` is unrepresentable (multi-trait
   `dyn`) and `Pool<DB>`'s private fields block a transparent proxy — the
   opaque-combo proxy (`external_type_specification` + `external_body`, as
   vstd does for `String`) is the release's mechanism.
6. **Structure**: three proved fns — `login_flow`, `decide_login`,
   `issue_token`. The draft's `lookup_target` is folded into the spec helper
   `named_account` plus the trusted lookups; `login_core` is `issue_token`.
   Proof-side only; the eight clauses are untouched.

- [ ] **Human acknowledges the formalization (blanket, wording only):** ______

## Spec approval (the hash gate)

Approved while the five proved functions are still `unimplemented!()`.
Void if the hash changes — a changed spec is a new spec and needs a new
human approval (the proof may not silently chase a moving target).

- Spec file: `backend/src/verified/login.rs`
- `shasum -a 256` at approval: `3dfb3c6261af90307ec57b2b77ca6bba483af2b9c456712aee44acf238a5a137`
  (computed 2026-11-07 with all five proved functions still `unimplemented!()`
  behind `external_body` — awaiting human approval signature below)
- Approved by / date: ____________________
- Bodies written after this line? date: ____________________

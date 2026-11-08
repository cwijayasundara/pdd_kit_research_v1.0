//! Trusted boundary — the wrappers whose behaviour the proof takes on faith.
//!
//! Mirrors the published Pet Store trust list. Each one gets:
//!   1. an assumed contract in verified/src/login.rs, and
//!   2. a Given/When/Then test in tests/trusted_boundary.rs
//! Tests support these; nothing proves them. If one of these contracts is
//! false, every password / account-identity promise "holds only on paper."

use crate::models::{Admin, AppError, Claims, Config, Customer, Expiry};

/// What bcrypt says about `password` against the stored `hash`.
/// Errors only on a stored hash bcrypt can't read; `Ok(true)` means
/// exactly the right password.
pub fn check_password(password: &str, hash: &str) -> Result<bool, String> {
    bcrypt::verify(password, hash).map_err(|e| e.to_string())
}

/// Signs exactly the claims it is given under the configured secret.
/// Reports a signing failure as an error (surfaced as a 500).
pub fn generate_token(claims: &Claims, config: &Config) -> Result<String, String> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
}

/// Reads the clock and computes a token's expiry exactly 24 hours later.
/// Panics if the time is out of range; that panic is not promised against.
pub fn expiry_from_now() -> Expiry {
    let now = chrono::Utc::now().timestamp();
    Expiry {
        now,
        exp: now + 86_400,
    }
}

/// Turns a failed lookup into today's 500 with sqlx's message.
pub fn database_error(e: sqlx::Error) -> AppError {
    AppError::Internal(e.to_string())
}

/// Successful lookup helpers, so handler and tests share one shape.
pub async fn lookup_customer(
    pool: &sqlx::SqlitePool,
    email: &str,
) -> Result<Option<Customer>, sqlx::Error> {
    Customer::find_by_email(pool, email).await
}

pub async fn lookup_admin(
    pool: &sqlx::SqlitePool,
    username: &str,
) -> Result<Option<Admin>, sqlx::Error> {
    Admin::find_by_username(pool, username).await
}

/// Opaque pool handle for the proved flow: lets `verified/login.rs` take the
/// real pool without naming sqlx's (unrepresentable) types inside verus code.
/// The handler wraps `state.pool` (clone) and never touches it again.
pub struct PoolHandle(pub sqlx::SqlitePool);

impl PoolHandle {
    pub fn pool(&self) -> &sqlx::SqlitePool {
        &self.0
    }
}

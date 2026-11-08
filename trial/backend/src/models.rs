use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Role carried by the token — decided by which table the account is found
/// in. A plain enum so the trusted runtime and the Verus spec share one type
/// (the JWT still stores it as a string; see `as_str`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Customer,
    Admin,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Role::Customer => "customer",
            Role::Admin => "admin",
        }
    }
}

/// Customers row. Looked up by exact email.
#[derive(Debug, sqlx::FromRow)]
pub struct Customer {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
}

impl Customer {
    /// Trusted, not proved: the SQL query runs in sqlx, which Verus cannot check.
    /// Every customer promise of `login_flow` rests on it.
    pub async fn find_by_email(
        pool: &sqlx::SqlitePool,
        email: &str,
    ) -> Result<Option<Customer>, sqlx::Error> {
        sqlx::query_as::<_, Customer>(
            "SELECT id, email, password_hash FROM customers WHERE email = ?1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }
}

/// Admins row. Looked up by exact username.
#[derive(Debug, sqlx::FromRow)]
pub struct Admin {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

impl Admin {
    /// Trusted, not proved: the SQL query runs in sqlx, which Verus cannot check.
    pub async fn find_by_username(
        pool: &sqlx::SqlitePool,
        username: &str,
    ) -> Result<Option<Admin>, sqlx::Error> {
        sqlx::query_as::<_, Admin>(
            "SELECT id, username, password_hash FROM admins WHERE username = ?1",
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }
}

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
}

/// JWT claims: the token carries the account's id and its table's role.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub role: String,
    pub exp: i64,
}

/// What `expiry_from_now` returns: the clock reading and exactly 24h later.
#[derive(Debug, Clone, Copy)]
pub struct Expiry {
    pub now: i64,
    pub exp: i64,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

/// The app error enum the spec pins: today's 400 / 401 / 500.
pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "invalid credentials".into()),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

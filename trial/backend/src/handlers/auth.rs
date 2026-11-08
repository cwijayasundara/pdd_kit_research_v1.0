//! `POST /api/auth/login` — the trial's verification target.
//!
//! This file imitates the published Pet Store starting point:
//! `backend/src/handlers/auth.rs:82-127` — one handler doing every step.
//! After verification the handler must shrink to "unpack the request and
//! await `login_flow`" (see the commented shell at the bottom).

use crate::models::{AppError, Claims, TokenResponse};
use crate::trusted::{check_password, database_error, expiry_from_now, generate_token};
use crate::AppState;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}

/// PRE-VERIFICATION: the handler does everything. This is the code the
/// spec is written against (decide_login / lookup_target / login_core /
/// issue_token get factored out of exactly this logic).
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = login_impl(&state.pool, &state.config, &req).await?;
    Ok(Json(TokenResponse { token }))
}

async fn login_impl(
    pool: &sqlx::SqlitePool,
    config: &crate::models::Config,
    req: &LoginRequest,
) -> Result<String, AppError> {
    // Identity rule: if an email is sent, the customers table is picked and
    // the role is "customer" (email wins even when a username is also sent);
    // otherwise if a username is sent, the admins table is picked, role "admin".
    let (account_id, role, password_hash) = if let Some(email) = &req.email {
        match crate::trusted::lookup_customer(pool, email).await {
            Ok(Some(c)) => (c.id, crate::models::Role::Customer, c.password_hash),
            Ok(None) => return Err(AppError::Unauthorized),
            Err(e) => return Err(database_error(e)),
        }
    } else if let Some(username) = &req.username {
        match crate::trusted::lookup_admin(pool, username).await {
            Ok(Some(a)) => (a.id, crate::models::Role::Admin, a.password_hash),
            Ok(None) => return Err(AppError::Unauthorized),
            Err(e) => return Err(database_error(e)),
        }
    } else {
        return Err(AppError::BadRequest(
            "email or username required".to_string(),
        ));
    };

    match check_password(&req.password, &password_hash) {
        Ok(true) => {}
        Ok(false) => return Err(AppError::Unauthorized),
        Err(_) => return Err(AppError::Internal("unreadable password hash".to_string())),
    }

    let expiry = expiry_from_now();
    let claims = Claims {
        sub: account_id,
        role: role.as_str().to_string(),
        exp: expiry.exp,
    };

    generate_token(&claims, config)
        .map_err(|_| AppError::Internal("token signing failed".to_string()))
}

// ---------------------------------------------------------------------------
// AFTER VERIFICATION (do not apply until the proof exists and the mutants are
// rejected — proving a copy that the route never calls fails the trial):
//
// pub async fn login(
//     State(state): State<AppState>,
//     Json(req): Json<LoginRequest>,
// ) -> Result<Json<TokenResponse>, AppError> {
//     // The handler only unpacks the request and awaits login_flow.
//     let token = crate::verified::login_flow(
//         &state.pool,
//         &state.config,
//         req.email.as_deref(),
//         req.username.as_deref(),
//         &req.password,
//     )
//     .await?;
//     Ok(Json(TokenResponse { token }))
// }

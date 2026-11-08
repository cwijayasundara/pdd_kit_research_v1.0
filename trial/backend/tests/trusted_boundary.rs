//! Trusted-boundary tests and the end-to-end behaviour pin.
//!
//! These are the "existing API tests" analogue from the published demo:
//! they pin today's 200/400/401/500 matrix BEFORE verification, and must
//! keep passing AFTER the handler is rewired through the proved `login_flow`.
//! A passing test is EVIDENCE, not proof. These tests guard the trusted ring
//! (bcrypt wrapper, lookups, clock, signing); the proved core is guarded by
//! Verus, not by tests. Do not let an agent "fix" these tests to match new
//! behaviour — record a decision instead.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use backend::{build_router, AppState};
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn test_state() -> AppState {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    backend::seed::ensure_seed(&pool).await.unwrap();
    AppState {
        pool,
        config: backend::models::Config {
            jwt_secret: "trial-secret".to_string(),
        },
    }
}

async fn post_login(state: &AppState, body: Value) -> axum::response::Response {
    build_router(state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap()
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), 1_048_576)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

// ---------------------------------------------------------------------------
// End-to-end: the behaviour the spec's pinning clauses must preserve.

#[tokio::test]
async fn correct_customer_password_gets_200_and_customer_token() {
    let state = test_state().await;
    let res = post_login(&state, json!({"email": "ana@example.com", "password": "hunter2"})).await;
    assert_eq!(res.status(), StatusCode::OK);
    let body = body_json(res).await;
    let claims = decode_claims(body["token"].as_str().expect("token in body"));
    assert_eq!(claims.role, "customer");
    assert!(claims.sub >= 1);
}

#[tokio::test]
async fn wrong_password_is_401() {
    let state = test_state().await;
    let res = post_login(&state, json!({"email": "ana@example.com", "password": "hunter3"})).await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn unknown_account_is_401() {
    let state = test_state().await;
    let res = post_login(&state, json!({"email": "nobody@example.com", "password": "x"})).await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn neither_email_nor_username_is_400() {
    let state = test_state().await;
    let res = post_login(&state, json!({"password": "x"})).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn admin_login_via_username_gets_admin_token() {
    let state = test_state().await;
    let res = post_login(&state, json!({"username": "admin", "password": "admin123"})).await;
    assert_eq!(res.status(), StatusCode::OK);
    let body = body_json(res).await;
    let claims = decode_claims(body["token"].as_str().unwrap());
    assert_eq!(claims.role, "admin");
}

/// The published quiz case: email wins over username, so the admin password
/// is checked against the customer row and must be refused with 401.
#[tokio::test]
async fn email_wins_over_username_admin_password_is_401() {
    let state = test_state().await;
    let res = post_login(
        &state,
        json!({"email": "ana@example.com", "username": "admin", "password": "admin123"}),
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------
// Trusted ring: these wrappers are what the proof assumes. If one of these
// contracts is false, every password / identity promise holds only on paper.

#[tokio::test]
async fn trusted_check_password_true_false_and_unreadable_hash() {
    let state = test_state().await;
    let (hash,): (String,) = sqlx::query_as(
        "SELECT password_hash FROM customers WHERE email = 'ana@example.com'",
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    // Given the seeded customer hash, when the right password is checked,
    // then it is Ok(true) — exactly.
    assert_eq!(backend::trusted::check_password("hunter2", &hash), Ok(true));
    // Given a near-miss, then Ok(false).
    assert_eq!(backend::trusted::check_password("hunter3", &hash), Ok(false));
    // Given the empty string, then Ok(false) (bcrypt reads the hash fine).
    assert_eq!(backend::trusted::check_password("", &hash), Ok(false));
    // Given a hash bcrypt cannot read, then Err.
    assert!(backend::trusted::check_password("x", "not-a-real-hash").is_err());
}

#[tokio::test]
async fn trusted_expiry_is_exactly_24h() {
    let e = backend::trusted::expiry_from_now();
    assert_eq!(e.exp as i64, e.now as i64 + 86_400);
}

#[tokio::test]
async fn trusted_generate_token_signs_exactly_the_claims_given() {
    let state = test_state().await;
    let claims = backend::models::Claims {
        sub: 7,
        role: "customer".into(),
        exp: backend::trusted::expiry_from_now().exp,
    };
    let token = backend::trusted::generate_token(&claims, &state.config).unwrap();
    let decoded = decode_claims(&token);
    assert_eq!(decoded.sub, 7);
    assert_eq!(decoded.role, "customer");
}

#[tokio::test]
async fn trusted_lookup_returns_row_on_file_or_none() {
    let state = test_state().await;
    let found = backend::trusted::lookup_customer(&state.pool, "ana@example.com")
        .await
        .unwrap();
    assert!(found.is_some());
    let none = backend::trusted::lookup_customer(&state.pool, "nobody@example.com")
        .await
        .unwrap();
    assert!(none.is_none());
}

// ---------------------------------------------------------------------------

fn decode_claims(token: &str) -> backend::models::Claims {
    use jsonwebtoken::{DecodingKey, Validation};
    jsonwebtoken::decode::<backend::models::Claims>(
        token,
        &DecodingKey::from_secret(b"trial-secret"),
        &Validation::default(),
    )
    .unwrap()
    .claims
}

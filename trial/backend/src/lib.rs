pub mod handlers;
pub mod models;
pub mod seed;
pub mod trusted;
// Task 1 of the published build plan: the verified module now compiles as
// part of this crate. Under plain `cargo build/test` the verus! macro erases
// proof code; under `cargo verus verify` the contracts in verified/login.rs
// are checked. See trial/README.md -> Verification ladder.
//
// The lint allow is scoped here (not in login.rs) so the spec file stays
// byte-identical to the hash recorded at the approval gate. Plain cargo sees
// `unimplemented!()` placeholder bodies (unused params) and structural proxy
// fields that Verus requires but Rust never reads — contract, not dead weight.
// This module is allowed to change in the proof stage, where these lints get
// their signal back.
#[allow(unused_variables, dead_code)]
pub mod verified;

use axum::routing::post;
use axum::Router;
use models::Config;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/auth/login", post(handlers::auth::login))
        .with_state(state)
}

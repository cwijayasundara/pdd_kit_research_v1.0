use backend::{build_router, seed::ensure_seed, AppState};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://petstore.db".to_string());
    let opts = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(opts).await?;

    sqlx::migrate!().run(&pool).await?;
    ensure_seed(&pool).await?;

    let state = AppState {
        pool,
        config: backend::models::Config {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-me".to_string()),
        },
    };

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

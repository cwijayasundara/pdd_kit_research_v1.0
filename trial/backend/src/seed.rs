//! Seed data matching the published Pet Store examples:
//! admin `admin` / `admin123`, customer `ana@example.com` / `hunter2`.
//! Cost 4 is fast for tests; real deployments should use bcrypt::DEFAULT_COST.

use sqlx::SqlitePool;

pub async fn ensure_seed(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let admin_hash = bcrypt::hash("admin123", 4).expect("hash admin seed");
    let customer_hash = bcrypt::hash("hunter2", 4).expect("hash customer seed");

    sqlx::query("INSERT OR IGNORE INTO admins (username, password_hash) VALUES ('admin', ?1)")
        .bind(&admin_hash)
        .execute(pool)
        .await?;

    sqlx::query(
        "INSERT OR IGNORE INTO customers (email, password_hash) VALUES ('ana@example.com', ?1)",
    )
    .bind(&customer_hash)
    .execute(pool)
    .await?;

    Ok(())
}

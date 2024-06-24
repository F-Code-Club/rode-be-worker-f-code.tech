use once_cell::sync::Lazy;

use super::env_or_default;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    env_or_default(
        "DATABASE_URL",
        "postgres://user:password@host/database".to_string(),
    )
});

pub static SERVER_PORT: Lazy<u16> = Lazy::new(|| env_or_default("WORKER_PORT", 8080));

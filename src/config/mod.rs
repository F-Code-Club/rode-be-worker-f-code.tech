use std::{env, str::FromStr};

pub mod server;

pub fn env_or_default<T: FromStr>(env_name: &'static str, default: T) -> T {
    match env::var(env_name) {
        Err(_) => default,
        Ok(raw) => match raw.replace("\\n", "\n").parse() {
            Ok(value) => value,
            Err(_) => default,
        },
    }
}

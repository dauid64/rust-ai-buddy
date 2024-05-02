use std::{env, sync::OnceLock};

use crate::Result;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })

}

pub struct Config {
    pub openai_api_key: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(
            Config {
                openai_api_key: get_env("OPENAI_API_KEY")?,
            }
        )
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| name.into())
}
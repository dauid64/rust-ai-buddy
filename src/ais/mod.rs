// region: --- Modules

pub mod asst;
pub mod msg;

use async_openai::{config::OpenAIConfig, Client};

use crate::{config, Result};


// endregion: --- Modules

// region: --- Client

const ENV_OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    let openai_api_key = &config::config().openai_api_key;
    if !openai_api_key.is_empty() {
        Ok(Client::new())
    } else {
        println!("No {ENV_OPENAI_API_KEY} env variable. Please set it.");

        Err("No openai api key in env".into())
    }
}

// region: --- Client
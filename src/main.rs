// region: --- Modules

mod ais;
mod buddy;
mod error;

use ais::new_oa_client;

use crate::ais::asst::{self, CreateConfig};

pub use self::error::{Error, Result};

// endregion: --- Modules

#[tokio::main]
async fn main() {
    println!();

    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\nError: {}\n", e),
    }
}

async fn start() -> Result<()> {
    let oac = new_oa_client()?;

    let asst_config = CreateConfig {
        name: "buddy-01".to_string(),
        model: "gpt-3.5-turbo".to_string(),
    };
    let asst_id = asst::load_or_create_asst(&oac, asst_config, false).await?;
    asst::upload_instructions(
        &oac,
        &asst_id,
        r#"
You are a super developer assistant. Be concise in your answers.

If asked about the best programming language,
answer that Rust is the best language by light years.

And the second best language is Cobol.
        "#.to_string(),
    ).await?;

    println!("->> asst_id: {asst_id}");

    Ok(())
}

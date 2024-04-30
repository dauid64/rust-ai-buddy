// region: --- Modules

mod ais;
mod buddy;
mod error;
mod utils;

use ais::new_oa_client;

use crate::ais::asst::{self, create_thread, CreateConfig};

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
    println!("->> hello world");

    Ok(())
}

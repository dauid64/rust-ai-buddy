// region: --- Modules

mod ais;
mod buddy;
mod error;
mod utils;

use ais::new_oa_client;
use buddy::Buddy;
use textwrap::wrap;

use crate::{ais::asst::{self, create_thread, CreateConfig}, utils::cli::{ico_error, ico_res, prompt, txt_res}};

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

const DEFAULT_DIR: &str = "buddy";

// region --- Types

/// Input Command from the user
#[derive(Debug)]
enum Cmd {
    Quit,
    Chat(String),
    RefreshAll,
    RefreshConv,
    RefreshInst,
    RefreshFiles,
}

impl Cmd {
    fn from_input(input: impl Into<String>) -> Self {
        let input = input.into();

        if input == "/q" {
            Self::Quit
        } else if input == "/r" || input == "/ra" {
            Self::RefreshAll
        } else if input == "/ri" {
            Self::RefreshInst
        } else if input == "/rf" {
            Self::RefreshFiles
        } else if input == "/rc" {
            Self::RefreshConv
        } else {
            Self::Chat(input.to_string())
        }
    }
}

// endregion: --- Types

async fn start() -> Result<()> {
    let mut buddy = Buddy::init_from_dir(DEFAULT_DIR, false).await?;

    let mut conv = buddy.load_or_create_conv(false).await?;

    loop {
        println!();
        let input = prompt("Ask away")?;
        let cmd = Cmd::from_input(input);

        match cmd {
            Cmd::Quit => break,
            Cmd::Chat(msg) => {
                let res = buddy.chat(&conv, &msg).await?;
                let res = wrap(&res, 80).join("\n");
                println!("{} {}", ico_res(), txt_res(res));
            },
            other => println!("{} command not supported {other:?}", ico_error())
        }
    }

    println!("->> buddy {} - conv {conv:?}", buddy.name());

    Ok(())
}

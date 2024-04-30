// region --- Modules

mod config;

use std::{fs, path::{Path, PathBuf}};

use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

use crate::{ais::{asst::{self, AsstId, ThreadId}, new_oa_client, OaClient}, utils::{cli::ico_check, files::{ensure_dir, load_from_toml, read_to_string}}, Result};

use self::config::Config;

// endregion: --- Modules

const BUDDY_TOML: &str = "buddy.toml";

#[derive(Debug)]
pub struct Buddy {
    dir: PathBuf,
    oac: OaClient,
    asst_id: AsstId,
    config: Config,
}

#[derive(Debug, From, Deref, Serialize, Deserialize)]
pub struct Conv {
    thread_id: ThreadId,
}

/// Public functions
impl Buddy{
    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub async fn init_from_dir(dir: impl AsRef<Path>, recreate_asst: bool) -> Result<Self> {
        let dir = dir.as_ref();

        // -- Load from the directory
        let config: Config = load_from_toml(dir.join(BUDDY_TOML))?;

        // -- Get or Create the OpenAI Assistant
        let oac = new_oa_client()?;
        let asst_id = asst::load_or_create_asst(&oac, (&config).into(), recreate_asst).await?;

        // -- Create buddy
        let buddy = Buddy {
            dir: dir.to_path_buf(),
            oac,
            asst_id,
            config
        };

        // -- Upload instructions
        buddy.upload_instructions().await?;

        // -- Upload files
        // TODO - upload files

        Ok(buddy)
    }

    pub async fn upload_instructions(&self) -> Result<bool> {
        let file = self.dir.join(&self.config.instructions_file);
        if file.exists() {
            let inst_content = read_to_string(&file)?;
            asst::upload_instructions(&self.oac, &self.asst_id, inst_content).await?;
            println!("{} Instructions uploaded", ico_check());
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Private functions
impl Buddy{
    fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = self.dir.join(".buddy");
        ensure_dir(&data_dir)?;
        Ok(data_dir)
    }

    fn data_files_dir(&self) -> Result<PathBuf> {
        let dir = self.data_dir()?.join("files");
        ensure_dir(&dir)?;
        Ok(dir)
    }
}
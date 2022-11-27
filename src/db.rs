pub mod books;

use anyhow::{anyhow, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

use self::books::Book;

const DATA_DIR: &str = ".local/share";
const APP_DATA_DIR: &str = "bkm";
const FILE_NAME: &str = "db.db";

#[derive(Debug)]
pub struct Db {
    pub db_file_path: PathBuf,
}

impl Db {
    pub fn new(custom_path: Option<PathBuf>) -> Result<Db> {
        let db_file_path = Db::get_or_build_db(custom_path)?;
        Ok(Db { db_file_path })
    }

    pub fn set_up_tables(&self) -> Result<()> {
        Book::create_table(&self.db_file_path)?;
        Ok(())
    }

    pub fn get_or_build_db(custom_path: Option<PathBuf>) -> Result<PathBuf> {
        match custom_path {
            Some(path) => {
                if !path.exists() {
                    fs::create_dir_all(&path)?;
                }

                return Ok(path);
            }
            None => {}
        }

        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_data_dir = path.join(DATA_DIR);
                let app_data_dir = home_data_dir.join(APP_DATA_DIR);

                if !home_data_dir.exists() {
                    fs::create_dir_all(&home_data_dir)?;
                }

                if !app_data_dir.exists() {
                    fs::create_dir(&app_data_dir)?;
                }

                let db_file_path = &app_data_dir.join(FILE_NAME);

                Ok(db_file_path.to_path_buf())
            }
            None => Err(anyhow!("No $HOME directory found for db")),
        }
    }
}

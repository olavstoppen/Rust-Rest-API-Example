use sled::{Config, Db};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Init User DB and Open
pub fn init_sled_user_db() -> Result<Db, Box<dyn Error>> {
    let db_path = Path::new("db/users");

    // Create the database directory if it doesn't exist
    if !db_path.exists() {
        fs::create_dir_all(db_path)?;
    }

    let config = Config::default().path(db_path);
    let db = config.open()?;
    Ok(db)
}
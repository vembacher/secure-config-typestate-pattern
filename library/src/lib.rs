mod config;

use std::fs::OpenOptions;
use std::path::{PathBuf};
pub use crate::config::Configuration;

pub fn run(config_path: PathBuf) {
    let f = OpenOptions::new()
        .read(true)
        .open(&config_path)
        .expect(format!("No configuration file at specified path {:?}.", config_path).as_str());
    let config = Configuration::from_file(f);
    println!("Successfully ran! with {:?}", config);
}

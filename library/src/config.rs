use std::fs::{OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use typed_builder::TypedBuilder;
use serde::{Serialize, Deserialize};

#[derive(TypedBuilder)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Configuration {
    secure: bool,
}

impl Configuration {
    pub(crate) fn from_file<F: Read>(path: F) -> Configuration {
        serde_json::from_reader::<F, Configuration>(path)
            .unwrap()
            .validated()
    }

    pub fn to_file(&self, path: &str) {
        let path = PathBuf::from(path);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .expect(format!("Could not write configuration to file {:?}", path).as_str());
        let data = serde_json::to_string(self).unwrap();
        f.write(data.as_bytes()).unwrap();
    }

    pub(crate) fn validated(self) -> Configuration {
        assert!(self.secure);
        self
    }
}

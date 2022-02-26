use crate::classpath::{entry::Entry, entry::absolute};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fmt;

/// 目录形式的类路径
pub struct DirEntry {
    abs_dir: String
}

impl DirEntry {
    pub fn new(path: &str) -> Self {
        DirEntry {
            abs_dir: absolute(path)
        }
    }
}

impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let path = Path::new(&self.abs_dir);
        let new_path = path.join(class_name);
        let mut file = match File::open(&new_path) {
            Ok(file) => file,
            Err(err) => return Err(format!("{} not found: {}", class_name, err.to_string()))
        };
        let mut vec: Vec<u8> = vec![];
        file.read_to_end(&mut vec).map_err(|err| err.to_string())?;

        Ok(vec)
    }
}

impl fmt::Display for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abs_dir)
    }
}

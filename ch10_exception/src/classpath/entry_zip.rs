use crate::classpath::entry::{Entry, absolute};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fmt;
use zip::ZipArchive;

/// ZIP 或 JAR 文件形式的类路径
pub struct ZipEntry {
    abs_path: String,
    zip_archive: ZipArchive<File>,
}

impl ZipEntry {
    pub fn new(path: &str) -> Self {
        let abs_path = absolute(path);
        let path = Path::new(&abs_path);
        let zip_file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't open {}: {}", &path.display(),
                               err.to_string())
        };

        ZipEntry {
            abs_path,
            zip_archive: ZipArchive::new(zip_file).unwrap(),
        }
    }
}

impl Entry for ZipEntry {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let archive = &mut self.zip_archive;
        let mut file = match archive.by_name(&class_name) {
            Ok(file) => file,
            Err(err) => return Err(format!("{} not found: {}", class_name, err.to_string()))
        };
        let mut vec: Vec<u8> = vec![];
        file.read_to_end(&mut vec).map_err(|err| err.to_string())?;

        Ok(vec)
    }
}

impl fmt::Display for ZipEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abs_path)
    }
}

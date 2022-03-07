use crate::classpath::entry::{Entry, absolute};
use std::path::{Path, MAIN_SEPARATOR};
use std::fs::File;
use std::io::Read;
use std::fmt;

/// ZIP 或 JAR 文件形式的类路径
pub struct ZipEntry {
    abs_path: String
}

impl ZipEntry {
    pub fn new(path: &str) -> Self {
        ZipEntry {
            abs_path: absolute(path)
        }
    }
}

impl Entry for ZipEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let name = Path::new(&self.abs_path);
        let zip_file = match File::open(&name) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't open {}: {}", &name.display(),
                               err.to_string())
        };
        // ZipArchive 解压后的名称以 / 分割，传入的 class_name 如果包含 \ 替换成 / 比较
        let class_name = class_name.to_string().replace(MAIN_SEPARATOR, "/");

        let mut archive = zip::ZipArchive::new(zip_file).unwrap();
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

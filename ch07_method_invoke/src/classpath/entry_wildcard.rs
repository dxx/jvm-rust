use crate::classpath::{
    entry::Entry,
    entry_zip::ZipEntry,
    entry::PATH_SEPARATOR
};
use std::fs;
use std::fmt;

/// 处理以 * 结尾的类路径
pub struct WildcardEntry {
    entries: Vec<Box<dyn Entry>>
}

impl WildcardEntry {
    pub fn new(path: &str) -> Self {
        // 移除 *
        let base_dir = &path[..path.len() - 1];

        let dir = match fs::read_dir(base_dir) {
            Ok(dir) => dir,
            Err(err) => panic!("Couldn't open {}: {}", base_dir,
                               err.to_string())
        };
        let convert = |entry| -> Box<dyn Entry> {
            Box::new(entry)
        };
        let mut entries = vec![];
        for dir_entry in dir {
            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                continue;
            }
            let p = path.to_str().unwrap();
            if p.ends_with(".jar") || p.ends_with(".JAR") {
                let zip_entry = ZipEntry::new(&path.to_str().unwrap());
                entries.push(convert(zip_entry));
            }
        }

        WildcardEntry {
            entries
        }
    }
}

impl Entry for WildcardEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &self.entries {
            match entry.read_class(class_name) {
                Ok(data) => {
                    return Ok(data);
                },
                Err(_err) => {}
            }
        }
        Err(format!("{} not found", class_name))
    }
}

impl fmt::Display for WildcardEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec = vec![];
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}

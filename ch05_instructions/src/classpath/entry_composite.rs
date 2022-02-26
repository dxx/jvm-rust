use crate::classpath::entry::{
    Entry, new_entry, PATH_SEPARATOR
};
use std::fmt;

/// 由多个 Entry 组成
pub struct CompositeEntry {
    entries: Vec<Box<dyn Entry>>
}

impl CompositeEntry {
    pub fn new(path_list: &str) -> Self {
        let path_list = path_list.split(PATH_SEPARATOR);
        let mut entries = vec![];
        for path in path_list {
            entries.push(new_entry(path))
        }
        CompositeEntry {
            entries
        }
    }
}

impl Entry for CompositeEntry {
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

impl fmt::Display for CompositeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec = vec![];
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}

use crate::classpath::entry::{new_entry, Entry, PATH_SEPARATOR};
use std::fmt;

/// 组合形式的类路径：由多个 Entry 组成
/// 适用于由 PATH_SEPARATOR 分隔的多个路径，如：aaa.jar:bbb/*:ccc
pub struct CompositeEntry {
    entries: Vec<Box<dyn Entry>>,
}

impl CompositeEntry {
    /// 按 PATH_SEPARATOR 分割路径列表，逐个创建对应的 Entry
    pub fn new(path_list: &str) -> Self {
        let path_list = path_list.split(PATH_SEPARATOR);
        let mut entries = vec![];
        for path in path_list {
            entries.push(new_entry(path))
        }
        CompositeEntry { entries }
    }
}

impl Entry for CompositeEntry {
    /// 依次尝试每个子 Entry，找到则返回
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &mut self.entries {
            match entry.read_class(class_name) {
                Ok(data) => {
                    return Ok(data);
                }
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

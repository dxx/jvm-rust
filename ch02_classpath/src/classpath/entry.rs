use crate::classpath::{
    entry_composite::CompositeEntry, entry_dir::DirEntry, entry_wildcard::WildcardEntry,
    entry_zip::ZipEntry,
};
use std::fmt;
use std::path::Path;

// 不同操作系统下 classpath 中多个路径的分隔符不同：Windows 用 ';'，类 Unix 用 ':'
#[cfg(windows)]
pub const PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
pub const PATH_SEPARATOR: char = ':';

/// 类路径项抽象：能够根据类名读取对应的 class 字节数据
pub trait Entry: fmt::Display {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String>;
}

/// 获取指定路径的绝对路径
pub fn absolute(path: &str) -> String {
    let path = Path::new(&path);
    match path.canonicalize() {
        Ok(p) => p.to_str().unwrap().to_string(),
        Err(e) => {
            panic!("{}", e);
        }
    }
}

/// 根据传入的 path 创建对应的 Entry
/// -classpath aaa1/bbb1;aaa2/bbb2 => CompositeEntry
/// -classpath aaa/*               => WildcardEntry
/// -classpath aaa.jar             => ZipEntry
/// -classpath aaa                 => DirEntry
pub fn new_entry(path: &str) -> Box<dyn Entry> {
    if path.contains(PATH_SEPARATOR) {
        return Box::new(CompositeEntry::new(path));
    }
    if path.ends_with("*") {
        return Box::new(WildcardEntry::new(path));
    }
    if path.ends_with(".jar")
        || path.ends_with(".JAR")
        || path.ends_with(".zip")
        || path.ends_with(".ZIP")
    {
        return Box::new(ZipEntry::new(path));
    }
    Box::new(DirEntry::new(path))
}

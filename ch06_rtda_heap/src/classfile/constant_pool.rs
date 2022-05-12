pub mod cp_class;
mod cp_utf8;
mod cp_name_and_type;
mod cp_invoke_dynamic;
pub mod cp_member_ref;
pub mod cp_numeric;
pub mod cp_string;

use crate::types::RcRefCell;
use crate::classfile::ClassReader;
use self::cp_class::ConstantClassInfo;
use self::cp_utf8::ConstantUtf8Info;
use self::cp_member_ref::{ConstantFieldRefInfo, ConstantMethodRefInfo, ConstantInterfaceMethodRefInfo};
use self::cp_numeric::{ConstantIntegerInfo, ConstantFloatInfo, ConstantLongInfo, ConstantDoubleInfo};
use self::cp_string::ConstantStringInfo;
use self::cp_name_and_type::ConstantNameAndTypeInfo;
use self::cp_invoke_dynamic::{ConstantMethodHandleInfo, ConstantMethodTypeInfo, ConstantInvokeDynamicInfo};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// pub type ConstantPool = Vec<Option<Box<dyn ConstantInfo>>>;

#[derive(Default)]
pub struct ConstantPool {
    infos: Vec<Option<Box<dyn ConstantInfo>>>,

    /// 存储 CONSTANT_Class_info 常量映射
    class_info_map: HashMap<u16, ConstantClassInfo>,
    /// 存储 CONSTANT_Utf8_info 常量映射
    utf8_info_map: HashMap<u16, ConstantUtf8Info>,
}

impl ConstantPool {
    pub fn put_constant_info(&mut self, info: Option<Box<dyn ConstantInfo>>) {
        self.infos.push(info);
    }

    pub fn get_constant_info(&self, index: usize) -> &Option<Box<dyn ConstantInfo>>{
        match self.infos.get(index) {
            Some(info) => {
                info
            },
            None => {
                panic!("Invalid constant pool index: {}!", index)
            }
        }
    }

    pub fn constant_len(&self) -> usize {
        self.infos.len()
    }

    pub fn get_class_name(&self, index: u16) -> String {
        match self.class_info_map.get(&index) {
            Some(info) => info.name(),
            None => "".to_string()
        }
    }

    pub fn get_utf8(&self, index: u16) -> String {
        match self.utf8_info_map.get(&index) {
            Some(info) => info.str(),
            None => "".to_string()
        }
    }
    
    pub fn get_name_and_type(&self, index: usize) -> (String, String) {
        let info = self.get_constant_info(index).as_ref().unwrap();
        let name_type_info = info.as_any().downcast_ref::<ConstantNameAndTypeInfo>().unwrap();
        (self.get_utf8(name_type_info.name_index()), self.get_utf8(name_type_info.descriptor_index()))
    }
}

pub fn read_constant_pool(reader: &mut ClassReader) -> RcRefCell<ConstantPool> {
    let cp_count = reader.read_u16();
    let cp = Rc::new(RefCell::new(ConstantPool::default()));
    // 第一个元素无效
    cp.borrow_mut().infos.push(None);
    // 常量池索引从 1 到 constant_pool_count - 1.
    let mut i = 1;
    loop {
        if i == cp_count {
            break;
        }
        let constant_info = read_constant_info(reader, i, cp.clone());
        // http://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4.5
        // All 8-byte constants take up two entries in the constant_pool table of the class file.
        // If a CONSTANT_Long_info or CONSTANT_Double_info structure is the item in the constant_pool
        // table at index n, then the next usable item in the pool is located at index n+2.
        // The constant_pool index n+1 must be valid but is considered unusable.
        match (&constant_info).tag() {
            // CONSTANT_Long_info 和 CONSTANT_Double_info 各占两个位置
            CONSTANT_LONG | CONSTANT_DOUBLE => {
                cp.borrow_mut().infos.push(Some(constant_info));
                cp.borrow_mut().infos.push(None);
                i += 1;
            }
            _ => {
                cp.borrow_mut().infos.push(Some(constant_info));
            }
        }

        i += 1;
    }

    cp
}

/// Constant pool tags
pub const CONSTANT_UTF8: u8                    = 1;
pub const CONSTANT_INTEGER: u8                 = 3;
pub const CONSTANT_FLOAT: u8                   = 4;
pub const CONSTANT_LONG: u8                    = 5;
pub const CONSTANT_DOUBLE: u8                  = 6;
pub const CONSTANT_CLASS: u8                   = 7;
pub const CONSTANT_STRING: u8                  = 8;
pub const CONSTANT_FIELD_REF: u8               = 9;
pub const CONSTANT_METHOD_REF: u8              = 10;
pub const CONSTANT_INTERFACE_METHOD_REF: u8    = 11;
pub const CONSTANT_NAME_AND_TYPE: u8           = 12;
pub const CONSTANT_METHOD_HANDLE: u8           = 15;
pub const CONSTANT_METHOD_TYPE: u8             = 16;
pub const CONSTANT_INVOKE_DYNAMIC: u8          = 18;

pub trait ConstantInfo {
    fn read_info(&mut self, reader: &mut ClassReader);

    /// 获取标志
    fn tag(&self) -> u8;

    fn as_any(&self) -> &dyn std::any::Any;
}

fn read_constant_info(reader: &mut ClassReader, i: u16, cp: RcRefCell<ConstantPool>) -> Box<dyn ConstantInfo> {
    let tag = reader.read_u8();
    let mut c = new_constant_info(reader, tag, i, cp);
    match (&c).tag() {
        // CONSTANT_Utf8_info、CONSTANT_Class_info 在创建之后立即调用 read_info
        CONSTANT_UTF8 | CONSTANT_CLASS => {},
        _ => {
            c.read_info(reader)
        }
    }
    c
}

fn new_constant_info(reader: &mut ClassReader, tag: u8, i: u16, cp: RcRefCell<ConstantPool>) -> Box<dyn ConstantInfo> {
    match tag {
        CONSTANT_CLASS => {
            let mut b = Box::new(ConstantClassInfo::new(cp.clone()));
            // 立即调用
            b.read_info(reader);
            cp.borrow_mut().class_info_map.insert(i, *b.clone());
            b
        },
        CONSTANT_FIELD_REF => Box::new(ConstantFieldRefInfo::new(cp)),
        CONSTANT_METHOD_REF => Box::new(ConstantMethodRefInfo::new(cp)),
        CONSTANT_INTERFACE_METHOD_REF => Box::new(ConstantInterfaceMethodRefInfo::new(cp)),
        CONSTANT_STRING => Box::new(ConstantStringInfo::new(cp)),
        CONSTANT_INTEGER => Box::new(ConstantIntegerInfo::default()),
        CONSTANT_FLOAT => Box::new(ConstantFloatInfo::default()),
        CONSTANT_LONG => Box::new(ConstantLongInfo::default()),
        CONSTANT_DOUBLE => Box::new(ConstantDoubleInfo::default()),
        CONSTANT_NAME_AND_TYPE => Box::new(ConstantNameAndTypeInfo::default()),
        CONSTANT_UTF8 => {
            let mut b = Box::new(ConstantUtf8Info::default());
            // 立即调用
            b.read_info(reader);
            cp.borrow_mut().utf8_info_map.insert(i, *b.clone());
            b
        },
        CONSTANT_METHOD_HANDLE => Box::new(ConstantMethodHandleInfo::default()),
        CONSTANT_METHOD_TYPE => Box::new(ConstantMethodTypeInfo::default()),
        CONSTANT_INVOKE_DYNAMIC => Box::new(ConstantInvokeDynamicInfo::default()),
        _ => panic!("{}", "java.lang.ClassFormatError: constant pool tag!")
    }
}

pub mod class_reader;
pub mod constant_pool;
pub mod member_info;
pub mod attribute_info;

/// ClassFile {
///     u4             magic;
///     u2             minor_version;
///     u2             major_version;
///     u2             constant_pool_count;
///     cp_info        constant_pool[constant_pool_count-1];
///     u2             access_flags;
///     u2             this_class;
///     u2             super_class;
///     u2             interfaces_count;
///     u2             interfaces[interfaces_count];
///     u2             fields_count;
///     field_info     fields[fields_count];
///     u2             methods_count;
///     method_info    methods[methods_count];
///     u2             attributes_count;
///     attribute_info attributes[attributes_count];
/// }

use crate::types::RcRefCell;
use crate::classfile::constant_pool::read_constant_pool;
use crate::classfile::attribute_info::read_attributes;
use self::class_reader::ClassReader;
use self::constant_pool::ConstantPool;
use self::member_info::MemberInfo;
use self::attribute_info::AttributeInfo;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ClassFile {
    /// magic: u32,
    minor_version: u16, /// 次版本号
    major_version: u16, /// 主版本号
    constant_pool: RcRefCell<ConstantPool>, /// 常量池
    access_flags: u16, /// 访问标志
    this_class: u16, /// 类索引
    super_class: u16, /// 超类索引
    interfaces: Vec<u16>, /// 接口索引表
    fields: Vec<MemberInfo>,
    methods: Vec<MemberInfo>,
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl ClassFile {
    pub fn parse(class_data: Vec<u8>) -> Result<ClassFile, String>{
        let mut class_reader = ClassReader::new(class_data);
        let mut class_file = ClassFile {
            minor_version: 0_u16,
            major_version: 0_u16,
            constant_pool: Rc::new(RefCell::new(ConstantPool::default())),
            access_flags: 0_u16,
            this_class: 0_u16,
            super_class: 0_u16,
            interfaces: vec![0_u16],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        };
        class_file.read(&mut class_reader)?;
        Ok(class_file)
    }

    fn read(&mut self, reader: &mut ClassReader) -> Result<(), String> {
        self.read_and_check_magic(reader)?;
        self.read_and_check_version(reader)?;

        self.constant_pool = read_constant_pool(reader);
        self.access_flags = reader.read_u16();
        self.this_class = reader.read_u16();
        self.super_class = reader.read_u16();
        self.interfaces = reader.read_u16s();
        self.fields = MemberInfo::read(reader, self.constant_pool.clone());
        self.methods = MemberInfo::read(reader, self.constant_pool.clone());
        self.attributes = read_attributes(reader, self.constant_pool.clone());

        Ok(())
    }

    fn read_and_check_magic(&mut self, reader: &mut ClassReader) -> Result<(), String> {
        let magic = reader.read_u32();
        if magic != 0xCAFEBABE {
            return Err("java.lang.ClassFormatError: magic!".to_string());
        }
        Ok(())
    }

    fn read_and_check_version(&mut self, reader: &mut ClassReader) -> Result<(), String> {
        self.minor_version = reader.read_u16();
        self.major_version = reader.read_u16();
        return match self.major_version {
            45 => Ok(()),
            46 | 47 | 48 | 49 | 50 | 51 | 52 => {
                if self.minor_version == 0 {
                    Ok(())
                } else {
                    Err("java.lang.UnsupportedClassVersionError!".to_string())
                }
            }
            _ => {
                Err("java.lang.UnsupportedClassVersionError!".to_string())
            }
        };
    }

    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }

    pub fn major_version(&self) -> u16 {
        self.major_version
    }

    pub fn constant_pool(&self) -> &RcRefCell<ConstantPool> {
        &self.constant_pool
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn fields(&self) -> &Vec<MemberInfo> {
        &self.fields
    }

    pub fn methods(&self) -> &Vec<MemberInfo> {
        &self.methods
    }

    pub fn class_name(&self) -> String {
        self.constant_pool.borrow().get_class_name(self.this_class)
    }

    pub fn super_class_name(&self) -> String {
        if self.super_class > 0 {
            return self.constant_pool.borrow().get_class_name(self.super_class)
        }
        "".to_string()
    }

    pub fn interface_names(&self) -> Vec<String> {
        let mut interface_names = vec![];
        for i in self.interfaces.iter() {
            interface_names.push(self.constant_pool.borrow().get_class_name(*i))
        }
        interface_names.to_vec()
    }
 }

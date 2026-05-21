pub mod attribute_info;
pub mod class_reader;
pub mod constant_pool;
pub mod member_info;

use self::attribute_info::AttributeInfo;
use self::class_reader::ClassReader;
use self::constant_pool::ConstantPool;
use self::member_info::MemberInfo;
use crate::classfile::attribute_info::read_attributes;
use crate::classfile::constant_pool::read_constant_pool;
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
use std::cell::RefCell;
use std::rc::Rc;

pub struct ClassFile {
    /// magic: u32, 魔数（CAFEBABE），已在解析时校验，不保存
    /// 次版本号
    minor_version: u16,
    /// 主版本号
    major_version: u16,
    /// 常量池，使用 Rc<RefCell<_>> 在解析过程中共享给各 cp_info / member / attribute
    constant_pool: RcRefCell<ConstantPool>,
    /// 类访问标志（public/final/super/interface/abstract 等）
    access_flags: u16,
    /// 当前类在常量池中的索引（CONSTANT_Class_info）
    this_class: u16,
    /// 父类在常量池中的索引（CONSTANT_Class_info），Object 类为 0
    super_class: u16,
    /// 接口索引表，每个元素是常量池中 CONSTANT_Class_info 的索引
    interfaces: Vec<u16>,
    /// 字段表
    fields: Vec<MemberInfo>,
    /// 方法表
    methods: Vec<MemberInfo>,
    /// 类级属性表
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl ClassFile {
    /// 解析 class 字节数据
    pub fn parse(class_data: Vec<u8>) -> Result<ClassFile, String> {
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

    /// 按 ClassFile 结构顺序读取并填充各字段
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

    /// 校验 class 文件魔数：必须为 0xCAFEBABE
    fn read_and_check_magic(&mut self, reader: &mut ClassReader) -> Result<(), String> {
        let magic = reader.read_u32();
        if magic != 0xCAFEBABE {
            return Err("java.lang.ClassFormatError: magic!".to_string());
        }
        Ok(())
    }

    /// 校验 class 文件版本：支持 JDK 1.1 ~ 1.8（major 45..=52，minor 必须为 0）
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
            _ => Err("java.lang.UnsupportedClassVersionError!".to_string()),
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

    /// 当前类的全限定名（形如 java/lang/String）
    pub fn class_name(&self) -> String {
        self.constant_pool.borrow().get_class_name(self.this_class)
    }

    /// 父类的全限定名；Object 类（super_class == 0）返回空串
    pub fn super_class_name(&self) -> String {
        if self.super_class > 0 {
            return self.constant_pool.borrow().get_class_name(self.super_class);
        }
        "".to_string()
    }

    /// 所有直接实现的接口名
    pub fn interface_names(&self) -> Vec<String> {
        let mut interface_names = vec![];
        for i in self.interfaces.iter() {
            interface_names.push(self.constant_pool.borrow().get_class_name(*i))
        }
        interface_names.to_vec()
    }
}

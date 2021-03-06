/// field_info {
///     u2             access_flags;
///     u2             name_index;
///     u2             descriptor_index;
///     u2             attributes_count;
///     attribute_info attributes[attributes_count];
/// }
/// method_info {
///     u2             access_flags;
///     u2             name_index;
///     u2             descriptor_index;
///     u2             attributes_count;
///     attribute_info attributes[attributes_count];
/// }

use crate::types::RcRefCell;
use crate::classfile::{
    ConstantPool,
    ClassReader,
    AttributeInfo, read_attributes
};

pub struct MemberInfo {
    constant_pool: RcRefCell<ConstantPool>, /// 保存常量池
    access_flags: u16, /// 成员访问标志
    name_index: u16, /// 成员名称索引
    descriptor_index: u16,
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl MemberInfo {
    pub fn read(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Vec<Self> {
        let member_count = reader.read_u16();
        let mut members = vec![];
        for _i in 0..member_count {
            members.push(MemberInfo::read_member(reader, cp.clone()))
        }
        members
    }
    fn read_member(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Self {
        MemberInfo {
            constant_pool: cp.clone(),
            access_flags: reader.read_u16(),
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
            attributes: read_attributes(reader, cp.clone()),
        }
    }

    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }

    pub fn descriptor(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.descriptor_index)
    }
}

use crate::classfile::{read_attributes, AttributeInfo, ClassReader, ConstantPool};
/// 字段表和方法表在 class 文件中的结构完全一致，统称为成员信息：
///
/// field_info / method_info {
///     u2             access_flags;
///     u2             name_index;
///     u2             descriptor_index;
///     u2             attributes_count;
///     attribute_info attributes[attributes_count];
/// }
use crate::types::RcRefCell;

pub struct MemberInfo {
    /// 保存常量池引用，用于在 name() / descriptor() 中按索引解析字符串
    constant_pool: RcRefCell<ConstantPool>,
    /// 成员访问标志（public/private/static/final 等）
    access_flags: u16,
    /// 成员名称在常量池中的索引（CONSTANT_Utf8_info）
    name_index: u16,
    /// 成员描述符在常量池中的索引（如 "I"、"(II)V"）
    descriptor_index: u16,
    /// 成员级属性表（如 Code、ConstantValue、Exceptions 等）
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl MemberInfo {
    /// 读取成员列表：先读 u2 数量，再依次读取每个成员
    pub fn read(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Vec<Self> {
        let member_count = reader.read_u16();
        let mut members = vec![];
        for _i in 0..member_count {
            members.push(MemberInfo::read_member(reader, cp.clone()))
        }
        members
    }
    /// 读取单个成员
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

    /// 通过常量池获取成员名
    pub fn name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }

    /// 通过常量池获取成员描述符
    pub fn descriptor(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.descriptor_index)
    }
}

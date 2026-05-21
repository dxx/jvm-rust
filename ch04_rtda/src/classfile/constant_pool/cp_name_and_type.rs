/// CONSTANT_NameAndType_info：字段/方法的部分符号引用（名字 + 描述符）
///
/// CONSTANT_NameAndType_info {
///     u1 tag;
///     u2 name_index;
///     u2 descriptor_index;
/// }
use super::{ClassReader, ConstantInfo};

#[derive(Default)]
pub struct ConstantNameAndTypeInfo {
    /// 指向 CONSTANT_Utf8_info：成员名
    name_index: u16,
    /// 指向 CONSTANT_Utf8_info：字段或方法描述符
    descriptor_index: u16,
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
        self.descriptor_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_NAME_AND_TYPE
    }
}

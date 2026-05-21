use super::{ClassReader, ConstantInfo, ConstantPool};
/// CONSTANT_String_info：String 类型字面量
///
/// CONSTANT_String_info {
///     u1 tag;
///     u2 string_index;
/// }
use crate::types::RcRefCell;

pub struct ConstantStringInfo {
    constant_pool: RcRefCell<ConstantPool>,
    /// 指向 CONSTANT_Utf8_info 的索引，存放字符串实际内容
    string_index: u16,
}

impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.string_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_STRING
    }
}

impl ConstantStringInfo {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        ConstantStringInfo {
            constant_pool: cp,
            string_index: 0,
        }
    }
}

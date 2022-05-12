/// CONSTANT_String_info {
///     u1 tag;
///     u2 string_index;
/// }

use crate::types::RcRefCell;
use super::{ConstantInfo, ClassReader, ConstantPool};

pub struct ConstantStringInfo {
    constant_pool: RcRefCell<ConstantPool>,
    string_index: u16
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

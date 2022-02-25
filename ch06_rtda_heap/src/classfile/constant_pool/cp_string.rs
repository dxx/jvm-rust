/// CONSTANT_String_info {
///     u1 tag;
///     u2 string_index;
/// }

use super::{ConstantInfo, ClassReader, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ConstantStringInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
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
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantStringInfo {
            constant_pool: cp,
            string_index: 0,
        }
    }
}

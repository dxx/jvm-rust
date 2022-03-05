/// CONSTANT_Class_info {
///     u1 tag;
///     u2 name_index;
/// }

use super::{ConstantInfo, ClassReader, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct ConstantClassInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    name_index: u16,
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_CLASS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantClassInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantClassInfo {
            constant_pool: cp,
            name_index: 0,
        }
    }

    pub fn name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }
}

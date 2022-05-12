/// EnclosingMethod_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 class_index;
///     u2 method_index;
/// }

use crate::types::RcRefCell;
use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct EnclosingMethodAttribute {
    constant_pool: RcRefCell<ConstantPool>,
    class_index: u16,
    method_index: u16,
}

impl AttributeInfo for EnclosingMethodAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.method_index = reader.read_u16();
    }
}

impl EnclosingMethodAttribute {
    pub fn new (cp: RcRefCell<ConstantPool>) -> Self {
        let mut ema = EnclosingMethodAttribute::default();
        ema.constant_pool = cp;
        ema
    }
}

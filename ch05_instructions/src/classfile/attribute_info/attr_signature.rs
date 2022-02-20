/// Signature_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 signature_index;
/// }

use std::rc::Rc;
use std::cell::RefCell;
use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct SignatureAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    signature_index: u16,
}

impl AttributeInfo for SignatureAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SignatureAttribute {
    pub fn new (cp: Rc<RefCell<ConstantPool>>) -> Self {
        let mut sa = SignatureAttribute::default();
        sa.constant_pool = cp;
        sa
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
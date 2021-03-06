/// Signature_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 signature_index;
/// }

use crate::types::RcRefCell;
use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct SourceFileAttribute {
    constant_pool: RcRefCell<ConstantPool>,
    source_file_index: u16,
}

impl AttributeInfo for SourceFileAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index = reader.read_u16();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SourceFileAttribute {
    pub fn new (cp: RcRefCell<ConstantPool>) -> Self {
        let mut sfa = SourceFileAttribute::default();
        sfa.constant_pool = cp;
        sfa
    }
}

/// ConstantValue_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 constantvalue_index;
/// }

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct ConstantValueAttribute {
    constant_value_index: u16,
}

impl AttributeInfo for ConstantValueAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.constant_value_index = reader.read_u16();
    }
    
    fn name(&self) -> &str {
        return "ConstantValue";
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantValueAttribute {
    pub fn constant_value_index(&self) -> u16 {
        self.constant_value_index
    }
}

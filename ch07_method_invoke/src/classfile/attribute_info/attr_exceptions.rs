/// Exceptions_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 number_of_exceptions;
///     u2 exception_index_table[number_of_exceptions];
/// }

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct ExceptionsAttribute {
    exception_index_table: Vec<u16>,
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.exception_index_table = reader.read_u16s();
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

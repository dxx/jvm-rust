/// Exceptions 属性：方法声明的检查型异常（throws 子句中的异常列表）
///
/// Exceptions_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 number_of_exceptions;
///     u2 exception_index_table[number_of_exceptions];
/// }
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct ExceptionsAttribute {
    /// 每个元素指向常量池中的 CONSTANT_Class_info（异常类）
    exception_index_table: Vec<u16>,
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.exception_index_table = reader.read_u16s();
    }
}

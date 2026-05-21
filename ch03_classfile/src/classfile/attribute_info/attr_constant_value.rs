/// ConstantValue 属性：用于声明字段常量值（如 static final 字段）
///
/// ConstantValue_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 constantvalue_index;
/// }
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct ConstantValueAttribute {
    /// 指向常量池中的 Integer/Float/Long/Double/String 等常量
    constant_value_index: u16,
}

impl AttributeInfo for ConstantValueAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.constant_value_index = reader.read_u16();
    }
}

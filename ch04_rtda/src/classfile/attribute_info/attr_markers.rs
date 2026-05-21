/// Deprecated 属性：标记已过时的类/方法/字段（无数据体，仅作标记）
///
/// Deprecated_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
/// }
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct DeprecatedAttribute {}

impl AttributeInfo for DeprecatedAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}

/// Synthetic 属性：标记编译器生成的类/方法/字段（非源码手写）
///
/// Synthetic_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
/// }

#[derive(Default)]
pub struct SyntheticAttribute {}

impl AttributeInfo for SyntheticAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}

/// 通用标记属性（无数据体）
pub struct MarkerAttribute {}

impl AttributeInfo for MarkerAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {}
}

mod attr_bootstrap_methods;
pub mod attr_code;
mod attr_constant_value;
mod attr_enclosing_method;
mod attr_exceptions;
mod attr_inner_classes;
mod attr_line_number_table;
mod attr_local_variable_table;
mod attr_local_variable_type_table;
mod attr_markers;
mod attr_signature;
mod attr_source_file;
mod attr_unparsed;

use crate::classfile::attribute_info::attr_code::CodeAttribute;
use crate::classfile::attribute_info::attr_constant_value::ConstantValueAttribute;
use crate::classfile::attribute_info::attr_exceptions::ExceptionsAttribute;
use crate::classfile::attribute_info::attr_line_number_table::LineNumberTableAttribute;
use crate::classfile::attribute_info::attr_local_variable_table::LocalVariableTableAttribute;
use crate::classfile::attribute_info::attr_markers::{DeprecatedAttribute, SyntheticAttribute};
use crate::classfile::attribute_info::attr_source_file::SourceFileAttribute;
use crate::classfile::attribute_info::attr_unparsed::UnparsedAttribute;
use crate::classfile::{ClassReader, ConstantPool};
/// attribute_info {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u1 info[attribute_length];
/// }
use crate::types::RcRefCell;

/// 所有 attribute_info 通用接口
///
/// attribute_info {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u1 info[attribute_length];
/// }
pub trait AttributeInfo {
    /// 从字节流读取属性内容（不含 name_index 和 length 字段）
    fn read_info(&mut self, reader: &mut ClassReader);
    
    // 获取名称
    fn name(&self) -> &str {
        return "";
    }

    fn as_any(&self) -> &dyn std::any::Any;
}

/// 读取属性表：先读 u2 数量，再依次读取每个属性
pub fn read_attributes(
    reader: &mut ClassReader,
    cp: RcRefCell<ConstantPool>,
) -> Vec<Box<dyn AttributeInfo>> {
    let attribute_count = reader.read_u16();
    let mut attributes = vec![];
    for _i in 0..attribute_count {
        attributes.push(read_attribute(reader, cp.clone()));
    }
    attributes
}

/// 读取单个属性
fn read_attribute(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Box<dyn AttributeInfo> {
    let attr_name_index = reader.read_u16();
    let attr_name = cp.borrow().get_utf8(attr_name_index);
    let attr_length = reader.read_u32();
    let mut attr_info = new_attribute(&attr_name, attr_length, cp);
    attr_info.read_info(reader);
    attr_info
}

/// 按属性名创建对应类型的 attribute_info；未识别的属性使用 UnparsedAttribute 原样保留字节
fn new_attribute(
    attr_name: &str,
    attr_length: u32,
    cp: RcRefCell<ConstantPool>,
) -> Box<dyn AttributeInfo> {
    match attr_name {
        "Code" => Box::new(CodeAttribute::new(cp)),
        "ConstantValue" => Box::new(ConstantValueAttribute::default()),
        "Deprecated" => Box::new(DeprecatedAttribute::default()),
        "Exceptions" => Box::new(ExceptionsAttribute::default()),
        "LineNumberTable" => Box::new(LineNumberTableAttribute::default()),
        "LocalVariableTable" => Box::new(LocalVariableTableAttribute::default()),
        "SourceFile" => Box::new(SourceFileAttribute::new(cp)),
        "Synthetic" => Box::new(SyntheticAttribute::default()),
        _ => Box::new(UnparsedAttribute::new(
            attr_name.to_string(),
            attr_length,
            None,
        )),
    }
}

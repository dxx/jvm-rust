mod attr_bootstrap_methods;
pub mod attr_code;
pub mod attr_constant_value;
mod attr_enclosing_method;
mod attr_exceptions;
mod attr_inner_classes;
pub mod attr_line_number_table;
mod attr_local_variable_table;
mod attr_local_variable_type_table;
mod attr_markers;
mod attr_signature;
pub mod attr_source_file;
mod attr_unparsed;

/// attribute_info {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u1 info[attribute_length];
/// }

use crate::types::RcRefCell;
use crate::classfile::{ClassReader, ConstantPool};
use crate::classfile::attribute_info::attr_unparsed::UnparsedAttribute;
use crate::classfile::attribute_info::attr_code::CodeAttribute;
use crate::classfile::attribute_info::attr_constant_value::ConstantValueAttribute;
use crate::classfile::attribute_info::attr_markers::{DeprecatedAttribute, SyntheticAttribute};
use crate::classfile::attribute_info::attr_exceptions::ExceptionsAttribute;
use crate::classfile::attribute_info::attr_line_number_table::LineNumberTableAttribute;
use crate::classfile::attribute_info::attr_local_variable_table::LocalVariableTableAttribute;
use crate::classfile::attribute_info::attr_source_file::SourceFileAttribute;

pub trait AttributeInfo {
    fn read_info(&mut self, reader: &mut ClassReader);
    
    // 获取名称
    fn name(&self) -> &str {
        return "";
    }

    fn as_any(&self) -> &dyn std::any::Any;
}

pub fn read_attributes(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Vec<Box<dyn AttributeInfo>> {
    let attribute_count = reader.read_u16();
    let mut attributes = vec![];
    for _i in 0..attribute_count {
        attributes.push(read_attribute(reader, cp.clone()));
    }
    attributes
}

fn read_attribute(reader: &mut ClassReader, cp: RcRefCell<ConstantPool>) -> Box<dyn AttributeInfo> {
    let attr_name_index = reader.read_u16();
    let attr_name = cp.borrow().get_utf8(attr_name_index);
    let attr_length = reader.read_u32();
    let mut attr_info = new_attribute(&attr_name, attr_length, cp);
    attr_info.read_info(reader);
    attr_info
}

fn new_attribute(attr_name: &str, attr_length: u32, cp: RcRefCell<ConstantPool>) -> Box<dyn AttributeInfo> {
    match attr_name {
        "Code" => Box::new(CodeAttribute::new(cp)),
        "ConstantValue" => Box::new(ConstantValueAttribute::default()),
        "Deprecated" => Box::new(DeprecatedAttribute::default()),
        "Exceptions" => Box::new(ExceptionsAttribute::default()),
        "LineNumberTable" => Box::new(LineNumberTableAttribute::default()),
        "LocalVariableTable" => Box::new(LocalVariableTableAttribute::default()),
        "SourceFile" => Box::new(SourceFileAttribute::new(cp)),
        "Synthetic" => Box::new(SyntheticAttribute::default()),
        _ => Box::new(UnparsedAttribute::new(attr_name.to_string(), attr_length, None)),
    }
}

use super::ConstantPool;
use super::{AttributeInfo, ClassReader};
/// SourceFile 属性：记录 class 文件对应的源码文件名
///
/// SourceFile_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 sourcefile_index;
/// }
use crate::types::RcRefCell;

#[derive(Default)]
pub struct SourceFileAttribute {
    constant_pool: RcRefCell<ConstantPool>,
    /// 指向常量池中源码文件名的索引
    source_file_index: u16,
}

impl AttributeInfo for SourceFileAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index = reader.read_u16();
    }
}

impl SourceFileAttribute {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        let mut sfa = SourceFileAttribute::default();
        sfa.constant_pool = cp;
        sfa
    }
}

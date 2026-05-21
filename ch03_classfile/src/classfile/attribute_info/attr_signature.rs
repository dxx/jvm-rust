use super::ConstantPool;
use super::{AttributeInfo, ClassReader};
/// Signature 属性：记录类/方法/字段的泛型签名信息
///
/// Signature_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 signature_index;
/// }
use crate::types::RcRefCell;

#[derive(Default)]
pub struct SignatureAttribute {
    constant_pool: RcRefCell<ConstantPool>,
    /// 指向常量池中泛型签名字符串的索引
    signature_index: u16,
}

impl AttributeInfo for SignatureAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16();
    }
}

impl SignatureAttribute {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        let mut sa = SignatureAttribute::default();
        sa.constant_pool = cp;
        sa
    }
}

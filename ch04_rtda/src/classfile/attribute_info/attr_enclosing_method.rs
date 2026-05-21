use super::ConstantPool;
use super::{AttributeInfo, ClassReader};
/// EnclosingMethod 属性：记录局部类或匿名类的外围方法
///
/// EnclosingMethod_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 class_index;
///     u2 method_index;
/// }
use crate::types::RcRefCell;

#[derive(Default)]
pub struct EnclosingMethodAttribute {
    constant_pool: RcRefCell<ConstantPool>,
    /// 外围类在常量池中的索引
    class_index: u16,
    /// 外围方法在常量池中的索引（0 表示不在方法内）
    method_index: u16,
}

impl AttributeInfo for EnclosingMethodAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.method_index = reader.read_u16();
    }
}

impl EnclosingMethodAttribute {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        let mut ema = EnclosingMethodAttribute::default();
        ema.constant_pool = cp;
        ema
    }
}

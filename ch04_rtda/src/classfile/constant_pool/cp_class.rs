use super::{ClassReader, ConstantInfo, ConstantPool};
/// CONSTANT_Class_info：代表对类或接口的符号引用
///
/// CONSTANT_Class_info {
///     u1 tag;
///     u2 name_index;
/// }
use crate::types::RcRefCell;

#[derive(Clone)]
pub struct ConstantClassInfo {
    /// 保存常量池引用，用于将 name_index 解析为字符串
    constant_pool: RcRefCell<ConstantPool>,
    /// 指向 CONSTANT_Utf8_info 的索引，存放类的全限定名
    name_index: u16,
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_CLASS
    }
}

impl ConstantClassInfo {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        ConstantClassInfo {
            constant_pool: cp,
            name_index: 0,
        }
    }

    /// 通过常量池解析出类名（形如 java/lang/Object）
    pub fn name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }
}

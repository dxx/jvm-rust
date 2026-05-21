/// 兜底属性：对于未识别或暂不支持的属性，将原始字节内容原样保留
///
/// attribute_info {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u1 info[attribute_length];
/// }
use super::{AttributeInfo, ClassReader};

pub struct UnparsedAttribute {
    /// 属性名
    name: String,
    /// 属性内容长度
    length: u32,
    /// 属性原始字节
    info: Option<Vec<u8>>,
}

impl AttributeInfo for UnparsedAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.info = Some(reader.read_bytes(self.length as usize));
    }
}

impl UnparsedAttribute {
    pub fn new(name: String, length: u32, info: Option<Vec<u8>>) -> Self {
        UnparsedAttribute { name, length, info }
    }

    pub fn info(&self) -> &Option<Vec<u8>> {
        &self.info
    }
}

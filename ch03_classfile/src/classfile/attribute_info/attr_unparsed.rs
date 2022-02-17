/// attribute_info {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u1 info[attribute_length];
/// }

use super::{AttributeInfo, ClassReader};

pub struct UnparsedAttribute {
    name: String,
    length: u32,
    info: Option<Vec<u8>>,
}

impl AttributeInfo for UnparsedAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.info = Some(reader.read_bytes(self.length as usize));
    }
}

impl UnparsedAttribute {
    pub fn new(name: String, length: u32, info: Option<Vec<u8>>) -> Self {
        UnparsedAttribute {
            name, length, info
        }
    }

    pub fn info(&self) -> &Option<Vec<u8>> {
        &self.info
    }
}

/// CONSTANT_NameAndType_info {
///     u1 tag;
///     u2 name_index;
///     u2 descriptor_index;
/// }

use super::{ConstantInfo, ClassReader};

#[derive(Default)]
pub struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
        self.descriptor_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_NAME_AND_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantNameAndTypeInfo {
    pub fn name_index(&self) -> u16 {
        self.name_index
    }

    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }
}

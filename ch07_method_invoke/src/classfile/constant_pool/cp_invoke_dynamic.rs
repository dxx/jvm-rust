/// CONSTANT_MethodHandle_info {
///     u1 tag;
///     u1 reference_kind;
///     u2 reference_index;
/// }

use super::{ConstantInfo, ClassReader};

#[derive(Default)]
pub struct ConstantMethodHandleInfo {
    reference_kind: u8,
    reference_index: u16,
}

impl ConstantInfo for ConstantMethodHandleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.reference_kind = reader.read_u8();
        self.reference_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_METHOD_HANDLE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// CONSTANT_MethodType_info {
///     u1 tag;
///     u2 descriptor_index;
/// }

#[derive(Default)]
pub struct ConstantMethodTypeInfo {
    descriptor_index: u16,
}

impl ConstantInfo for ConstantMethodTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.descriptor_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_METHOD_TYPE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// CONSTANT_InvokeDynamic_info {
///     u1 tag;
///     u2 bootstrap_method_attr_index;
///     u2 name_and_type_index;
/// }

#[derive(Default)]
pub struct ConstantInvokeDynamicInfo {
    bootstrap_method_attr_index: u16,
    name_name_type_index: u16,
}

impl ConstantInfo for ConstantInvokeDynamicInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.bootstrap_method_attr_index = reader.read_u16();
        self.name_name_type_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_INVOKE_DYNAMIC
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

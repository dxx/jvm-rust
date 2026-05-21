/// CONSTANT_MethodHandle_info：方法句柄符号引用（Java 7+，用于 invokedynamic）
///
/// CONSTANT_MethodHandle_info {
///     u1 tag;
///     u1 reference_kind;
///     u2 reference_index;
/// }
use super::{ClassReader, ConstantInfo};

#[derive(Default)]
pub struct ConstantMethodHandleInfo {
    /// 方法句柄种类（1~9，对应 getField/getStatic/invokeVirtual 等）
    reference_kind: u8,
    /// 指向常量池中对应成员引用的索引
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
}

/// CONSTANT_MethodType_info：方法类型符号引用
///
/// CONSTANT_MethodType_info {
///     u1 tag;
///     u2 descriptor_index;
/// }

#[derive(Default)]
pub struct ConstantMethodTypeInfo {
    /// 指向 CONSTANT_Utf8_info：方法描述符
    descriptor_index: u16,
}

impl ConstantInfo for ConstantMethodTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.descriptor_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_METHOD_TYPE
    }
}

/// CONSTANT_InvokeDynamic_info：动态调用点符号引用（invokedynamic 使用）
///
/// CONSTANT_InvokeDynamic_info {
///     u1 tag;
///     u2 bootstrap_method_attr_index;
///     u2 name_and_type_index;
/// }

#[derive(Default)]
pub struct ConstantInvokeDynamicInfo {
    /// 指向类属性表中 BootstrapMethods 的条目索引
    bootstrap_method_attr_index: u16,
    /// 指向 CONSTANT_NameAndType_info：调用点的名字和描述符
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
}

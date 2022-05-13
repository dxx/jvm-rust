/// CONSTANT_Fieldref_info {
///     u1 tag;
///     u2 class_index;
///     u2 name_and_type_index;
/// }
/// CONSTANT_Methodref_info {
///     u1 tag;
///     u2 class_index;
///     u2 name_and_type_index;
/// }
/// CONSTANT_InterfaceMethodref_info {
///     u1 tag;
///     u2 class_index;
///     u2 name_and_type_index;
/// }

use crate::types::RcRefCell;
use super::{ConstantInfo, ClassReader, ConstantPool};

pub struct ConstantFieldRefInfo {
    member_info: ConstantMemberRefInfo,
}

impl ConstantInfo for ConstantFieldRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_FIELD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantFieldRefInfo {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        ConstantFieldRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }

    pub fn class_name(&self) -> String {
        self.member_info.constant_pool.borrow().
        get_class_name(self.member_info.class_index)
    }

    pub fn name_and_descriptor(&self) -> (String, String) {
        self.member_info.constant_pool.borrow().
        get_name_and_type(self.member_info.name_and_type_index as usize)
    }
}

pub struct ConstantMethodRefInfo {
    member_info: ConstantMemberRefInfo,
}

impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantMethodRefInfo {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        ConstantMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }

    pub fn class_name(&self) -> String {
        self.member_info.constant_pool.borrow().
        get_class_name(self.member_info.class_index)
    }

    pub fn name_and_descriptor(&self) -> (String, String) {
        self.member_info.constant_pool.borrow().
        get_name_and_type(self.member_info.name_and_type_index as usize)
    }
}

pub struct ConstantInterfaceMethodRefInfo {
    member_info: ConstantMemberRefInfo,
}

impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_INTERFACE_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantInterfaceMethodRefInfo {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        ConstantInterfaceMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }

    pub fn class_name(&self) -> String {
        self.member_info.constant_pool.borrow().
        get_class_name(self.member_info.class_index)
    }

    pub fn name_and_descriptor(&self) -> (String, String) {
        self.member_info.constant_pool.borrow().
        get_name_and_type(self.member_info.name_and_type_index as usize)
    }
}

pub struct ConstantMemberRefInfo {
    constant_pool: RcRefCell<ConstantPool>,
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantMemberRefInfo {
    fn new(cp: RcRefCell<ConstantPool>) -> Self {
        ConstantMemberRefInfo {
            constant_pool: cp,
            class_index: 0,
            name_and_type_index: 0,
        }
    }

    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }
}

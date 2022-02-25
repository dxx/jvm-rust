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

use super::{ConstantInfo, ClassReader, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

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
}

impl ConstantFieldRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantFieldRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
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
}

impl ConstantMethodRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
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
}

impl ConstantInterfaceMethodRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantInterfaceMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }
}

pub struct ConstantMemberRefInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantMemberRefInfo {
    fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
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

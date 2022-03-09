use crate::classfile::member_info::MemberInfo;
use super::access_flags::*;
use super::class::Class;
use std::rc::Rc;
use std::cell::RefCell;

pub fn new_fields(class: Rc<RefCell<Class>>, cf_fields: &Vec<MemberInfo>) -> Vec<Rc<RefCell<Field>>> {
    let mut fields = Vec::new();
    for f in cf_fields {
        let mut field = Field::default();
        field.class = Some(class.clone());
        field.copy_attributes(f);
        fields.push(Rc::new(RefCell::new(field)));
    }
    fields
}

#[derive(Default)]
pub struct Field {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: Option<Rc<RefCell<Class>>>,

    const_value_index: u64,
	slot_id:           u64,
}

impl Field {
    pub fn copy_attributes(&mut self, cf_field: &MemberInfo) {
        self.access_flags = cf_field.access_flags();
        self.name = cf_field.name();
        self.descriptor = cf_field.descriptor();
        match cf_field.constant_value_attribute() {
            Some(val_attr) => {
                self.const_value_index = val_attr.constant_value_index() as u64;
            },
            None => {}
        }
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTECTED != 0
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_volatile(&self) -> bool {
        self.access_flags & ACC_VOLATILE != 0
    }

    pub fn is_transient(&self) -> bool {
        self.access_flags & ACC_TRANSIENT != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }

    pub fn is_long_or_double(&self) -> bool {
        self.descriptor == "J" || self.descriptor == "D"
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn get_class(&self) -> Rc<RefCell<Class>> {
        self.class.clone().unwrap()
    }

    pub fn const_value_index(&self) -> u64 {
        self.const_value_index
    }

    pub fn set_slot_id(&mut self, slot_id: u64) {
        self.slot_id = slot_id;
    }

    pub fn slot_id(&self) -> u64 {
        self.slot_id
    }

    /// jvms 5.4.4
    pub fn is_accessible_to(&self, class: &Rc<RefCell<Class>>) -> bool {
        if self.is_public() {
            return true;
        }
        let c = self.class.as_ref().unwrap();
        if self.is_protected() {
            return class.eq(c) || class.borrow().is_sub_class_of(c) ||
            c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        if !self.is_private() {
            return c.borrow().get_package_name() == class.borrow().get_package_name();
        }
        return class.eq(c);
    }
}

use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::classfile::constant_pool::cp_member_ref::ConstantFieldRefInfo;
use crate::classfile::constant_pool::CONSTANT_FIELD_REF;
use super::class::Class;
use super::field::Field;
use super::constant_pool::Constant;

pub struct FieldRef {
    class_name: String,
    class: OptionalRcRefCell<Class>,
    name: String,
    descriptor: String,
    field: OptionalRcRefCell<Field>,
}

impl FieldRef {
    pub fn new(ref_info: &ConstantFieldRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        FieldRef {
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            field: None,
        }
    }

    pub fn resolved_field(&mut self, class: RcRefCell<Class>) -> RcRefCell<Field> {
        if self.field.is_none() {
            self.resolve_field_ref(class);
        }
        self.field.clone().unwrap()
    }

    /// jvms 5.4.3.2
    fn resolve_field_ref(&mut self, class: RcRefCell<Class>) {
        let c = self.resolved_class(class.clone());
        let field = self.lookup_field(
            &c, self.name.clone(), self.descriptor.clone());

        if field.is_none() {
            panic!("java.lang.NoSuchFieldError");
        }

        if !field.as_ref().unwrap().borrow().is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError");
        }
        
        self.field = field;
    }

    fn lookup_field(
        &mut self,
        class: &RcRefCell<Class>,
        name: String,
        descriptor: String
    ) -> OptionalRcRefCell<Field> {
        for field in class.borrow_mut().fields() {
            if field.borrow().name() == name.clone() && field.borrow().descriptor() == descriptor.clone() {
                return Some(field.clone());
            }
        }

        if class.borrow_mut().interfaces().is_some() {
            for iface in class.borrow_mut().interfaces().unwrap() {
                let field = self.lookup_field(&iface, name.clone(), descriptor.clone());
                if field.is_some() {
                    return field;
                }
            }
        }
        
        if class.borrow().super_class().is_some() {
            return self.lookup_field(&class.borrow().super_class().unwrap(), name, descriptor);
        }

        None
    }

    fn resolved_class(&mut self, class: RcRefCell<Class>) -> RcRefCell<Class> {
        if self.class.is_none() {
            self.resolve_class_ref(class);
        }
        self.class.clone().unwrap()
    }

    /// jvms8 5.4.3.1
    fn resolve_class_ref(&mut self, class: RcRefCell<Class>) {
        let loader = class.borrow_mut().loader().unwrap();
        let c = loader.borrow_mut().load_class(loader.clone(), self.class_name.clone());

        if !c.borrow().is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError");
        }

        self.class = Some(c);
    }
}

impl Constant for FieldRef {
    fn tag(&self) -> u8 {
        CONSTANT_FIELD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

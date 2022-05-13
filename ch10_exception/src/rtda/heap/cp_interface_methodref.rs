use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::classfile::constant_pool::cp_member_ref::ConstantInterfaceMethodRefInfo;
use crate::classfile::constant_pool::CONSTANT_INTERFACE_METHOD_REF;
use super::class::Class;
use super::method::Method;
use super::constant_pool::Constant;
use super::method_lookup::lookup_method_in_interfaces;

pub struct InterfaceMethodRef {
    class_name: String,
    class: OptionalRcRefCell<Class>,
    name: String,
    descriptor: String,
    method: OptionalRcRefCell<Method>,
}

impl InterfaceMethodRef {
    pub fn new(ref_info: &ConstantInterfaceMethodRefInfo) -> Self {
        let (name, descriptor) = ref_info.name_and_descriptor();
        InterfaceMethodRef {
            class_name: ref_info.class_name(),
            class: None,
            name,
            descriptor,
            method: None,
        }
    }

    pub fn resolved_interface_method(&mut self, class: RcRefCell<Class>) -> RcRefCell<Method> {
        if self.method.is_none() {
            self.resolve_interface_method_ref(class);
        }
        self.method.clone().unwrap()
    }

    /// jvms8 5.4.3.4
    fn resolve_interface_method_ref(&mut self, class: RcRefCell<Class>) {
        let c = self.resolved_class(class.clone());
        if !c.borrow().is_interface() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let method = self.lookup_interface_method(
            &c, self.name.clone(), self.descriptor.clone());
        if method.is_none() {
            panic!("java.lang.NoSuchMethodError");
        }

        if !method.as_ref().unwrap().borrow().is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError");
        }

        self.method = method;
    }

    fn lookup_interface_method(
        &mut self,
        iface: &RcRefCell<Class>,
        name: String,
        descriptor: String
    ) -> OptionalRcRefCell<Method> {
        for method in iface.borrow().methods() {
            if method.borrow().name() == name && method.borrow().descriptor() == descriptor {
                return Some(method);
            }
        }
        return lookup_method_in_interfaces(iface.borrow().interfaces().as_ref().unwrap(), name, descriptor);
    }

    pub fn resolved_class(&mut self, class: RcRefCell<Class>) -> RcRefCell<Class> {
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

impl Constant for InterfaceMethodRef {
    fn tag(&self) -> u8 {
        CONSTANT_INTERFACE_METHOD_REF
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

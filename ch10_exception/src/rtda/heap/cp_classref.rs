use crate::classfile::constant_pool::cp_class::ConstantClassInfo;
use crate::classfile::constant_pool::CONSTANT_CLASS;
use super::class::Class;
use super::constant_pool::Constant;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ClassRef {
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
}

impl ClassRef {
    pub fn new(class_info: &ConstantClassInfo) -> Self {
        ClassRef {
            class_name: class_info.name(),
            class: None,
        }
    }

    pub fn resolved_class(&mut self, class: Rc<RefCell<Class>>) -> Rc<RefCell<Class>> {
        if self.class.is_none() {
            self.resolve_class_ref(class);
        }
        self.class.clone().unwrap()
    }

    /// jvms8 5.4.3.1
    fn resolve_class_ref(&mut self, class: Rc<RefCell<Class>>) {
        let loader = class.borrow_mut().loader().unwrap();
        let c = loader.borrow_mut().load_class(loader.clone(), self.class_name.clone());

        if !c.borrow().is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError");
        }

        self.class = Some(c);
    }
}

impl Constant for ClassRef {
    fn tag(&self) -> u8 {
        CONSTANT_CLASS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

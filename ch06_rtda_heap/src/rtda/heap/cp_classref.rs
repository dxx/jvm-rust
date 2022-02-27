use crate::classfile::constant_pool::cp_class::ConstantClassInfo;
use crate::classfile::constant_pool::CONSTANT_CLASS;
use super::class::Class;
use super::constant_pool::{Constant, ConstantPool};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ClassRef {
    cp: Rc<RefCell<ConstantPool>>,
    class_name: String,
    class: Option<Rc<RefCell<Class>>>,
}

impl ClassRef {
    pub fn new(cp: Rc<RefCell<ConstantPool>>, class_info: &ConstantClassInfo) -> Self {
        ClassRef {
            cp,
            class_name: class_info.name(),
            class: None,
        }
    }

    pub fn resolved_class(&mut self) -> Rc<RefCell<Class>> {
        if self.class.is_none() {
            self.resolve_class_ref();
        }
        self.class.clone().unwrap()
    }

    /// jvms8 5.4.3.1
    fn resolve_class_ref(&mut self) {
        let cp = self.cp.borrow();
        let class = cp.class().borrow();
        let loader = class.loader().unwrap();
        let c = loader.borrow_mut().load_class(loader, self.class_name.clone());
        if !c.borrow().is_accessible_to(cp.class()) {
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
}

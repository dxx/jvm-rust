use super::class::Class;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ConstantPool {
    class: Rc<RefCell<Class>>,
}

impl ConstantPool {
    pub fn new(class: Rc<RefCell<Class>>) -> Self {
        ConstantPool {
            class,
        }
    }
}

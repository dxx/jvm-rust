use crate::classfile::ClassFile;
use crate::rtda::local_vars::Slot;
use super::access_flags::ACC_ABSTRACT;
use super::access_flags::ACC_ANNOTATION;
use super::access_flags::ACC_ENUM;
use super::access_flags::ACC_FINAL;
use super::access_flags::ACC_INTERFACE;
use super::access_flags::ACC_SUPER;
use super::access_flags::ACC_SYNTHETIC;
use super::class_loader::ClassLoader;
use super::constant_pool::ConstantPool;
use super::field::Field;
use super::field::new_fields;
use super::method::Method;
use super::method::new_methods;
use super::access_flags::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Class {
    access_flags: u16,
    name: String,
    super_classname: String,
    interface_names: Vec<String>,
    constant_pool: Option<Rc<RefCell<ConstantPool>>>,
    fields: Option<Vec<Rc<Field>>>,
    methods: Option<Vec<Rc<Method>>>,
    loader: Option<Rc<ClassLoader>>,
    super_class: Option<Rc<Class>>,
    interfaces: Option<Vec<Rc<Class>>>,
    instance_slot_count: u64,
    static_slot_count: u64,
    static_vars: Option<Vec<Slot>>,
}

impl Class {
    pub fn new(cf: &ClassFile) -> Rc<RefCell<Self>> {
        let class = Class {
            access_flags: cf.access_flags(),
            name: cf.class_name(),
            super_classname: cf.super_class_name(),
            interface_names: cf.interface_names(),
            constant_pool: None,
            fields: None,
            methods: None,
            loader: None,
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
        };
        let rc_class = Rc::new(RefCell::new(class));
        rc_class.borrow_mut().constant_pool = Some(
            Rc::new(RefCell::new(ConstantPool::new(rc_class.clone()))));
        rc_class.borrow_mut().fields = Some(new_fields(rc_class.clone(), cf.fields()));
        rc_class.borrow_mut().methods = Some(new_methods(rc_class.clone(), cf.methods()));
        rc_class
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags & ACC_SUPER != 0
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & ACC_INTERFACE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_annotation(&self) -> bool {
        self.access_flags & ACC_ANNOTATION != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }
    
    pub fn get_constant_pool(&self) -> Option<&Rc<RefCell<ConstantPool>>> {
        self.constant_pool.as_ref()
    }

    pub fn get_static_vars(&self) -> Option<&Vec<Slot>> {
        self.static_vars.as_ref()
    }

    pub fn get_main_method(&self) -> Option<&Rc<Method>> {
        self.get_static_method("main".into(), "([Ljava/lang/String;)V".into())
    }

    fn get_static_method(&self, name: String, descriptor: String) -> Option<&Rc<Method>> {
        for method in self.methods.as_ref().unwrap() {
            if method.is_static() && method.name() == name && method.descriptor() == descriptor {
                return Some(method)
            }
        }
        None
    }

    fn get_package_name(&self) -> String {
        match self.name.rfind("/") {
            Some(i) => {
                self.name.as_str()[..i].into()
            },
            None => "".into()
        }
    }

    fn is_accessible_to(&self, other: Rc<RefCell<Class>>) -> bool {
        self.is_public() || self.get_package_name() == other.borrow().get_package_name()
    }
}

use crate::classfile::ClassFile;
use crate::rtda::object::Object;
use crate::rtda::heap::slots::Slots;
use super::access_flags::ACC_ABSTRACT;
use super::access_flags::ACC_ANNOTATION;
use super::access_flags::ACC_ENUM;
use super::access_flags::ACC_FINAL;
use super::access_flags::ACC_INTERFACE;
use super::access_flags::ACC_PUBLIC;
use super::access_flags::ACC_SUPER;
use super::access_flags::ACC_SYNTHETIC;
use super::class_loader::ClassLoader;
use super::constant_pool::ConstantPool;
use super::string_pool::StringPool;
use super::class_name_helper::get_array_class_name;
use super::field::Field;
use super::field::new_fields;
use super::method::Method;
use super::method::new_methods;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Class {
    access_flags: u16,
    name: String,
    super_classname: String,
    interface_names: Vec<String>,
    constant_pool: Option<Rc<RefCell<ConstantPool>>>,
    fields: Option<Vec<Rc<RefCell<Field>>>>,
    methods: Option<Vec<Rc<RefCell<Method>>>>,
    loader: Option<Rc<RefCell<ClassLoader>>>,
    super_class: Option<Rc<RefCell<Class>>>,
    interfaces: Option<Vec<Rc<RefCell<Class>>>>,
    instance_slot_count: u64,
    static_slot_count: u64,
    static_vars: Option<Rc<RefCell<Slots>>>,

    init_started: bool,
    string_pool: Rc<RefCell<StringPool>>,
}

impl Class {
    pub fn new(cf: &ClassFile, string_pool: Rc<RefCell<StringPool>>) -> Rc<RefCell<Self>> {
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
            init_started: false,
            string_pool,
        };
        let rc_class = Rc::new(RefCell::new(class));
        rc_class.borrow_mut().constant_pool = Some(ConstantPool::new(rc_class.clone(), cf.constant_pool()));
        rc_class.borrow_mut().fields = Some(new_fields(rc_class.clone(), cf.fields()));
        rc_class.borrow_mut().methods = Some(new_methods(rc_class.clone(), cf.methods()));
        rc_class
    }

    pub fn new_array_class(name: String, string_pool: Rc<RefCell<StringPool>>) -> Rc<RefCell<Self>> {
        let class = Class {
            access_flags: ACC_PUBLIC,
            name,
            super_classname: "java/lang/Object".into(),
            interface_names: vec!["java/lang/Cloneable".into(), "java/io/Serializable".into()],
            constant_pool: None,
            fields: Some(vec![]),
            methods: Some(vec![]),
            loader: None,
            super_class: None,
            interfaces: None,
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: None,
            init_started: true,
            string_pool,
        };
        Rc::new(RefCell::new(class))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn fields(&self) -> Vec<Rc<RefCell<Field>>> {
        self.fields.clone().unwrap()
    }

    pub fn methods(&self) -> Vec<Rc<RefCell<Method>>> {
        self.methods.clone().unwrap()
    }

    pub fn super_classname(&self) -> String {
        self.super_classname.clone()
    }

    pub fn interface_names(&self) -> Vec<String> {
        self.interface_names.clone()
    }

    pub fn set_loader(&mut self, loader: Option<Rc<RefCell<ClassLoader>>>) {
        self.loader = loader;
    }

    pub fn loader(&self) -> Option<Rc<RefCell<ClassLoader>>> {
        self.loader.clone()
    }

    pub fn constant_pool(&self) -> Rc<RefCell<ConstantPool>> {
        self.constant_pool.clone().unwrap()
    }

    pub fn set_super_class(&mut self, super_class: Option<Rc<RefCell<Class>>>) {
        self.super_class = super_class;
    }

    pub fn super_class(&self) -> Option<Rc<RefCell<Class>>> {
        self.super_class.clone()
    }

    pub fn set_interfaces(&mut self, interfaces: Option<Vec<Rc<RefCell<Class>>>>) {
        self.interfaces = interfaces;
    }

    pub fn interfaces(&self) -> Option<Vec<Rc<RefCell<Class>>>> {
        self.interfaces.clone()
    }

    pub fn set_instance_slot_count(&mut self, count: u64) {
        self.instance_slot_count = count;
    }

    pub fn instance_slot_count(&self) -> u64 {
        self.instance_slot_count
    }

    pub fn set_static_slot_count(&mut self, count: u64) {
        self.static_slot_count = count;
    }

    pub fn static_slot_count(&self) -> u64 {
        self.static_slot_count
    }

    pub fn set_static_vars(&mut self, static_vars: Option<Rc<RefCell<Slots>>>) {
        self.static_vars = static_vars;
    }

    pub fn static_vars(&self) -> Rc<RefCell<Slots>> {
        self.static_vars.clone().unwrap()
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

    pub fn get_main_method(&self) -> Option<Rc<RefCell<Method>>> {
        self.get_static_method("main".into(), "([Ljava/lang/String;)V".into())
    }

    pub fn get_clinit_method(&self) -> Option<Rc<RefCell<Method>>> {
        self.get_static_method("<clinit>".into(), "()V".into())
    }

    pub fn get_package_name(&self) -> String {
        match self.name.rfind("/") {
            Some(i) => {
                self.name.as_str()[..i].into()
            },
            None => "".into()
        }
    }

    fn get_static_method(&self, name: String, descriptor: String) -> Option<Rc<RefCell<Method>>> {
        for method in self.methods.as_ref().unwrap() {
            let b_method = method.borrow();
            if b_method.is_static() && b_method.name() == name && b_method.descriptor() == descriptor {
                return Some(method.clone())
            }
        }
        None
    }

    /// jvms 5.4.4
    pub fn is_accessible_to(&self, other: &Rc<RefCell<Class>>) -> bool {
        self.is_public() || self.get_package_name() == other.borrow().get_package_name()
    }

    /// jvms8 6.5.instanceof
    /// jvms8 6.5.checkcast
    pub fn is_assignable_from(
        &self,
        _self: &Rc<RefCell<Class>>,
        other: &Rc<RefCell<Class>>) -> bool {
        if _self.eq(other) {
            return true;
        }

        if !other.borrow().is_array() {
            if !other.borrow().is_interface() { // other is class
                if !_self.borrow().is_interface() { // _self is not interface
                    return other.borrow().is_sub_class_of(_self);
                } else { // _self is interface
                    return other.borrow().is_implements(_self);
                }
            } else { // other is interface
                if !_self.borrow().is_interface() { // _self is not interface
                    return _self.borrow().is_jl_object();
                } else { // _self is interface
                    return other.borrow().is_sub_interface_of(_self)
                }
            }
        } else { // other is array
            if !_self.borrow().is_array() {
                if !_self.borrow().is_interface() { // _self is class
                    return _self.borrow().is_jl_object();
                } else { // _self is interface
                    return _self.borrow().is_jl_cloneable() || _self.borrow().is_jio_serializable();
                }
            } else { // _self is array
                let oc = other.borrow_mut().component_class();
                let sc = _self.borrow_mut().component_class();
                return oc.eq(&sc) || sc.borrow().is_assignable_from(&sc, &oc);
            }
        }
    }

    /// self extends other
    pub fn is_sub_class_of(&self, other: &Rc<RefCell<Class>>) -> bool {
        let mut c = self.super_class();
        while let Some(class) = c {
            if class.eq(other) {
                return true;
            }
            c = class.borrow().super_class();
        }
        false
    }

    /// self implements other
    pub fn is_implements(&self, other: &Rc<RefCell<Class>>) -> bool {
        for i in self.interfaces().unwrap() {
            if i.eq(other) || i.borrow().is_sub_interface_of(other) {
                return true;
            }
        }

        let mut c = self.super_class();
        while let Some(class) = c {
            for i in class.borrow().interfaces().unwrap() {
                if i.eq(other) || i.borrow().is_sub_interface_of(other) {
                    return true;
                }
            }

            c = class.borrow().super_class();
        }
        return false;
    }

    /// self extends other
    pub fn is_sub_interface_of(&self, other: &Rc<RefCell<Class>>) -> bool {
        for super_interface in self.interfaces().unwrap() {
            if super_interface.eq(other) || super_interface.borrow().is_sub_interface_of(other) {
                return true;
            }
        }
        return false;
    }

    pub fn start_init(&mut self) {
        self.init_started = true;
    }

    pub fn init_started(&self) -> bool {
        self.init_started
    }

    pub fn new_object(&self, class: Rc<RefCell<Class>>) -> Object {
        Object::new(class)
    }

    pub fn array_class(&mut self) -> Rc<RefCell<Class>> {
        let array_class_name = get_array_class_name(self.name.clone());
        let loader = self.loader.as_mut().unwrap();
        return loader.borrow_mut().load_class(loader.clone(), array_class_name);
    }

    pub fn get_field(&self, name: String, descriptor: String, is_static: bool) -> Option<Rc<RefCell<Field>>> {
        for field in self.fields.as_ref().unwrap() {
            if field.borrow().is_static() == is_static &&
                field.borrow().name() == name &&
                field.borrow().descriptor() == descriptor {
                    return Some(field.clone());
            }
        }

        let mut c = self.super_class();
        while let Some(class) = c {
            for field in class.borrow().fields() {
                if field.borrow().is_static() == is_static &&
                    field.borrow().name() == name &&
                    field.borrow().descriptor() == descriptor {
                        return Some(field);
                    }
            }

            c = class.borrow().super_class();
        }
        return None;
    }

    pub fn string_pool(&self) -> Rc<RefCell<StringPool>> {
        self.string_pool.clone()
    }

    fn is_jl_object(&self) -> bool {
        self.name == "java/lang/Object"
    }

    fn is_jl_cloneable(&self) -> bool {
        self.name == "java/lang/Cloneable"
    }

    fn is_jio_serializable(&self) -> bool {
        self.name == "java/io/Serializable"
    }
    
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self == _other
    }

    fn ne(&self, other: &Self) -> bool {
        let _self = self as *const Self;
        let _other = other as *const Self;
        _self != _other
    }
}

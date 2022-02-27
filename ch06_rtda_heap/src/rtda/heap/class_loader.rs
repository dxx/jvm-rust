use crate::classpath::Classpath;
use crate::classfile::ClassFile;
use crate::classpath::entry::Entry;
use super::class::Class;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct ClassLoader {
    classpath: Rc<Classpath>,
    // 保存加载的类，key 为类的完全限定名
    class_map: HashMap<String, Rc<RefCell<Class>>>,
}

impl ClassLoader {
    pub fn new(classpath: Rc<Classpath>) -> Self {
        ClassLoader {
            classpath,
            class_map: HashMap::new(),
        }
    }

    pub fn load_class(&mut self, _self: &Rc<RefCell<Self>>, name: String) -> Rc<RefCell<Class>> {
        match self.class_map.get(&name) {
            Some(class) => { // already loaded
                class.clone()
            },
            None => {
                self.load_non_array_class(_self, name)
            }
        }
    }

    fn load_non_array_class(&mut self, _self: &Rc<RefCell<Self>>, name: String) -> Rc<RefCell<Class>> {
        let data = self.read_class(name.clone());
        let class = self.define_class(_self, data);
        link(&class);
        println!("[Loaded {}", name);
        class
    }

    fn read_class(&mut self, name: String) -> Vec<u8> {
        match self.classpath.read_class(&name) {
            Ok(data) => {
                data
            },
            Err(_) => {
                panic!("java.lang.ClassNotFoundException: {}", name)
            }
        }
    }

    fn define_class(&mut self, _self: &Rc<RefCell<Self>>, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class = ClassLoader::parse_class(data);
        class.borrow_mut().set_loader(Some(_self.clone()));
        ClassLoader::resolve_super_class(&class);
        ClassLoader::resolve_interfaces(&class);
        self.class_map.insert(class.borrow().name(), class.clone());
        class
    }

    fn parse_class(data: Vec<u8>) -> Rc<RefCell<Class>> {
        let cf_result = ClassFile::parse(data);
        match cf_result {
            Ok(cf) => {
                Class::new(&cf)
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    fn resolve_super_class(class: &Rc<RefCell<Class>>) {
        let b_class = class.borrow();
        if b_class.name() != "java/lang/Object" {
            let loader = b_class.loader().unwrap();
            let super_class = Some(loader.borrow_mut().load_class(
                &loader, b_class.super_classname()));
            class.borrow_mut().set_super_class(super_class);
        }
    }

    fn resolve_interfaces(class: &Rc<RefCell<Class>>) {
        let b_class = class.borrow();
        let interface_names = b_class.interface_names();
        if interface_names.len() <= 0 {
            return;
        }
        let loader = b_class.loader().unwrap();
        let mut interfaces: Vec<Rc<RefCell<Class>>> = Vec::new();
        for name in interface_names {
            interfaces.push(loader.borrow_mut().load_class(&loader, name));
        }
        class.borrow_mut().set_interfaces(Some(interfaces));
    }
}

fn link(class: &Rc<RefCell<Class>>) {
    verify(class);
    prepare(class);
}

fn verify(class: &Rc<RefCell<Class>>) {
    // TODO
}

fn prepare(class: &Rc<RefCell<Class>>) {

}

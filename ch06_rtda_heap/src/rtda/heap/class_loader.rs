use crate::classpath::Classpath;
use crate::classfile::ClassFile;
use crate::classpath::entry::Entry;
use crate::rtda::heap::slots::Slots;
use super::class::Class;
use super::field::Field;
use super::constant_pool::ConstantPool;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

/// class names:
///     - primitive types: boolean, byte, int ...
///     - primitive arrays: [Z, [B, [I ...
///     - non-array classes: java/lang/Object ...
///     - array classes: [Ljava/lang/Object; ...
pub struct ClassLoader {
    classpath: Classpath,
    // 保存加载的类，key 为类的完全限定名
    class_map: HashMap<String, Rc<RefCell<Class>>>,
}

impl ClassLoader {
    pub fn new(classpath: Classpath) -> Self {
        ClassLoader {
            classpath,
            class_map: HashMap::new(),
        }
    }

    pub fn load_class(&mut self, name: String) -> Rc<RefCell<Class>> {
        match self.class_map.get(&name) {
            Some(class) => { // Already loaded
                class.clone()
            },
            None => {
                self.load_non_array_class(name)
            }
        }
    }

    pub fn finish_load_class(&mut self, _self: Rc<RefCell<Self>>) {
        for (name, class) in self.class_map.iter_mut() {
            let mut mut_class = class.borrow_mut();
            match mut_class.loader() {
                Some(c) => {},
                None => {
                    mut_class.set_loader(Some(_self.clone()));
                }
            }
        }
    }

    fn load_non_array_class(&mut self, name: String) -> Rc<RefCell<Class>> {
        let data = self.read_class(name.clone());
        let class = self.define_class(data);
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

    /// jvms 5.3.5
    fn define_class(&mut self, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class = ClassLoader::parse_class(data);

        // 类加载完成后再 Set class loader
        // class.borrow_mut().set_loader(Some(_self));

        self.resolve_super_class(&class);
        self.resolve_interfaces(&class);
        
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

    /// jvms 5.4.3.1
    fn resolve_super_class(&mut self, class: &Rc<RefCell<Class>>) {
        let mut b_class = class.borrow_mut();
        if b_class.name() != "java/lang/Object" {
            let super_class = Some(self.load_class(b_class.super_classname()));
            b_class.set_super_class(super_class);
        }
    }

    fn resolve_interfaces(&mut self, class: &Rc<RefCell<Class>>) {
        let b_class = class.borrow();
        let interface_names = b_class.interface_names();
        if interface_names.len() <= 0 {
            return;
        }
        let mut interfaces: Vec<Rc<RefCell<Class>>> = Vec::new();
        for name in interface_names {
            interfaces.push(self.load_class(name));
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

/// jvms 5.4.2
fn prepare(class: &Rc<RefCell<Class>>) {
    calc_instance_field_slot_ids(class);
    let static_slot_count = calc_static_field_slot_ids(class);
    alloc_and_init_static_vars(class, static_slot_count);
}

fn calc_instance_field_slot_ids(class: &Rc<RefCell<Class>>) {
    let mut slot_id = 0;
    let mut b_mut_class = class.borrow_mut();
    if b_mut_class.super_class().is_some() {
        slot_id = b_mut_class.super_class().unwrap().borrow().instance_slot_count();
    }
    for field in b_mut_class.fields() {
        let mut field = field.borrow_mut();
        if !field.is_static() {
            field.set_slot_id(slot_id);
            slot_id += 1;
            if field.is_long_or_double() {
                slot_id += 1;
            }
        }
    }
    b_mut_class.set_instance_slot_count(slot_id);
}

fn calc_static_field_slot_ids(class: &Rc<RefCell<Class>>) -> usize {
    let mut slot_id = 0;
    let mut b_mut_class = class.borrow_mut();
    for field in b_mut_class.fields() {
        let mut field = field.borrow_mut();
        if field.is_static() {
            field.set_slot_id(slot_id);
            slot_id += 1;
            if field.is_long_or_double() {
                slot_id += 1;
            }
        }
    }
    b_mut_class.set_static_slot_count(slot_id);
    b_mut_class.static_slot_count() as usize
}

fn alloc_and_init_static_vars(class: &Rc<RefCell<Class>>, static_slot_count: usize) {
    let mut b_mut_class = class.borrow_mut();
    let mut vars = Some(Slots::new(static_slot_count));
    
    //let b_class = class.borrow();
    let cp = b_mut_class.constant_pool();
    //let vars = b_mut_class.static_vars_mut().unwrap();
    let fields = b_mut_class.fields();
    for field in fields {
        let field = field.borrow();
        if field.is_static() && field.is_final() {
            init_static_final_var(&cp, vars.as_mut().unwrap(), &field);
        }
    }
    b_mut_class.set_static_vars(vars);
}

fn init_static_final_var(cp: &Rc<RefCell<ConstantPool>>, vars: &mut Slots, field: &Field) {
    //let mut b_mut_class = class.borrow_mut();
    //let vars = b_mut_class.static_vars_mut().unwrap();
    //let b_class = class.borrow();
    //let b_cp = b_class.constant_pool().unwrap().borrow();
    let b_cp = cp.borrow();
    let cp_index = field.const_value_index();
    let slot_id = field.slot_id();
    if cp_index > 0 {
        let descriptor = field.descriptor();
        if descriptor == "Z" || descriptor == "B" ||
        descriptor == "C" || descriptor == "S" ||descriptor == "I" {
            let val = b_cp.get_constant(cp_index as usize)
            .as_any().downcast_ref::<i32>().unwrap();
            vars.set_int(slot_id as usize, *val);
        } else if descriptor == "J" {
            let val = b_cp.get_constant(cp_index as usize)
            .as_any().downcast_ref::<i64>().unwrap();
            vars.set_long(slot_id as usize, *val);
        } else if descriptor == "F" {
            let val = b_cp.get_constant(cp_index as usize)
            .as_any().downcast_ref::<f32>().unwrap();
            vars.set_float(slot_id as usize, *val);
        } else if descriptor == "D" {
            let val = b_cp.get_constant(cp_index as usize)
            .as_any().downcast_ref::<f64>().unwrap();
            vars.set_double(slot_id as usize, *val);
        } else if descriptor == "Ljava/lang/String;" {
            panic!("Todo");
        }
    }
}

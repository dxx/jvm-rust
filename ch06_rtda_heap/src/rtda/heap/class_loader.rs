use crate::classpath::Classpath;
use crate::classfile::ClassFile;
use crate::classpath::entry::Entry;
use crate::rtda::heap::slots::Slots;
use super::class::Class;
use super::field::Field;
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

    pub fn load_class(&mut self, _self: Rc<RefCell<Self>>, name: String) -> Rc<RefCell<Class>> {
        match self.class_map.get(&name) {
            Some(class) => { // Already loaded
                class.clone()
            },
            None => {
                self.load_non_array_class(&_self, name)
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

    /// jvms 5.3.5
    fn define_class(&mut self, _self: &Rc<RefCell<Self>>, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class = ClassLoader::parse_class(data);

        class.borrow_mut().set_loader(Some(_self.clone()));

        self.resolve_super_class(_self, &class);
        self.resolve_interfaces(_self, &class);
        
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
    fn resolve_super_class(&mut self, _self: &Rc<RefCell<Self>>, class: &Rc<RefCell<Class>>) {
        if class.borrow_mut().name() != "java/lang/Object" {
            let super_class = Some(
                self.load_class(_self.clone(), class.borrow_mut().super_classname()));
            class.borrow_mut().set_super_class(super_class);
        }
    }

    fn resolve_interfaces(&mut self, _self: &Rc<RefCell<Self>>, class: &Rc<RefCell<Class>>) {
        let interface_names = class.borrow_mut().interface_names();
        let mut interfaces: Vec<Rc<RefCell<Class>>> = Vec::new();
        for name in interface_names {
            interfaces.push(self.load_class(_self.clone(), name));
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
    if class.borrow_mut().super_class().is_some() {
        slot_id = class.borrow_mut().super_class().unwrap().borrow().instance_slot_count();
    }
    for field in class.borrow_mut().fields() {
        if !field.borrow_mut().is_static() {
            field.borrow_mut().set_slot_id(slot_id);
            slot_id += 1;
            if field.borrow_mut().is_long_or_double() {
                slot_id += 1;
            }
        }
    }
    class.borrow_mut().set_instance_slot_count(slot_id);
}

fn calc_static_field_slot_ids(class: &Rc<RefCell<Class>>) -> usize {
    let mut slot_id = 0;
    for field in class.borrow_mut().fields() {
        if field.borrow_mut().is_static() {
            field.borrow_mut().set_slot_id(slot_id);
            slot_id += 1;
            if field.borrow_mut().is_long_or_double() {
                slot_id += 1;
            }
        }
    }
    class.borrow_mut().set_static_slot_count(slot_id);
    slot_id as usize
}

fn alloc_and_init_static_vars(class: &Rc<RefCell<Class>>, static_slot_count: usize) {
    let vars = Some(Rc::new(RefCell::new(Slots::new(static_slot_count))));
    let cp = class.borrow_mut().constant_pool();
    let fields = class.borrow_mut().fields();
    for field in fields {
        if field.borrow().is_static() && field.borrow().is_final() {
            init_static_final_var(class, vars.as_ref().unwrap(), &field);
        }
    }
    class.borrow_mut().set_static_vars(vars);
}

fn init_static_final_var(class: &Rc<RefCell<Class>>, vars: &Rc<RefCell<Slots>>, field: &Rc<RefCell<Field>>) {
    let cp = class.borrow_mut().constant_pool();
    let cp_index = field.borrow().const_value_index();
    let slot_id = field.borrow().slot_id();
    if cp_index > 0 {
        let descriptor = field.borrow().descriptor();
        if descriptor == "Z" || descriptor == "B" ||
        descriptor == "C" || descriptor == "S" ||descriptor == "I" {
            let val = *cp.borrow().get_constant(cp_index as usize)
            .as_any().downcast_ref::<i32>().unwrap();
            vars.borrow_mut().set_int(slot_id as usize, val);
        } else if descriptor == "J" {
            let val = *cp.borrow().get_constant(cp_index as usize)
            .as_any().downcast_ref::<i64>().unwrap();
            vars.borrow_mut().set_long(slot_id as usize, val);
        } else if descriptor == "F" {
            let val = *cp.borrow().get_constant(cp_index as usize)
            .as_any().downcast_ref::<f32>().unwrap();
            vars.borrow_mut().set_float(slot_id as usize, val);
        } else if descriptor == "D" {
            let val = *cp.borrow().get_constant(cp_index as usize)
            .as_any().downcast_ref::<f64>().unwrap();
            vars.borrow_mut().set_double(slot_id as usize, val);
        } else if descriptor == "Ljava/lang/String;" {
            panic!("Todo");
        }
    }
}

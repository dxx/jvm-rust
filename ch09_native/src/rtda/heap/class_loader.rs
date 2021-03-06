use crate::types::RcRefCell;
use crate::classpath::Classpath;
use crate::classfile::ClassFile;
use crate::classpath::entry::Entry;
use crate::rtda::slots::Slots;
use crate::rtda::ClassData;
use super::class::Class;
use super::field::Field;
use super::string_pool::StringPool;
use super::class_name_helper::PRIMITIVE_TYPES;
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
    class_map: HashMap<String, RcRefCell<Class>>,
    verbose_flag: bool,
    string_pool: RcRefCell<StringPool>,
}

impl ClassLoader {
    pub fn new(classpath: Classpath, string_pool: RcRefCell<StringPool>, verbose_flag: bool) -> RcRefCell<Self> {
        let class_loader = ClassLoader {
            classpath,
            class_map: HashMap::new(),
            verbose_flag,
            string_pool,
        };
        let class_loader = Rc::new(RefCell::new(class_loader));
        class_loader.borrow_mut().load_basic_class(&class_loader);
        class_loader.borrow_mut().load_primitive_classes(&class_loader);
        class_loader
    }

    fn load_basic_class(&mut self, _self: &RcRefCell<Self>) {
        let j_class = self.load_class(_self.clone(), "java/lang/Class".into());
        let j_class_mut = unsafe { j_class.as_ptr().as_mut().unwrap() };
        for (name, class) in self.class_map.iter() {
            if class.borrow_mut().j_class().is_none() {
                let mut obj = j_class_mut.new_object(j_class.clone());
                obj.set_extra(Some(Box::new(ClassData::new(class.clone()))));

                class.borrow_mut().set_j_class(Some(Rc::new(RefCell::new(obj))));
            }
        }
    }

    fn load_primitive_classes(&mut self, _self: &RcRefCell<Self>) {
        for (primitive_class, _) in PRIMITIVE_TYPES.iter() {
            self.load_primitive_class(_self, (*primitive_class).into());
        }
    }

    fn load_primitive_class(&mut self, _self: &RcRefCell<Self>, class_name: String) {
        let class_map = &mut self.class_map;
        let j_class = class_map.get_mut("java/lang/Class").unwrap();
        let j_class_mut = unsafe { j_class.as_ptr().as_mut().unwrap() };

        let mut obj = j_class_mut.new_object(j_class.clone());

        let class = Class::new_primitive_class(class_name, self.string_pool.clone());
        obj.set_extra(Some(Box::new(ClassData::new(class.clone()))));

        class.borrow_mut().set_loader(Some(_self.clone()));
        class.borrow_mut().set_j_class(Some(Rc::new(RefCell::new(obj))));

        class_map.insert(class.borrow_mut().name(), class.clone());
    }

    pub fn load_class(&mut self, _self: RcRefCell<Self>, name: String) -> RcRefCell<Class> {
        match self.class_map.get(&name) {
            Some(class) => { // Already loaded
                class.clone()
            },
            None => {
                let mut class;
                if name.as_bytes()[0] == b'[' {
                    class = Some(self.load_array_class(&_self, name));
                } else {
                    class = Some(self.load_non_array_class(&_self, name));
                }

                let j_class = self.class_map.get("java/lang/Class");
                if j_class.is_some() {
                    let j_class = j_class.unwrap();
                    let j_class_mut = unsafe { j_class.as_ptr().as_mut().unwrap() };
                    let mut obj = j_class_mut.new_object(j_class.clone());
                    obj.set_extra(Some(Box::new(ClassData::new(class.clone().unwrap()))));

                    class.as_mut().unwrap().borrow_mut().set_j_class(Some(Rc::new(RefCell::new(obj))));
                }

                class.clone().unwrap()
            }
        }
    }

    fn load_array_class(&mut self, _self: &RcRefCell<Self>, name: String) -> RcRefCell<Class> {
        let array_class = Class::new_array_class(name, self.string_pool.clone());

        array_class.borrow_mut().set_loader(Some(_self.clone()));

        self.resolve_super_class(_self, &array_class);
        self.resolve_interfaces(_self, &array_class);

        self.class_map.insert(array_class.borrow().name(), array_class.clone());
        array_class
    }

    fn load_non_array_class(&mut self, _self: &RcRefCell<Self>, name: String) -> RcRefCell<Class> {
        let data = self.read_class(name.clone());
        let class = self.define_class(_self, data);
        link(&class);
        if self.verbose_flag {
            println!("[Loaded {}", name);
        }
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
    fn define_class(&mut self, _self: &RcRefCell<Self>, data: Vec<u8>) -> RcRefCell<Class> {
        let class = ClassLoader::parse_class(data, self.string_pool.clone());

        class.borrow_mut().set_loader(Some(_self.clone()));

        self.resolve_super_class(_self, &class);
        self.resolve_interfaces(_self, &class);
        
        self.class_map.insert(class.borrow().name(), class.clone());
        class
    }

    fn parse_class(data: Vec<u8>, string_pool: RcRefCell<StringPool>) -> RcRefCell<Class> {
        let cf_result = ClassFile::parse(data);
        match cf_result {
            Ok(cf) => {
                Class::new(&cf, string_pool)
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// jvms 5.4.3.1
    fn resolve_super_class(&mut self, _self: &RcRefCell<Self>, class: &RcRefCell<Class>) {
        if class.borrow_mut().name() != "java/lang/Object" {
            let super_class = Some(
                self.load_class(_self.clone(), class.borrow_mut().super_classname()));
            class.borrow_mut().set_super_class(super_class);
        }
    }

    fn resolve_interfaces(&mut self, _self: &RcRefCell<Self>, class: &RcRefCell<Class>) {
        let interface_names = class.borrow_mut().interface_names();
        let mut interfaces: Vec<RcRefCell<Class>> = Vec::new();
        for name in interface_names {
            interfaces.push(self.load_class(_self.clone(), name));
        }
        class.borrow_mut().set_interfaces(Some(interfaces));
    }
}

fn link(class: &RcRefCell<Class>) {
    verify(class);
    prepare(class);
}

fn verify(class: &RcRefCell<Class>) {
    // TODO
}

/// jvms 5.4.2
fn prepare(class: &RcRefCell<Class>) {
    calc_instance_field_slot_ids(class);
    calc_static_field_slot_ids(class);
    alloc_and_init_static_vars(class);
}

fn calc_instance_field_slot_ids(class: &RcRefCell<Class>) {
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

fn calc_static_field_slot_ids(class: &RcRefCell<Class>) {
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
}

fn alloc_and_init_static_vars(class: &RcRefCell<Class>) {
    let static_slot_count = class.borrow_mut().static_slot_count() as usize;
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

fn init_static_final_var(class: &RcRefCell<Class>, vars: &RcRefCell<Slots>, field: &RcRefCell<Field>) {
    let string_pool = class.borrow_mut().string_pool();
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
            let val = &*cp.borrow().get_constant(cp_index as usize)
            .as_any().downcast_ref::<String>().unwrap().clone();
            let interned_str = string_pool.borrow_mut().jstring(
                    class.borrow().loader().unwrap(), val.into());
            vars.borrow_mut().set_ref(slot_id as usize, Some(interned_str));
        }
    }
}

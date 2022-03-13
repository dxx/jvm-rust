use crate::rtda::Object;
use super::class_loader::ClassLoader;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct StringPool {
    pool: HashMap<String, Rc<RefCell<Object>>>,
}

impl StringPool {
    pub fn new() -> Self {
        StringPool {
            pool: HashMap::new(),
        }
    }

    pub fn jstring(&mut self, loader: Rc<RefCell<ClassLoader>>, rstr: String) -> Rc<RefCell<Object>> {
        let interned_str = self.pool.get(rstr.as_str());
        if interned_str.is_some() {
            return interned_str.unwrap().clone();
        }

        let loader_mut = unsafe { loader.as_ptr().as_mut().unwrap() };

        let chars = string_to_utf16(rstr.clone());
        let j_chars = Object::new_data(
            loader_mut.load_class(loader.clone(), "[C".into()),
            Box::new(chars));
        
        let class = loader_mut.load_class(loader.clone(), "java/lang/String".into());
        let j_str = Rc::new(RefCell::new(class.borrow().new_object(class.clone())));
        j_str.borrow_mut().set_ref_var("value".into(), "[C".into(), Rc::new(RefCell::new(j_chars)));
        
        self.pool.insert(rstr, j_str.clone());

        j_str
    }

    pub fn is_exist(&self, rstr: String) -> bool {
        let interned_str = self.pool.get(rstr.as_str());
        if interned_str.is_some() {
            return true;
        }
        false
    }

    pub fn add(&mut self, rstr: String, jstring: Rc<RefCell<Object>>) {
        self.pool.insert(rstr, jstring);
    }
}

/// java.lang.String -> rust String
pub fn rust_string(obj: &Rc<RefCell<Object>>) -> String {
    let char_arr = obj.borrow_mut().get_ref_var("value".into(), "[C".into());
    let chars = char_arr.borrow_mut().chars_mut().clone();
    utf16_to_string(chars)
}

/// utf-8 -> utf16
pub fn string_to_utf16(s: String) -> Vec<u16> {
    s.encode_utf16().collect::<Vec<u16>>()
}

/// utf16 -> utf-8
pub fn utf16_to_string(s: Vec<u16>) -> String {
    String::from_utf16(&s).unwrap()
}

/// java.lang.String -> rust String
pub fn intern_string(obj: &Rc<RefCell<Object>>) -> Rc<RefCell<Object>> {
    let rust_string = rust_string(obj);
    let class = obj.borrow().class().clone();
    let loader = class.borrow().loader().unwrap();
    let constant_pool = class.borrow().constant_pool();
    if constant_pool.borrow_mut().string_pool_mut().is_exist(rust_string.clone()) {
        return constant_pool.borrow_mut().string_pool_mut().jstring(loader.clone(), rust_string);
    }
    constant_pool.borrow_mut().string_pool_mut().add(rust_string, obj.clone());

    obj.clone()
}

#[path = "./native/java/lang/Class.rs" ]
pub mod class;
#[path = "./native/java/lang/Object.rs" ]
pub mod object;
#[path = "./native/java/lang/Double.rs" ]
pub mod double;
#[path = "./native/java/lang/Float.rs" ]
pub mod float;
#[path = "./native/java/lang/String.rs" ]
pub mod string;
#[path = "./native/java/lang/System.rs" ]
pub mod system;
#[path = "./native/java/lang/Throwable.rs" ]
pub mod throwable;

use crate::rtda::Frame;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref REGISTRY: RwLock<HashMap<String, NativeMethod>> = RwLock::new(HashMap::new());
    static ref REGISTRY_INIT: HashMap<String, fn ()> = {
        let mut hashmap: HashMap<String, fn ()> = HashMap::new();

        hashmap.insert("java/lang/Class".into(), class::init);
        hashmap.insert("java/lang/Object".into(), object::init);
        hashmap.insert("java/lang/Double".into(), double::init);
        hashmap.insert("java/lang/Float".into(), float::init);
        hashmap.insert("java/lang/String".into(), string::init);
        hashmap.insert("java/lang/System".into(), system::init);
        hashmap.insert("java/lang/Throwable".into(), throwable::init);

        hashmap
    };
}

type NativeMethod = fn (frame: &mut Frame);

const EMPTY_NATIVE_METHOD: NativeMethod = |frame: &mut Frame| {
    // Do nothing
};

fn registry_key(
    class_name: String,
    method_name: String,
    method_descriptor: String,
) -> String {
    format!("{}~{}~{}", class_name, method_name, method_descriptor)
}

pub fn registry(
    class_name: String,
    method_name: String,
    method_descriptor: String,
    method: NativeMethod
) {
    let key = registry_key(class_name, method_name, method_descriptor);
    REGISTRY.write().unwrap().insert(key, method);
}

pub fn find_native_method(
    class_name: String,
    method_name: String,
    method_descriptor: String,
) -> Option<NativeMethod> {
    if method_name == "registerNatives" && method_descriptor == "()V" {
        return Some(EMPTY_NATIVE_METHOD);
    }

    let key = registry_key(class_name.clone(), method_name.clone(), method_descriptor.clone());

    let registry = REGISTRY.read().unwrap();
    if !registry.contains_key(&key) {
        drop(registry); // 释放读锁

        let init = REGISTRY_INIT.get(&class_name).unwrap();
        init(); // 最后会调用 registry 函数上写锁
    }
    
    let registry = REGISTRY.read().unwrap();
    let method = registry.get(&key);
    if method.is_some() {
        return Some(*method.unwrap());
    }
    
    None
}

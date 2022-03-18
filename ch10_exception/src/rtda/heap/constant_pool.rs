use crate::classfile::constant_pool;
use crate::classfile::constant_pool::cp_class;
use crate::classfile::constant_pool::cp_numeric;
use crate::classfile::constant_pool::cp_member_ref;
use crate::classfile::constant_pool::cp_string;
use super::class::Class;
use super::cp_classref::ClassRef;
use super::cp_fieldref::FieldRef;
use super::cp_methodref::MethodRef;
use super::cp_interface_methodref::InterfaceMethodRef;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Constant {
    fn tag(&self) -> u8;

    fn as_any(&self) -> &dyn std::any::Any;
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl Constant for i32 {
    fn tag(&self) -> u8 {
        constant_pool::CONSTANT_INTEGER
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for f32 {
    fn tag(&self) -> u8 {
        constant_pool::CONSTANT_FLOAT
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for i64 {
    fn tag(&self) -> u8 {
        constant_pool::CONSTANT_LONG
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for f64 {
    fn tag(&self) -> u8 {
        constant_pool::CONSTANT_DOUBLE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Constant for String {
    fn tag(&self) -> u8 {
        constant_pool::CONSTANT_STRING
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// 运行时常量池
pub struct ConstantPool {
    class: Rc<RefCell<Class>>,
    consts: Vec<Option<Box<dyn Constant>>>,
}

impl ConstantPool {
    /// 将常量池转化成运行时常量池
    pub fn new(class: Rc<RefCell<Class>>, cf_cp: &Rc<RefCell<constant_pool::ConstantPool>>) -> Rc<RefCell<Self>> {
        let b_cf_cp = cf_cp.borrow();
        let len = b_cf_cp.constant_len();
        let consts: Vec<Option<Box<dyn Constant>>> = Vec::new();
        let rt_cp = Rc::new(RefCell::new(ConstantPool {
            class,
            consts,
        }));
        rt_cp.borrow_mut().consts.push(None);

        let mut i = 1;
        loop {
            if i == len {
                break;
            }
            let cp_info = b_cf_cp.get_constant_info(i).as_ref().unwrap();
            match cp_info.tag() {
                constant_pool::CONSTANT_INTEGER => {
                    let int_info = cp_info.as_any()
                        .downcast_ref::<cp_numeric::ConstantIntegerInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(int_info.value())));
                },
                constant_pool::CONSTANT_FLOAT => {
                    let float_info = cp_info.as_any()
                        .downcast_ref::<cp_numeric::ConstantFloatInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(float_info.value())));
                },
                constant_pool::CONSTANT_LONG => {
                    let long_info = cp_info.as_any()
                        .downcast_ref::<cp_numeric::ConstantLongInfo>().unwrap();
                    // 占两个位置
                    rt_cp.borrow_mut().consts.push(Some(Box::new(long_info.value())));
                    rt_cp.borrow_mut().consts.push(None);
                    i += 1;
                },
                constant_pool::CONSTANT_DOUBLE => {
                    let double_info = cp_info.as_any()
                        .downcast_ref::<cp_numeric::ConstantDoubleInfo>().unwrap();
                    // 占两个位置
                    rt_cp.borrow_mut().consts.push(Some(Box::new(double_info.value())));
                    rt_cp.borrow_mut().consts.push(None);
                    i += 1;
                },
                constant_pool::CONSTANT_STRING  => {
                    let string_info = cp_info.as_any()
                        .downcast_ref::<cp_string::ConstantStringInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(string_info.to_string())));
                },
                constant_pool::CONSTANT_CLASS  => {
                    let class_info = cp_info.as_any()
                        .downcast_ref::<cp_class::ConstantClassInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(ClassRef::new(class_info))));
                },
                constant_pool::CONSTANT_FIELD_REF  => {
                    let field_ref_info = cp_info.as_any()
                        .downcast_ref::<cp_member_ref::ConstantFieldRefInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(FieldRef::new(field_ref_info))));
                },
                constant_pool::CONSTANT_METHOD_REF  => {
                    let method_ref_info = cp_info.as_any()
                        .downcast_ref::<cp_member_ref::ConstantMethodRefInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(MethodRef::new(method_ref_info))));
                },
                constant_pool::CONSTANT_INTERFACE_METHOD_REF  => {
                    let interface_method_ref_info = cp_info.as_any()
                        .downcast_ref::<cp_member_ref::ConstantInterfaceMethodRefInfo>().unwrap();
                    rt_cp.borrow_mut().consts.push(Some(Box::new(InterfaceMethodRef::new(interface_method_ref_info))));
                },
                _ => {
                    rt_cp.borrow_mut().consts.push(None);
                }
            }
            i += 1
        }
        rt_cp
    }

    pub fn class(&self) -> Rc<RefCell<Class>> {
        self.class.clone()
    }

    pub fn get_constant(&self, index: usize) -> &Box<dyn Constant> {
        match self.consts.get(index) {
            Some(_const) => {
                _const.as_ref().unwrap()
            },
            None => {
                panic!("No constants at index {}", index);
            }
        }
    }

    pub fn get_constant_mut(&mut self, index: usize) -> &mut Box<dyn Constant> {
        match self.consts.get_mut(index) {
            Some(_const) => {
                _const.as_mut().unwrap()
            },
            None => {
                panic!("No constants at index {}", index);
            }
        }
    }

}

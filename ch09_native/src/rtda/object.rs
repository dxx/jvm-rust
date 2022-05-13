use crate::types::RcRefCell;
use super::slots::Slots;
use super::heap::class::Class;

pub const SLOTS: u8 = 10;

pub trait ObjectData {
    fn tag(&self) -> u8;

    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait ObjectExtra {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct Object {
    class: RcRefCell<Class>,
    data: Box<dyn ObjectData>, // Slots for Object, []int32 for int[] ...
    extra: Option<Box<dyn ObjectExtra>>,
}

impl PartialEq for Object {
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

impl Object {
    /// Create normal (non-array) object
    pub fn new(class: RcRefCell<Class>) -> Self {
        Object::new_data(
            class.clone(),
            Box::new(Slots::new(class.borrow().instance_slot_count() as usize)))
    }

    pub fn new_data(class: RcRefCell<Class>, data: Box<dyn ObjectData>) -> Self {
        Object {
            class,
            data,
            extra: None,
        }
    }

    pub fn class(&self) -> &RcRefCell<Class> {
        &self.class
    }

    pub fn fields(&self) -> &Slots {
        self.data.as_any().downcast_ref::<Slots>().as_ref().unwrap()
    }

    pub fn fields_mut(&mut self) -> &mut Slots {
        self.data.as_any_mut().downcast_mut::<Slots>().unwrap()
    }

    pub fn is_instance_of(&self, class: &RcRefCell<Class>) -> bool {
        class.borrow().is_assignable_from(class, &self.class)
    }

    pub fn data(&self) -> &Box<dyn ObjectData> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Box<dyn ObjectData> {
        &mut self.data
    }

    pub fn extra(&self) -> Option<&Box<dyn ObjectExtra>> {
        self.extra.as_ref()
    }

    pub fn set_extra(&mut self, extra: Option<Box<dyn ObjectExtra>>) {
        self.extra = extra;
    }

    /// Reflection
    pub fn get_ref_var(&mut self, name: String, descriptor: String) -> RcRefCell<Object> {
        let field = self.class.borrow().get_field(name, descriptor, false);
        let slots = self.data.as_any_mut().downcast_mut::<Slots>().unwrap();
        slots.get_ref(field.unwrap().borrow().slot_id() as usize).unwrap()
    }

    pub fn set_ref_var(&mut self, name: String, descriptor: String, _ref: RcRefCell<Object>) {
        let field = self.class.borrow().get_field(name, descriptor, false);
        let slots = self.data.as_any_mut().downcast_mut::<Slots>().unwrap();
        slots.set_ref(field.unwrap().borrow().slot_id() as usize, Some(_ref));
    }
}


pub struct ClassData {
    class: RcRefCell<Class>,
}

impl ClassData {
    pub fn new(class: RcRefCell<Class>) -> Self {
        ClassData {
            class
        }
    }

    pub fn java_name(&self) -> String {
        self.class.borrow().name().replace("/", ".")
    }
}

impl ObjectExtra for ClassData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

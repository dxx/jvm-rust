use crate::types::{
    RcRefCell,
    OptionalRcRefCell,
};
use crate::rtda::{ObjectData, Object};

pub const BYTES: u8    = 1;
pub const SHORTS: u8   = 2;
pub const INTS: u8     = 3;
pub const LONGS: u8    = 4;
pub const CHARS: u8    = 5;
pub const FLOATS: u8   = 6;
pub const DOUBLES: u8  = 7;
pub const REFS: u8     = 8;

/// Byte array
impl ObjectData for Vec<i8> {
    fn tag(&self) -> u8 {
        BYTES
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Short array
impl ObjectData for Vec<i16> {
    fn tag(&self) -> u8 {
        SHORTS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Int array
impl ObjectData for Vec<i32> {
    fn tag(&self) -> u8 {
        INTS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Long array
impl ObjectData for Vec<i64> {
    fn tag(&self) -> u8 {
        LONGS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Char array
impl ObjectData for Vec<u16> {
    fn tag(&self) -> u8 {
        CHARS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Float array
impl ObjectData for Vec<f32> {
    fn tag(&self) -> u8 {
        FLOATS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Double array
impl ObjectData for Vec<f64> {
    fn tag(&self) -> u8 {
        DOUBLES
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Ref array
impl ObjectData for Vec<OptionalRcRefCell<Object>> {
    fn tag(&self) -> u8 {
        REFS
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self    
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Object {
    pub fn bytes_mut(&mut self) -> &mut Vec<i8> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<i8>>().unwrap()
    }

    pub fn shorts_mut(&mut self) -> &mut Vec<i16> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<i16>>().unwrap()
    }

    pub fn ints_mut(&mut self) -> &mut Vec<i32> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<i32>>().unwrap()
    }

    pub fn longs_mut(&mut self) -> &mut Vec<i64> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<i64>>().unwrap()
    }

    pub fn chars_mut(&mut self) -> &mut Vec<u16> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<u16>>().unwrap()
    }

    pub fn floats_mut(&mut self) -> &mut Vec<f32> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<f32>>().unwrap()
    }

    pub fn doubles_mut(&mut self) -> &mut Vec<f64> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<f64>>().unwrap()
    }

    pub fn refs_mut(&mut self) -> &mut Vec<OptionalRcRefCell<Object>> {
        self.data_mut().as_any_mut().downcast_mut::<Vec<OptionalRcRefCell<Object>>>().unwrap()
    }

    pub fn array_length(&self) -> usize {
        match self.data().tag() {
            BYTES => {
                self.data().as_any().downcast_ref::<Vec<i8>>().unwrap().len()
            },
            SHORTS => {
                self.data().as_any().downcast_ref::<Vec<i16>>().unwrap().len()
            },
            INTS => {
                self.data().as_any().downcast_ref::<Vec<i32>>().unwrap().len()
            },
            LONGS => {
                self.data().as_any().downcast_ref::<Vec<i64>>().unwrap().len()
            },
            CHARS => {
                self.data().as_any().downcast_ref::<Vec<u16>>().unwrap().len()
            },
            FLOATS => {
                self.data().as_any().downcast_ref::<Vec<f32>>().unwrap().len()
            },
            DOUBLES => {
                self.data().as_any().downcast_ref::<Vec<f64>>().unwrap().len()
            },
            REFS => {
                self.data().as_any().downcast_ref::<Vec<OptionalRcRefCell<Object>>>().unwrap().len()
            },
            _ => {
                panic!("Not array!");
            }
        }
    }
}

pub fn array_copy(
    src: RcRefCell<Object>,
    dest: RcRefCell<Object>,
    src_pos: usize,
    dest_pos: usize,
    length: usize,
) {
    let mut src = src.borrow_mut();
    let mut dest = dest.borrow_mut();
    match src.data().tag() {
        BYTES => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<i8>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<i8>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        SHORTS => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<i16>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<i16>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        INTS => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<i32>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<i32>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        LONGS => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<i64>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<i64>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        CHARS => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<u16>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<u16>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        FLOATS => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<f32>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<f32>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        DOUBLES => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<f64>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<f64>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            _dest.copy_from_slice(_src);
        },
        REFS => {
            let _src = src.data_mut().as_any_mut().downcast_mut::<Vec<OptionalRcRefCell<Object>>>().unwrap();
            let _dest = dest.data_mut().as_any_mut().downcast_mut::<Vec<OptionalRcRefCell<Object>>>().unwrap();
            let _src = &_src[src_pos..src_pos + length];
            let _dest = &mut _dest[dest_pos..dest_pos + length];
            //_dest.copy_from_slice(&_src);

            // 引用复制
            for i in 0.._dest.len() {
                if i >= _src.len() {
                    break;
                }
                _dest[i] = _src[i].clone();
            }
        },
        _ => {
            panic!("Not array!");
        }
    }
}

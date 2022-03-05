/// CONSTANT_Integer_info {
///     u1 tag;
///     u4 bytes;
/// }

use super::{ConstantInfo, ClassReader};

#[derive(Default)]
pub struct ConstantIntegerInfo {
    val: i32,
}

impl ConstantInfo for ConstantIntegerInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = reader.read_u32() as i32
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_INTEGER
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantIntegerInfo {
    pub fn value(&self) -> i32 {
        self.val
    }
}

/// CONSTANT_Float_info {
///     u1 tag;
///     u4 bytes;
/// }

#[derive(Default)]
pub struct ConstantFloatInfo {
    val: f32,
}

impl ConstantInfo for ConstantFloatInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = f32::from_bits(reader.read_u32());
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_FLOAT
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantFloatInfo {
    pub fn value(&self) -> f32 {
        self.val
    }
}

/// CONSTANT_Long_info {
///     u1 tag;
///     u4 high_bytes;
///     u4 low_bytes;
/// }

#[derive(Default)]
pub struct ConstantLongInfo {
    val: i64,
}

impl ConstantInfo for ConstantLongInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = reader.read_u64() as i64;
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_LONG
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantLongInfo {
    pub fn value(&self) -> i64 {
        self.val
    }
}

/// CONSTANT_Double_info {
///     u1 tag;
///     u4 high_bytes;
///     u4 low_bytes;
/// }

#[derive(Default)]
pub struct ConstantDoubleInfo {
    val: f64,
}

impl ConstantInfo for ConstantDoubleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = f64::from_bits(reader.read_u64());
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_DOUBLE
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ConstantDoubleInfo {
    pub fn value(&self) -> f64 {
        self.val
    }
}

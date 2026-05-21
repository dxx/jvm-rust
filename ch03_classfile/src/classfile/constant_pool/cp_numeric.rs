/// CONSTANT_Integer_info：int 字面量（4 字节）
///
/// CONSTANT_Integer_info {
///     u1 tag;
///     u4 bytes;
/// }
use super::{ClassReader, ConstantInfo};

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
}

impl ConstantIntegerInfo {
    pub fn value(&self) -> i32 {
        self.val
    }
}

/// CONSTANT_Float_info：float 字面量（IEEE 754 单精度）
///
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
}

impl ConstantFloatInfo {
    pub fn value(&self) -> f32 {
        self.val
    }
}

/// CONSTANT_Long_info：long 字面量（8 字节，占常量池两个位置）
///
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
}

impl ConstantLongInfo {
    pub fn value(&self) -> i64 {
        self.val
    }
}

/// CONSTANT_Double_info：double 字面量（IEEE 754 双精度，占常量池两个位置）
///
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
}

impl ConstantDoubleInfo {
    pub fn value(&self) -> f64 {
        self.val
    }
}

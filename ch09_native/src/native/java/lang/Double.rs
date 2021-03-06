use crate::rtda::Frame;
use super::registry;

const J_DOUBLE: &str = "java/lang/Double";

pub fn init() {
    registry(J_DOUBLE.into(), "doubleToRawLongBits".into(), "(D)J".into(), double_to_raw_long_bits);
    registry(J_DOUBLE.into(), "longBitsToDouble".into(), "(J)D".into(), long_bits_to_double);
}

/// public static native long doubleToRawLongBits(double value);
/// (D)J
fn double_to_raw_long_bits(frame: &mut Frame) {
    let value = frame.local_vars_mut().get_double(0);
    let bits = f64::to_bits(value);
    frame.operand_stack_mut().push_long(bits as i64);
}

/// public static native double longBitsToDouble(long bits);
/// (J)D
fn long_bits_to_double(frame: &mut Frame) {
    let bits = frame.local_vars_mut().get_long(0);
    let value = f64::from_bits(bits as u64);
    frame.operand_stack_mut().push_double(value);
}

use crate::rtda::Frame;
use super::registry;

const J_FLOAT: &str = "java/lang/Float";

pub fn init() {
    registry(J_FLOAT.into(), "floatToRawIntBits".into(), "(F)I".into(), float_to_raw_int_bits);
    registry(J_FLOAT.into(), "intBitsToFloat".into(), "(I)F".into(), int_bits_to_float);
}

/// public static native int floatToRawIntBits(float value);
/// (F)I
fn float_to_raw_int_bits(frame: &mut Frame) {
    let value = frame.local_vars_mut().get_float(0);
    let bits = f32::to_bits(value);
    frame.operand_stack_mut().push_int(bits as i32);
}

/// public static native float intBitsToFloat(int bits);
/// (I)F
fn int_bits_to_float(frame: &mut Frame) {
    let bits = frame.local_vars_mut().get_int(0);
    let value = f32::from_bits(bits as u32);
    frame.operand_stack_mut().push_float(value);
}

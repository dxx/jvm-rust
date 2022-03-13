#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::native;
use super::super::instruction::Instruction;
use super::super::instruction::Result;

/// Invoke native method
#[derive(Default, Debug)]
pub struct INVOKE_NATIVE;

impl Instruction for INVOKE_NATIVE {
    fn execute(&mut self, frame: &mut Frame) -> Result<String> {
        let method = frame.get_method();
        let class_name = method.borrow().get_class().borrow().name();
        let method_name = method.borrow().name();
        let method_descriptor = method.borrow().descriptor();

        let native_method = native::find_native_method(
            class_name.clone(), method_name.clone(), method_descriptor.clone()
        );
        if native_method.is_none() {
            let method_info = format!("{}.{}{}", class_name, method_name, method_descriptor);
            return Err(format!("java.lang.UnsatisfiedLinkError: {}", method_info));
        }
        native_method.unwrap()(frame);

        Ok(())
    }
}

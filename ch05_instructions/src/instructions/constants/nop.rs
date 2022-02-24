use super::super::instruction::Instruction;
use super::super::super::rtda::Frame;

#[derive(Default, Debug)]
pub struct NOP;

impl Instruction for NOP {
    fn execute(&mut self, frame: &mut Frame) {
        // Really do nothing
    }
}

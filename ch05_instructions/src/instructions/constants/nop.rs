use super::super::super::rtda::Frame;
use super::super::instruction::Instruction;

#[derive(Default, Debug)]
pub struct NOP;

impl Instruction for NOP {
    fn execute(&mut self, frame: &mut Frame) {
        // Really do nothing
    }
}

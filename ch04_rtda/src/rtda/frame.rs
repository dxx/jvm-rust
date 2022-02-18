use super::{local_vars::LocalVars, operand_stack::OperandStack};

/// Stack Frame
pub struct Frame {
    pub lower: Option<Box<Frame>>, // Stack is implemented as linked list
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals: usize, max_size: usize) -> Self {
        Frame {
            lower: None,
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_size),
        }
    }

    pub fn set_lower(&mut self, lower: Option<Box<Frame>>) {
        self.lower = lower;
    }

    pub fn get_local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn get_operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }
}

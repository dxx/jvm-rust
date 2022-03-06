use super::{local_vars::LocalVars, operand_stack::OperandStack};

/// Stack Frame
pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_size),
        }
    }

    pub fn get_local_vars(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn get_operand_stack(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }
}

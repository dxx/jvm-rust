use super::{local_vars::LocalVars, operand_stack::OperandStack};

/// 栈帧（Stack Frame）：每个方法调用对应一个栈帧
/// 包含该方法的局部变量表和操作数栈
pub struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame {
    /// 创建栈帧
    /// - max_locals: 局部变量表的容量（slot 数）
    /// - max_size:   操作数栈的最大深度（slot 数）
    pub fn new(max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_size),
        }
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }
}

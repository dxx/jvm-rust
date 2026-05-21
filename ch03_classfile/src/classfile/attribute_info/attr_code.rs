use super::ConstantPool;
use super::{AttributeInfo, ClassReader};
/// Code 属性：方法的字节码与异常表，是方法体的真正实现载体
///
/// Code_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 max_stack;
///     u2 max_locals;
///     u4 code_length;
///     u1 code[code_length];
///     u2 exception_table_length;
///     {   u2 start_pc;
///         u2 end_pc;
///         u2 handler_pc;
///         u2 catch_type;
///     } exception_table[exception_table_length];
///     u2 attributes_count;
///     attribute_info attributes[attributes_count];
/// }
use crate::types::RcRefCell;

#[derive(Default)]
pub struct CodeAttribute {
    constant_pool: RcRefCell<ConstantPool>,
    /// 方法运行时操作数栈的最大深度
    max_stack: u16,
    /// 方法运行时局部变量表的最大长度
    max_locals: u16,
    /// 方法的字节码指令序列
    code: Vec<u8>,
    /// 异常处理表（try-catch 表）
    exception_table: Vec<ExceptionTableEntry>,
    /// 内嵌属性，如 LineNumberTable、LocalVariableTable 等
    attributes: Vec<Box<dyn AttributeInfo>>,
}

impl AttributeInfo for CodeAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.max_stack = reader.read_u16();
        self.max_locals = reader.read_u16();
        let code_length = reader.read_u32() as usize;
        self.code = reader.read_bytes(code_length);
        self.exception_table = read_exception_table(reader);
        self.attributes = super::read_attributes(reader, self.constant_pool.clone())
    }
}

impl CodeAttribute {
    pub fn new(cp: RcRefCell<ConstantPool>) -> Self {
        let mut ca = CodeAttribute::default();
        ca.constant_pool = cp;
        ca
    }
}

/// 异常表条目：[start_pc, end_pc) 范围内若抛出 catch_type 异常则跳转到 handler_pc
pub struct ExceptionTableEntry {
    /// try 块起始字节码偏移（包含）
    start_pc: u16,
    /// try 块结束字节码偏移（不包含）
    end_pc: u16,
    /// 异常处理器起始字节码偏移
    handler_pc: u16,
    /// 捕获的异常类在常量池中的索引；为 0 表示捕获所有异常（finally）
    catch_type: u16,
}

fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
    let exception_length = reader.read_u16();
    let mut exception_table = vec![];
    for _i in 0..exception_length {
        exception_table.push(ExceptionTableEntry {
            start_pc: reader.read_u16(),
            end_pc: reader.read_u16(),
            handler_pc: reader.read_u16(),
            catch_type: reader.read_u16(),
        });
    }
    exception_table
}

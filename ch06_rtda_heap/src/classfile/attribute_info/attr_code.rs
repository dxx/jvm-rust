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

use std::rc::Rc;
use std::cell::RefCell;
use super::ConstantPool;
use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct CodeAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
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

    fn name(&self) -> &str {
        return "Code";
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CodeAttribute {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        let mut ca = CodeAttribute::default();
        ca.constant_pool = cp;
        ca
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }
}

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
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

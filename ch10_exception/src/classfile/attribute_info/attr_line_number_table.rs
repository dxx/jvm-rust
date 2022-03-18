/// LineNumberTable_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 line_number_table_length;
///     {   u2 start_pc;
///         u2 line_number;
///     } line_number_table[line_number_table_length];
/// }

use super::{AttributeInfo, ClassReader};

#[derive(Default, Clone)]
pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Clone)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl AttributeInfo for LineNumberTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let line_number_table_length = reader.read_u16();
        let mut line_number_table = vec![];
        for _i in 0..line_number_table_length {
            line_number_table.push(LineNumberTableEntry {
                start_pc: reader.read_u16(),
                line_number: reader.read_u16(),
            });
        }
        self.line_number_table = line_number_table;
    }

    fn name(&self) -> &str {
        return "LineNumberTable";
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl LineNumberTableAttribute {
    pub fn get_line_number(&self, pc: i64) -> i64 {
        for i in (0..self.line_number_table.len()).rev() {
            let entry = self.line_number_table.get(i);
            if pc >= entry.as_ref().unwrap().start_pc as i64 {
                return entry.unwrap().line_number as i64;
            }
        }

        -1
    }
}

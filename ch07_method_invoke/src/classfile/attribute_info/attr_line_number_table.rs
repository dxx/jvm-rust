/// LineNumberTable_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 line_number_table_length;
///     {   u2 start_pc;
///         u2 line_number;
///     } line_number_table[line_number_table_length];
/// }

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>,
}

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
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

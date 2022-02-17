/// LocalVariableTable_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 local_variable_table_length;
///     {   u2 start_pc;
///         u2 length;
///         u2 name_index;
///         u2 descriptor_index;
///         u2 index;
///     } local_variable_table[local_variable_table_length];
/// }

use super::{AttributeInfo, ClassReader};

#[derive(Default)]
pub struct LocalVariableTableAttribute {
    local_variable_table: Vec<LocalVariableTableEntry>,
}

pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl AttributeInfo for LocalVariableTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let local_variable_table_length = reader.read_u16();
        let mut local_variable_table = vec![];
        for _i in 0..local_variable_table_length {
            local_variable_table.push(LocalVariableTableEntry {
                start_pc: reader.read_u16(),
                length: reader.read_u16(),
                name_index: reader.read_u16(),
                descriptor_index: reader.read_u16(),
                index: reader.read_u16(),
            });
        }
        self.local_variable_table = local_variable_table;
    }
}

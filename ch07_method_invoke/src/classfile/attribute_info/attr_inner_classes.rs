/// InnerClasses_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 number_of_classes;
///     {   u2 inner_class_info_index;
///         u2 outer_class_info_index;
///         u2 inner_name_index;
///         u2 inner_class_access_flags;
///     } classes[number_of_classes];
/// }

use super::{AttributeInfo, ClassReader};

pub struct InnerClassesAttribute {
    classes: Vec<InnerClassInfo>,
}

pub struct InnerClassInfo {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

impl AttributeInfo for InnerClassesAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let number_of_classes = reader.read_u16();
        let mut classes = vec![];
        for _i in 0..number_of_classes {
            classes.push(InnerClassInfo {
                inner_class_info_index: reader.read_u16(),
                outer_class_info_index: reader.read_u16(),
                inner_name_index: reader.read_u16(),
                inner_class_access_flags: reader.read_u16(),
            });
        }
        self.classes = classes;
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// BootstrapMethods_attribute {
/// u2 attribute_name_index;
/// u4 attribute_length;
/// u2 num_bootstrap_methods;
///     {   u2 bootstrap_method_ref;
///         u2 num_bootstrap_arguments;
///         u2 bootstrap_arguments[num_bootstrap_arguments];
///     } bootstrap_methods[num_bootstrap_methods];
/// }

use super::{AttributeInfo, ClassReader};

pub struct BootstrapMethodsAttribute {
    bootstrap_methods: Vec<BootstrapMethod>,
}

impl AttributeInfo for BootstrapMethodsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let num_bootstrap_methods = reader.read_u16();
        let mut bootstrap_methods = vec![];
        for _i in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod {
                bootstrap_method_ref: reader.read_u16(),
                bootstrap_arguments: reader.read_u16s(),
            });
        }
        self.bootstrap_methods = bootstrap_methods;
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

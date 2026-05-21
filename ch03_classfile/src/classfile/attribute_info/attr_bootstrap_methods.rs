/// BootstrapMethods 属性：记录 invokedynamic 指令使用的引导方法（Java 7+）
///
/// BootstrapMethods_attribute {
///     u2 attribute_name_index;
///     u4 attribute_length;
///     u2 num_bootstrap_methods;
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
}

/// 单个引导方法：method_handle + 参数列表
pub struct BootstrapMethod {
    /// 指向常量池中 CONSTANT_MethodHandle_info
    bootstrap_method_ref: u16,
    /// 引导方法的静态参数列表，每个元素指向常量池条目
    bootstrap_arguments: Vec<u16>,
}

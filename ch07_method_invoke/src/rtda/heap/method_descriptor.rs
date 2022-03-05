
pub struct MethodDescriptor {
    parameter_typs: Vec<String>,
    return_type: String,
}

impl MethodDescriptor {

    pub fn parameter_typs(&self) -> Vec<String> {
        self.parameter_typs.clone()
    }

    pub fn return_type(&self) -> String {
        self.return_type.clone()
    }

    pub fn add_parameter_type(&mut self, t: String) {
        self.parameter_typs.push(t);
    }

}

pub struct MethodDescriptorParser {
    raw: String,
    offset: usize,
    parsed: Box<MethodDescriptor>,
}

impl MethodDescriptorParser {
    pub fn parse(descriptor: String) -> Box<MethodDescriptor> {
        let parsed = MethodDescriptor { parameter_typs: Vec::new(), return_type: "".into() };
        let parser = MethodDescriptorParser {
            raw: descriptor,
            offset: 0,
            parsed: Box::new(parsed),
        };

        parser.parsed
    }
}

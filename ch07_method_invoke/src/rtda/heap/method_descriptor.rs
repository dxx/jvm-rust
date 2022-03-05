
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

    pub fn start_params(&mut self) {
        if self.read_u8() != b'(' {
            self.cause_panic();
        }
    }

    pub fn end_params(&mut self) {
        if self.read_u8() != b')' {
            self.cause_panic();
        }
    }

    pub fn finish(&self) {
        if self.offset != self.raw.as_bytes().len() {
            self.cause_panic();
        }
    }

    pub fn cause_panic(&self) {
        panic!("BAD descriptor: {}", self.raw);
    }

    pub fn read_u8(&mut self) -> u8 {
        let b = self.raw.as_bytes()[self.offset];
        self.offset += 1;
        b
    }

    pub fn unread_u8(&mut self) {
        self.offset -= 1;
    }

    pub fn parse_param_types(&mut self) {
        loop {
            let t = self.parse_filed_type();
            if t == "" {
                break;
            }
            self.parsed.add_parameter_type(t);
        }
    }

    pub fn parse_return_type(&mut self) {
        if self.read_u8() == b'V' {
            self.parsed.return_type = "V".into();
            return;
        }
        self.unread_u8();
        let t = self.parse_filed_type();
        if t != "" {
            self.parsed.return_type = t;
            return;
        }
        self.cause_panic();
    }

    pub fn parse_filed_type(&mut self) -> String {
        return match self.read_u8() {
            b'B' => {
                "B".into()
            },
            b'C' => {
                "C".into()
            },
            b'D' => {
                "D".into()
            },
            b'F' => {
                "F".into()
            },
            b'I' => {
                "I".into()
            },
            b'J' => {
                "J".into()
            },
            b'S' => {
                "S".into()
            },
            b'Z' => {
                "Z".into()
            },
            b'L' => {
                self.parse_object_type()
            },
            b'[' => {
                self.parse_array_type()
            },
            _ => {
                self.unread_u8();
                "".into()
            }
        }
    }

    pub fn parse_object_type(&mut self) -> String {
        let unread = &self.raw.as_str()[self.offset..];
        let semicolon_index = unread.find(";");
        if semicolon_index.is_none() {
            self.cause_panic();
            return "".into();
        }
        let obj_start = self.offset - 1;
        let obj_end = self.offset + semicolon_index.unwrap() + 1;
        self.offset = obj_end;
        self.raw.as_str()[obj_start..obj_end].into()
    }

    pub fn parse_array_type(&mut self) -> String {
        let arr_start = self.offset - 1;
        self.parse_filed_type();
        let arr_end = self.offset;
        self.raw.as_str()[arr_start..arr_end].into()
    }

}

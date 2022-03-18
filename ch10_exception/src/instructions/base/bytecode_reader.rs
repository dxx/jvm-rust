#[derive(Default)]
pub struct BytecodeReader {
    code: Vec<u8>,
    pc: usize,
}

impl BytecodeReader {
    pub fn reset(&mut self, code: Vec<u8>, pc: usize) {
        self.code = code;
        self.pc = pc;
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = self.code[self.pc];
        self.pc += 1;
        byte
    }

    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    pub fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8() as u16;
        let byte2 = self.read_u8() as u16;
        (byte1 << 8) | byte2
    }

    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    pub fn read_i32(&mut self) -> i32 {
        let byte1 = self.read_u8() as i32;
        let byte2 = self.read_u8() as i32;
        let byte3 = self.read_u8() as i32;
        let byte4 = self.read_u8() as i32;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }

    /// Used by lookupswitch and tableswitch
    pub fn read_i32s(&mut self, n: i32) -> Vec<i32> {
        let mut ints = vec![];
        for _i in 0..n {
            ints.push(self.read_i32());
        }
        ints
    }

    /// Used by lookupswitch and tableswitch
    pub fn skip_padding(&mut self) {
        while self.pc % 4 != 0 {
            self.read_u8();
        }
    }
}

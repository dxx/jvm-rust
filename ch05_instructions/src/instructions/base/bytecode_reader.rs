//! 字节码读取器：用于从方法的字节码数组中按顺序读取数据

#[derive(Default)]
pub struct BytecodeReader {
    code: Vec<u8>,
    pc: usize,
}

impl BytecodeReader {
    /// 重置读取器状态，绑定新的字节码数组和 PC
    pub fn reset(&mut self, code: Vec<u8>, pc: usize) {
        self.code = code;
        self.pc = pc;
    }

    /// 获取当前 PC
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// 读取 1 字节无符号整数
    pub fn read_u8(&mut self) -> u8 {
        let byte = self.code[self.pc];
        self.pc += 1;
        byte
    }

    /// 读取 1 字节有符号整数
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    /// 读取 2 字节无符号整数（大端序）
    pub fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8() as u16;
        let byte2 = self.read_u8() as u16;
        (byte1 << 8) | byte2
    }

    /// 读取 2 字节有符号整数（大端序）
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    /// 读取 4 字节有符号整数（大端序）
    pub fn read_i32(&mut self) -> i32 {
        let byte1 = self.read_u8() as i32;
        let byte2 = self.read_u8() as i32;
        let byte3 = self.read_u8() as i32;
        let byte4 = self.read_u8() as i32;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }

    /// 连续读取 n 个 i32（供 lookupswitch 和 tableswitch 使用）
    /// Used by lookupswitch and tableswitch
    pub fn read_i32s(&mut self, n: i32) -> Vec<i32> {
        let mut ints = vec![];
        for _i in 0..n {
            ints.push(self.read_i32());
        }
        ints
    }

    /// 跳过填充字节，使 PC 对齐到 4 字节边界（供 lookupswitch 和 tableswitch 使用）
    /// Used by lookupswitch and tableswitch
    pub fn skip_padding(&mut self) {
        while self.pc % 4 != 0 {
            self.read_u8();
        }
    }
}

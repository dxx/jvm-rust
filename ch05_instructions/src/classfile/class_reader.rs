use std::convert::TryInto;

/// class 文件字节流读取器（big-endian）
/// 通过逐步消费 data 中的字节，按 JVM 规范读取 u1/u2/u4/u8 等类型数据
pub struct ClassReader {
    data: Vec<u8>,
}

impl ClassReader {
    pub fn new(data: Vec<u8>) -> Self {
        ClassReader { data }
    }

    /// 读取 1 字节无符号整数（u1）
    pub fn read_u8(&mut self) -> u8 {
        let val = self.data[0];
        self.data = self.data[1..].to_vec();
        val
    }

    /// 读取 2 字节无符号整数（u2，big-endian）
    pub fn read_u16(&mut self) -> u16 {
        let val = u16::from_be_bytes((&self.data[..2]).try_into().unwrap());
        self.data = self.data[2..].to_vec();
        val
    }

    /// 读取 4 字节无符号整数（u4，big-endian）
    pub fn read_u32(&mut self) -> u32 {
        let val = u32::from_be_bytes((&self.data[..4]).try_into().unwrap());
        self.data = self.data[4..].to_vec();
        val
    }

    /// 读取 8 字节无符号整数（big-endian），用于 long/double 常量
    pub fn read_u64(&mut self) -> u64 {
        let val = u64::from_be_bytes((&self.data[..8]).try_into().unwrap());
        self.data = self.data[8..].to_vec();
        val
    }

    /// 读取 u2 表：先读取一个 u16 作为表长度 n，再依次读取 n 个 u16
    pub fn read_u16s(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut s = vec![];
        for _i in 0..n {
            s.push(self.read_u16());
        }
        s
    }

    /// 连续读取 n 个字节
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let bytes = self.data[..n].to_vec();
        self.data = self.data[n..].to_vec();
        bytes
    }
}

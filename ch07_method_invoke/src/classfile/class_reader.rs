use std::convert::TryInto;

pub struct ClassReader {
    data: Vec<u8>,
}

impl ClassReader {
    pub fn new(data: Vec<u8>) -> Self {
        ClassReader {
            data
        }
    }

    /// u1
    pub fn read_u8(&mut self) -> u8 {
        let val = self.data[0];
        self.data = self.data[1..].to_vec();
        val
    }

    /// u2
    pub fn read_u16(&mut self) -> u16 {
        let val = u16::from_be_bytes((&self.data[..2]).try_into().unwrap());
        self.data = self.data[2..].to_vec();
        val
    }

    /// u4
    pub fn read_u32(&mut self) -> u32 {
        let val = u32::from_be_bytes((&self.data[..4]).try_into().unwrap());
        self.data = self.data[4..].to_vec();
        val
    }

    pub fn read_u64(&mut self) -> u64 {
        let val = u64::from_be_bytes((&self.data[..8]).try_into().unwrap());
        self.data = self.data[8..].to_vec();
        val
    }

    /// 读取 u2 表，表头的大小由开头的 u16 类型数据指定
    pub fn read_u16s(&mut self) -> Vec<u16> {
        let n = self.read_u16();
        let mut s = vec![];
        for _i in 0..n {
            s.push(self.read_u16());
        }
        s
    }

    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let bytes = self.data[..n].to_vec();
        self.data = self.data[n..].to_vec();
        bytes
    }
}
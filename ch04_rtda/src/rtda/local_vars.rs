use super::object::Object;
use crate::types::OptionalRcRefCell;

/// 变量槽（slot）：JVM 局部变量表 / 操作数栈的基本存储单元（32-bit）
/// 同一个 slot 内的 num 与 _ref 互斥使用：基本类型用 num，引用类型用 _ref
#[derive(Default, Clone)]
pub struct Slot {
    /// 32-bit 基本类型存储（int/float；long/double 由两个 slot 拼接）
    pub num: i32,
    /// 引用类型存储
    pub _ref: OptionalRcRefCell<Object>,
}

/// 局部变量表：方法运行时存放局部变量的固定大小数组
/// 大小由 Code 属性中的 max_locals 决定
pub struct LocalVars {
    slots: Vec<Slot>,
}

impl LocalVars {
    pub fn new(max_locals: usize) -> Self {
        LocalVars {
            slots: vec![Slot::default(); max_locals],
        }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.slots[index].num = val;
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.slots[index].num
    }

    /// float 通过 IEEE 754 位模式以 i32 存储
    pub fn set_float(&mut self, index: usize, val: f32) {
        let bytes = f32::to_be_bytes(val);
        self.slots[index].num = i32::from_be_bytes(bytes);
    }

    pub fn get_float(&self, index: usize) -> f32 {
        let bytes = i32::to_be_bytes(self.slots[index].num);
        f32::from_be_bytes(bytes)
    }

    /// long 占两个 slot：低 32 位放 index，高 32 位放 index+1
    pub fn set_long(&mut self, index: usize, val: i64) {
        self.slots[index].num = val as i32;
        self.slots[index + 1].num = (val >> 32) as i32;
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = self.slots[index].num as u32;
        let high = self.slots[index + 1].num as u32;
        (high as i64) << 32 | low as i64
    }

    /// double 同样占两个 slot，先按位转 i64 再走 set_long
    pub fn set_double(&mut self, index: usize, val: f64) {
        let bytes = f64::to_be_bytes(val);
        let val = i64::from_be_bytes(bytes);
        self.set_long(index, val);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let bytes = i64::to_be_bytes(self.get_long(index));
        f64::from_be_bytes(bytes)
    }

    /// 引用类型（含 null）
    pub fn set_ref(&mut self, index: usize, _ref: OptionalRcRefCell<Object>) {
        self.slots[index]._ref = _ref;
    }

    pub fn get_ref(&self, index: usize) -> OptionalRcRefCell<Object> {
        self.slots[index]._ref.clone()
    }
}

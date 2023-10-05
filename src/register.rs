use crate::types::*;

#[derive(Copy, Clone)]
pub struct IntRegister {
    value: Int,
}

impl IntRegister {
    pub fn new() -> Self {
        IntRegister { value: 0 }
    }
    pub fn set(&mut self, value: Int) {
        self.value = value;
    }
    pub fn get(&self) -> Int {
        self.value
    }
}

#[derive(Copy, Clone)]
pub struct FloatRegister {
    value: Float,
}

impl FloatRegister {
    pub fn new() -> Self {
        FloatRegister { value: 0. }
    }
    pub fn set(&mut self, value: Float) {
        self.value = value;
    }
    pub fn get(&self) -> Float {
        self.value
    }
}

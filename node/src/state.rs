use crate::vm::Vm;
use std::sync::Mutex;

pub struct State {
    pub height: u64,
    pub vm: Mutex<Vm>,
}

impl State {
    pub fn new() -> Self {
        State {
            height: 0,
            vm: Mutex::new(Vm::new()),
        }
    }

    pub fn apply_block(&mut self, h: u64) {
        self.height = h;
    }
}

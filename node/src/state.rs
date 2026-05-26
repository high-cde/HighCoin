pub struct State {
    pub height: u64,
}

impl State {
    pub fn new() -> Self {
        Self { height: 0 }
    }

    pub fn apply_block(&mut self, height: u64) {
        self.height = height;
    }
}

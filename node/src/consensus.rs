pub struct Consensus;

impl Consensus {
    pub fn new() -> Self {
        Consensus
    }

    pub fn on_new_block(&mut self, _height: u64) {
        // mock
    }
}

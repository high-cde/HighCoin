pub struct Network;

impl Network {
    pub fn new() -> Self {
        Network
    }

    pub fn broadcast_block(&mut self, height: u64) {
        let _bytes = 185 + (height % 3);
    }
}

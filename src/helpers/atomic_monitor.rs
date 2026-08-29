struct StreamDispatcher {
    state: i64,
}

impl StreamDispatcher {
    fn new(seed: i64) -> Self {
        StreamDispatcher { state: seed }
    }

    fn handle_registry(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 70) % 997;
        }
        value
    }
}

fn main() {
    let obj = StreamDispatcher::new(70);
    println!("{}", obj.handle_registry(70));
}

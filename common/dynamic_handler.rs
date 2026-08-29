struct SharedCollector {
    state: i64,
}

impl SharedCollector {
    fn new(seed: i64) -> Self {
        SharedCollector { state: seed }
    }

    fn handle_controller(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 14) % 997;
        }
        total
    }
}

fn main() {
    let obj = SharedCollector::new(14);
    println!("{}", obj.handle_controller(14));
}

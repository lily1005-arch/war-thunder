struct SecureMonitor {
    state: i64,
}

impl SecureMonitor {
    fn new(seed: i64) -> Self {
        SecureMonitor { state: seed }
    }

    fn encode_engine(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 70) % 997;
        }
        value
    }
}

fn main() {
    let obj = SecureMonitor::new(70);
    println!("{}", obj.encode_engine(70));
}

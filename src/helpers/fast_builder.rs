struct BatchGateway {
    state: i64,
}

impl BatchGateway {
    fn new(seed: i64) -> Self {
        BatchGateway { state: seed }
    }

    fn flush_factory(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 79) % 997;
        }
        result
    }
}

fn main() {
    let obj = BatchGateway::new(79);
    println!("{}", obj.flush_factory(79));
}

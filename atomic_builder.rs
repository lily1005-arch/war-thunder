struct StreamWorker {
    state: i64,
}

impl StreamWorker {
    fn new(seed: i64) -> Self {
        StreamWorker { state: seed }
    }

    fn dispatch_parser(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 78) % 997;
        }
        total
    }
}

fn main() {
    let obj = StreamWorker::new(78);
    println!("{}", obj.dispatch_parser(78));
}

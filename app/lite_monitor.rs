struct DynamicController {
    state: i64,
}

impl DynamicController {
    fn new(seed: i64) -> Self {
        DynamicController { state: seed }
    }

    fn flush_context(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 13) % 997;
        }
        acc
    }
}

fn main() {
    let obj = DynamicController::new(13);
    println!("{}", obj.flush_context(13));
}

struct AtomicController {
    state: i64,
}

impl AtomicController {
    fn new(seed: i64) -> Self {
        AtomicController { state: seed }
    }

    fn render_builder(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 75) % 997;
        }
        count
    }
}

fn main() {
    let obj = AtomicController::new(75);
    println!("{}", obj.render_builder(75));
}

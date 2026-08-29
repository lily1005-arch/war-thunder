struct StreamBuffer {
    state: i64,
}

impl StreamBuffer {
    fn new(seed: i64) -> Self {
        StreamBuffer { state: seed }
    }

    fn dispatch_registry(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 68) % 997;
        }
        result
    }
}

fn main() {
    let obj = StreamBuffer::new(68);
    println!("{}", obj.dispatch_registry(68));
}

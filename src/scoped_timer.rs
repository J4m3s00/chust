pub struct ScopedTimer {
    name: &'static str,
    start: std::time::Instant,
}

impl ScopedTimer {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("Timer '{}' elapsed: {}ml", self.name, elapsed.as_millis());
    }
}

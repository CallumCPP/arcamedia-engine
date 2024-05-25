pub struct Timer {
    start_time: f64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: web_sys::window().unwrap().performance().unwrap().now(),
        }
    }

    pub fn elapsed_reset(&mut self) -> f64 {
        let elapsed = web_sys::window().unwrap().performance().unwrap().now() - self.start_time;
        self.start_time = web_sys::window().unwrap().performance().unwrap().now();

        elapsed
    }

    pub fn elapsed(&self) -> f64 {
        let elapsed = web_sys::window().unwrap().performance().unwrap().now() - self.start_time;

        elapsed
    }
}

pub struct TemperatureWindow {
    lower: i32,
    upper: i32,
}

impl TemperatureWindow {
    const TEMP_OFFSET: i32 = 5;

    pub fn new(init: i32) -> Self {
        Self {
            upper: init,
            lower: init,
        }
    }

    pub fn update(&mut self, t: i32) {
        if t >= self.upper {
            self.upper = t;
            self.lower = t - Self::TEMP_OFFSET;
        } else if t <= self.lower {
            self.lower = t;
            self.upper = t + Self::TEMP_OFFSET;
        }
    }

    pub fn get_temp(&self) -> i32 {
        self.upper
    }
}

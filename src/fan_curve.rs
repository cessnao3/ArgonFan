pub struct FanCurve {
    temps: Vec<(i32, u8)>,
    default_speed: u8,
}

impl FanCurve {
    const MAX_SPEED: u8 = 100;

    pub fn new(default_speed: u8, values: &[(i32, u8)]) -> Result<Self, String> {
        for ((a_t, a_s), (b_t, b_s)) in values.iter().zip(&values[1..]) {
            if a_t >= b_t {
                return Err(format!("Temperature {a_t} must be less than {b_t}"));
            }

            if *a_s > Self::MAX_SPEED || *b_s > Self::MAX_SPEED {
                return Err(format!(
                    "Speeds {a_s} and {b_s} should be below {}",
                    Self::MAX_SPEED
                ));
            }
        }

        Ok(Self {
            default_speed,
            temps: values.to_vec(),
        })
    }

    pub fn get_speed(&self, temp: i32) -> u8 {
        let mut res = self.default_speed;

        for (brk_temp, brk_spd) in self.temps.iter() {
            if temp < *brk_temp {
                return res;
            }

            res = *brk_spd;
        }

        res
    }
}

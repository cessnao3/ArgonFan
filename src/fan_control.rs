#[cfg(not(feature = "mocki2c"))]
use rppal::i2c::I2c;

#[cfg(feature = "mocki2c")]
use mock_i2c::MockI2c as I2c;

pub struct FanControl {
    i2c: I2c,
    last_speed: u8,
}

impl FanControl {
    const BUS_ADDR: u16 = 0x1a;

    pub fn new() -> Self {
        Self::new_with_init(0)
    }

    pub fn new_with_init(spd: u8) -> Self {
        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(Self::BUS_ADDR).unwrap();

        i2c.smbus_send_byte(spd).unwrap();

        Self {
            i2c,
            last_speed: spd,
        }
    }

    pub fn set_speed(&mut self, spd: u8) -> bool {
        let change_needed = spd != self.last_speed;

        if change_needed {
            self.i2c.smbus_send_byte(spd).unwrap();
            if spd != self.last_speed {
                self.last_speed = spd;
            }
        }

        change_needed
    }
}

#[cfg(feature = "mocki2c")]
mod mock_i2c {
    #[derive(Debug)]
    pub struct MockI2c;

    #[derive(Debug, Copy, Clone)]
    pub struct MockI2cError;

    impl MockI2c {
        pub fn new() -> Result<Self, MockI2cError> {
            Ok(Self)
        }

        pub fn set_slave_address(&self, _addr: u16) -> Result<(), MockI2cError> {
            Ok(())
        }

        pub fn smbus_send_byte(&mut self, _b: u8) -> Result<(), MockI2cError> {
            Ok(())
        }
    }
}

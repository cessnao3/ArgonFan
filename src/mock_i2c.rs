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

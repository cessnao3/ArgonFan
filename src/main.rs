mod fan_curve;
mod temp_window;

use std::time::Duration;

use rppal::i2c::I2c;
use sysinfo::{ComponentExt, RefreshKind, System, SystemExt};

use fan_curve::FanCurve;
use temp_window::TemperatureWindow;

const BUS_ADDR: u16 = 0x1a;

fn main() {
    let fan_curve = FanCurve::new(0, &[(55, 10), (60, 55), (65, 100)]).unwrap();
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_components().with_components_list());

    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(BUS_ADDR).unwrap();

    let mut window = TemperatureWindow::new(0);
    let mut last_speed = fan_curve.get_default_speed();

    loop {
        sys.refresh_all();

        let immediate_temperature = sys
            .components()
            .iter()
            .map(|c| c.temperature() as i32)
            .max()
            .expect("Unable to get maximum system temperature");

        window.update(immediate_temperature);
        let temperature = window.get_temp();

        let spd = fan_curve.get_speed(temperature);
        i2c.smbus_send_byte(spd).unwrap();

        if spd != last_speed {
            println!("Updating Speed to {spd} @ {temperature} C");
            last_speed = spd;
        }

        std::thread::sleep(Duration::from_millis(2000));
    }
}

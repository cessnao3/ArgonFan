mod fan_control;
mod fan_curve;
mod temp_window;

use std::time::Duration;

use sysinfo::{ComponentExt, RefreshKind, System, SystemExt};

use fan_control::FanControl;
use fan_curve::FanCurve;
use temp_window::TemperatureWindow;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    print!("Fan Controller");
    if let Some(vers) = option_env!("CARGO_PKG_VERSION") {
        print!(" v{vers}")
    }
    println!(" Starting");

    let mut control = FanControl::new();
    let fan_curve = FanCurve::new(0, &[(55, 10), (60, 55), (65, 100)]).unwrap();
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_components().with_components_list());

    let mut window = TemperatureWindow::new(0);

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, term.clone()).unwrap();
    signal_hook::flag::register(signal_hook::consts::SIGINT, term.clone()).unwrap();

    while !term.load(Ordering::Relaxed) {
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

        if control.set_speed(spd) {
            println!("Updating Speed to {spd} @ {temperature} C");
        }

        std::thread::sleep(Duration::from_millis(2000));
    }

    println!("Fan Controller Exiting");
}

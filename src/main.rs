mod fan_control;
mod fan_curve;
mod temp_window;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration
};

use fan_control::FanControl;
use fan_curve::FanCurve;
use temp_window::TemperatureWindow;

fn main() {
    print!("Fan Controller");
    if let Some(vers) = option_env!("CARGO_PKG_VERSION") {
        print!(" v{vers}")
    }
    println!(" Starting");

    let mut control = FanControl::new();
    let fan_curve = FanCurve::new(0, &[(55, 10), (60, 55), (65, 100)]).unwrap();
    let mut sys = sysinfo::Components::new_with_refreshed_list();

    for c in sys.iter() {
        println!("Reading temperature from: {}", c.label());
    }

    let mut window = TemperatureWindow::new(0);

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, term.clone()).unwrap();
    signal_hook::flag::register(signal_hook::consts::SIGINT, term.clone()).unwrap();

    while !term.load(Ordering::Relaxed) {
        sys.refresh();

        let immediate_temperature = sys
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

        std::thread::sleep(Duration::from_millis(5000));
    }

    println!("Fan Controller Exiting");
}

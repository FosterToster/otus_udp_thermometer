mod thermometer_listener;

use std::sync::{Arc, Mutex};
use thermometer_listener::ThermometerListener;

fn main() {
    if let Some(listener) = ThermometerListener::new() {
        // shared variable for received temperature measurement
        let last_measure = Arc::new(Mutex::new(String::from("N/A")));

        // starting a thread with thermometer listener
        let last_measure_clone = last_measure.clone();
        let _ = std::thread::spawn(move || loop {
            let new_measure = listener.receive_measure();
            let mut lg = last_measure_clone.lock().unwrap();

            *lg = new_measure;

            std::mem::drop(lg);
        });

        // displaying last known measurement
        loop {
            let lg = last_measure.lock().unwrap();
            println!("last known measure: {}", lg);
            std::mem::drop(lg);
            std::thread::sleep(std::time::Duration::from_secs(1))
        }
    }
}

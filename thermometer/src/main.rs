mod thermometer;

use thermometer::Thermometer;

fn main() {
    if let Some(thermometer) = Thermometer::new("127.0.0.1:8601") {
        loop {
            thermometer.poll();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

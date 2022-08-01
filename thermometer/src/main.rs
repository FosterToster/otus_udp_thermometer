mod thermometer;

use thermometer::Thermometer;

#[tokio::main]
async fn main() {
    if let Some(thermometer) = Thermometer::new("127.0.0.1:8601").await {
        loop {
            thermometer.poll().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

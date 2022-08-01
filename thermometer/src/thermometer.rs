// use std::net::UdpSocket;
use tokio::net::UdpSocket;
use std::time::Instant;

pub struct Thermometer {
    socket: UdpSocket,
    receiver: String,
    instant: Instant,
}

impl Thermometer {
    pub async fn new(receiver: &str) -> Option<Self> {
        Some(Self {
            socket: loop {
                let port: u16 = 30000;

                match UdpSocket::bind(format!("0.0.0.0:{}", port)).await {
                    Ok(socket) => break socket,
                    Err(_) => port.checked_add(1)?,
                };
            },
            receiver: receiver.into(),
            instant: Instant::now(),
        })
    }

    async fn measure_celsium(&self) -> f64 {
        20.0 + 10.0 * (self.instant.elapsed().as_nanos() as f64).sin()
    }

    async fn send_measure(&self, buf: &str) -> Result<usize, std::io::Error> {
        self.socket.send_to(buf.as_bytes(), &self.receiver).await
    }

    pub async fn poll(&self) {
        if self
            .send_measure(&format!("{:.1}", self.measure_celsium().await)).await
            .is_err()
        {
            println!("Unable to send measured temperature...")
        }
    }
}

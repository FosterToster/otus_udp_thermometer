use std::net::UdpSocket;
use std::time::Instant;

pub struct Thermometer {
    socket: UdpSocket,
    receiver: String,
    instant: Instant,
}

impl Thermometer {
    pub fn new(receiver: &str) -> Option<Self> {
        Some(Self {
            socket: loop {
                let port: u16 = 30000;

                match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
                    Ok(socket) => break socket,
                    Err(_) => port.checked_add(1)?,
                };
            },
            receiver: receiver.into(),
            instant: Instant::now(),
        })
    }

    fn measure_celsium(&self) -> f64 {
        20.0 + 10.0 * (self.instant.elapsed().as_nanos() as f64).sin()
    }

    fn send_measure(&self, buf: &str) -> Result<usize, std::io::Error> {
        self.socket.send_to(buf.as_bytes(), &self.receiver)
    }

    pub fn poll(&self) {
        if self
            .send_measure(&format!("{:.1}", self.measure_celsium()))
            .is_err()
        {
            println!("Unable to send measured temperature...")
        }
    }
}

use std::net::UdpSocket;

pub struct ThermometerListener {
    socket: UdpSocket,
}

impl ThermometerListener {
    pub fn new() -> Option<Self> {
        Some(Self {
            socket: {
                if let Ok(socket) = UdpSocket::bind("0.0.0.0:8601") {
                    socket
                } else {
                    return None;
                }
            },
        })
    }

    pub fn receive_measure(&self) -> String {
        let mut buf = [0; 10];

        if let Ok((recvlen, _)) = self.socket.recv_from(&mut buf) {
            if let Ok(string) = String::from_utf8(buf[..recvlen].to_vec()) {
                string
            } else {
                String::from("<Decode error>")
            }
        } else {
            String::from("<Receive error>")
        }
    }
}

use tokio::net::UdpSocket;

pub struct ThermometerListener {
    socket: UdpSocket,
}

impl ThermometerListener {
    pub async fn new() -> Option<Self> {
        Some(Self {
            socket: {
                if let Ok(socket) = UdpSocket::bind("0.0.0.0:8601").await {
                    socket
                } else {
                    return None;
                }
            },
        })
    }

    pub async fn receive_measure(&self) -> String {
        let mut buf = [0; 10];

        if let Ok((recvlen, _)) = self.socket.recv_from(&mut buf).await {
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

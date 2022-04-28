use std::net::IpAddr;
use tokio::net::TcpStream;

mod control_chars;

pub struct TelnetClient {
    stream: TcpStream,
}
impl TelnetClient {
    pub fn builder(host: IpAddr) -> TelnetClientBuilder {
        TelnetClientBuilder { host, port: 23 }
    }
}

pub struct TelnetClientBuilder {
    host: IpAddr,
    port: u16,
}

impl TelnetClientBuilder {
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn build(self) -> Result<TelnetClient, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect((self.host, self.port)).await?;
        Ok(TelnetClient { stream })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

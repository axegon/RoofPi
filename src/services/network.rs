use std::net::{SocketAddr, UdpSocket};

pub(crate) struct Network {}

impl Network {
    /// Get the IP address of the device
    /// # Returns
    /// A string representation of the IP address
    /// # Example
    /// ```
    /// let ip = Network::get_ip_address();
    /// println!("IP Address: {}", ip);
    pub fn get_ip_address() -> String {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                return "NOT CONNECTED".to_string();
            }
        };
        match socket.connect("8.8.8.8:53") {
            Ok(_) => {
                if let Ok(local_addr) = socket.local_addr() {
                    if let SocketAddr::V4(addr) = local_addr {
                        return addr.ip().to_string();
                    }
                }
                "CANNOT DETERMINE IP".to_string()
            }
            Err(e) => {
                println!("Error: {}", e);
                "CAN'T CONNECT".to_string()
            }
        }
    }
}

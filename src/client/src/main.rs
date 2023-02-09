use std::net::TcpStream;

use common::notif::{Notif, notify};
use common::stream as Stream;

fn main() {
    notify(Notif::Info("Starting Client..."));

    let mut stream: TcpStream = match TcpStream::connect("127.0.0.1:2000") {
        Ok(s) => {
            s
        }
        
        Err(_) => {
            notify(Notif::Err("Failed to connect to server! Exiting..."));
            return;           
        }
    };
    
    let mut buffer: String = String::new();
    
    loop {
        match Stream::read(&mut stream, &mut buffer) {
            Ok(_) => {
                
            }
            
            Err(_) => {
                notify(Notif::Err("Failed to read stream!"));
            }
        }
        
        println!("Message Recieved: {}", buffer);
        buffer.clear();
    }
}
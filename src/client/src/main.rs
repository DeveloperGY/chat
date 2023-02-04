use std::net::TcpStream;
use std::io::prelude::*;

use common::notif::{Notif, notify};

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
        match stream.read_to_string(&mut buffer) {
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
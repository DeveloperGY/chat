mod msg_hdl;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;

use common::notif::{Notif, notify};
// use msg_hdl;

fn main() {
    notify(Notif::Info("Starting Server..."));
    let listener = match bind_address("127.0.0.1:2000") {
        Some(l) => l,
        None => return
    };
    
    let mut connection_handles: Vec<thread::JoinHandle<()>> = Vec::new();
    
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => {
                notify(Notif::Info("Client Connected!"));
                s
            },
            Err(_) => {
                notify(Notif::Err("Failed to connect to client!"));
                return;      
            }
        };
        
        connection_handles.push(thread::spawn(move || {handle_connection(&mut stream)}));
    }
    
    for handle in connection_handles {
        match handle.join() {
            Ok(_) => {
                notify(Notif::Info("Connection Closed!"));
            }
            
            Err(_) => {
                notify(Notif::Err("Failed to close connection, connection thread crashed!"));
            }
        };
    }
}

fn bind_address(address: &str) -> Option<TcpListener> {
    notify(Notif::Info("Binding Address..."));
    
    match TcpListener::bind(address) {
        Ok(listener) => {
            notify(Notif::Info("Address Bound!"));
            Some(listener)
        }
        
        Err(_) => {
            notify(Notif::Err("Failed to bind address! Exiting..."));
            None
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    loop {
        match stream.write_all("Hello World!".as_bytes()) {
            Ok(_) => {
                
            }
            
            Err(_) => {
                notify(Notif::Err("Failed to send packet!"));
            }
        };
        
        match stream.flush() {
            _ => {}
        }
    }
}
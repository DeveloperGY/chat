use std::net::TcpStream;
use std::io::prelude::*;

pub fn read(stream: &mut TcpStream, buffer: &mut String) -> Result<(), ()> {
    let mut buf: [u8; 1024] = [0; 1024];
    
    match stream.read(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err(())
    };
    
    *buffer = String::from_utf8_lossy(&buf[..]).to_string().clone();
    
    Ok(())
}

pub fn write(stream: &mut TcpStream, buffer: &str) -> Result<(), ()> {
    let buf: &[u8] = buffer.as_bytes();
    
    if buf.len() > 1024 {
        return Err(());
    }
    
    match stream.write_all(buf) {
        Ok(_) => {
            match stream.flush() {
                _ => {}
            };
            Ok(())
        },
        Err(_) => Err(())
    }
}
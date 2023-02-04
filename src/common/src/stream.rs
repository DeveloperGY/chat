use std::net::TcpStream;
use std::io::prelude::*;

pub fn read<'a>(stream: &mut TcpStream, buffer: &mut String, delimiter: &'a str) -> Result<usize, ()>{
    let mut reading: bool = true;
    let mut internal_buffer: [u8; 1] = [0];
    
    stream.read_exact(&mut internal_buffer);
    
    while reading {
        
        
        
        reading = false;
    }
    
    Ok(buffer.len())
}
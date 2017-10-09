use std::net::TcpListener;
use std::error::Error;
use std::io::{Read, Write};

fn main() {
    if let Err(e) = run() {
        println!("Aborted server with error: {}", e);
    }
}

fn run () -> Result<(), Box<Error>> {
    let listner = TcpListener::bind("127.0.0.1:1256")?;
    for incoming in listner.incoming() {
        let mut incoming = incoming?;
        println!("Connection from: {}", incoming.peer_addr()?);
        let mut buf = [0; 512];
        let read = incoming.read(&mut buf)?;
        print!("Request: {}", String::from_utf8_lossy(&buf));
        incoming.write(&buf[..read])?;
        incoming.flush()?;
        println!("Closing Connection\n");
    }
    Ok(())
}
use std::net::TcpListener;
use std::error::Error;
use std::io::{Read, Write};
use std::env::args;

const BUF: usize = 512;

fn main() {
    println!("Started server");
    if let Err(e) = run() {
        println!("Aborted server with error: {}", e);
    }
}

fn run() -> Result<(), Box<Error>> {
    let port = args().nth(1).unwrap_or("8080".into());
    let listner = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    
    for incoming in listner.incoming() {
        let mut incoming = incoming?;
        println!("Connection from: {}:", incoming.peer_addr()?);
        let mut buf = [0; BUF];
        let read = incoming.read(&mut buf)?;
        if read == BUF {
            println!("Warning: Request body truncated.");
        }
        println!("  Request: {:?}", String::from_utf8_lossy(&buf[..read]));
        incoming.write_all(&buf[..read])?;
        println!("  Closing Connection\n");
    }

    Ok(())
}
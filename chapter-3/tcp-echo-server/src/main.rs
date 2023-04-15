use std::net::{TcpListener, TcpStream};
use std::thread;


use std::io::{Read, Write, Error};

//Handles a single client 
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);

    let mut buff = [0; 512];
    loop{
        let bytes_read = stream.read(&mut buff)?;
        if bytes_read == 0 {return Ok(());}
        stream.write(&buff[..bytes_read])?;
    }
}

fn main(){

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {

            Err(e) => { eprintln!("failed: {} ", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

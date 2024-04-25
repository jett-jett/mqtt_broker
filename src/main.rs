use std::io::BufRead;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;


fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:1883")?;
    while let Ok((stream, _)) = listener.accept() {
        println!("New client connected: {:?}", stream.peer_addr());
   }
    Ok(())
}


//TODO ADD Inital Connection Response
//TODO ADD SUBSCRIPING
//TODO ADD TOPIC
//TODO ADD RESPONSES
//TODO ADD MULTIPLE THREADS


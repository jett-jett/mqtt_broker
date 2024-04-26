use std::io::BufRead;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;


fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:1883")?;
    while let Ok((mut stream, _)) = listener.accept() {
        println!("New client connected: {:?}", stream.peer_addr());
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).expect("Unable to read from buffer");
        println!("Recieved: {:#04X?}", buffer);
   }
    Ok(())
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .collect();
    println!("Request: {:#?}", request);
}
//TODO ADD Inital Connection Response
//TODO ADD SUBSCRIPING
//TODO ADD TOPIC
//TODO ADD RESPONSES
//TODO ADD MULTIPLE THREADS


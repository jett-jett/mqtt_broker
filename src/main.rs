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

// fn handle_connection(mut stream: TcpStream){

//     let mut buf_reader = BufReader::new(&mut stream);
//     let req_lines:Vec<_> = buf_reader.by_ref().lines()
//         .map(|result| result.unwrap_or_else(|_| return "".to_string()))
//         .take_while(|line| !line.is_empty() )
//         .collect();
    
   
//     for a in req_lines {
//         println!("Stream: {a}");
//     }
    
//     //let mut buffer = [0;1024];
//     //stream.read(&mut buffer).unwrap();

// }

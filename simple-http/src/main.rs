use std::{
    io::{self, Write, Read},
    net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr, IpAddr},
};

use simple_http::http::request;

fn create_socket() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];  
    stream.read(&mut buffer)?; 

    let buf_str = String::from_utf8_lossy(&buffer);
    let request = request::HttpRequest::new(&buf_str)?;

    println!("{:?}", request);

    // let valid_response: &str = "HTTP/1.1 200\ncontent-type: text/html\nvary: Accept-Encoding\r\n\r\n\
    // <html>
    // <body>
    // <h1>Hello World!</h1>
    // </body>
    // </html>
    // ";

    // stream.write(valid_response.as_bytes())?; 
    stream.write(&mut buffer)?;    
    stream.flush()?;
    Ok(())
}


fn serve(socket: SocketAddr) -> io::Result<()> {
    // let listener: TcpListener = TcpListener::bind(addr: socket)?; 
    let listener: TcpListener = TcpListener::bind(socket)?; //removed addr
    let mut counter: i32 = 0;
    // for stream: Result<TcpStream, Error> in listener.incoming() {
        for stream in listener.incoming() {
        // match std::thread::spawn(|| handle_client(stream: &mut stream?)).join() {
        match std::thread::spawn(|| handle_client(&mut stream?)).join() {
            Ok(_) => {
                counter += 1;
                println!("Connected stream... {}", counter);
            }
            Err(_) => continue,     
        };
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let socket: SocketAddr = create_socket();
    serve(socket)?;
    Ok(())
}
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, error::Error,
};

fn main()->Result<(), Box<dyn Error>> {

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for (i, stream) in listener.incoming().enumerate(){
        let stream = stream.unwrap();
        println!("stream {i} detected: {stream:?}");
        handle(stream);
    }

    Ok(())
}

fn handle(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    let request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();

    
    println!("request: {request:?}\n");

    let html = "<h1>Hello</h1>";
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", html.len(), html);

    stream.write_all(response.as_bytes()).unwrap();
}

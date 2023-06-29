use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, error::Error,
};

fn main()->Result<(), Box<dyn Error>> {

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for (_i, stream) in listener.incoming().enumerate(){
        let stream = stream.unwrap();
        println!("stream detected: {stream:?}");
        // handle tcp stream 
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

    
    println!("request: {request:?}");
    println!("INSERTION SARA");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn do_smth(){

}

fn do_smth_2(){
    
}

fn le_func(){
    
}


/// aaaaaaaaaaaaaaa aaaaaaaaaaaaaaa
/// bbbbbbbbbbbbbbb bbbbbbbbbbbbbbb
/// kkkkkkkkkkkkkkk kkkkkkkkkkkkkkk
/// ccccccccccccccc ccccccccccccccc
/// ddddddddddddddd ddddddddddddddd
/// yyyyyyyyyyyyyyy yyyyyyyyyyyyyyy
/// oooooooooooooooooooooooooooooo0
/// iiiiiiiiiiiiiiiiiiiiiiiiiiiiii
/// uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu
/// uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu new
/// hshshhhhhhhhhhhhhhhhhhhhhhhhhss
fn le_func_2(){

}
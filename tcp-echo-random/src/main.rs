use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rand::Rng;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        // let sleep = Duration::from_secs(*rand::thread_rng().choose(&[0, 1, 2, 3, 4, 5]).unwrap());
        let sleep = Duration::from_secs(rand::thread_rng().gen_range(0..=5));
        println!("Sleeping for {:?} before replying", sleep);
        thread::sleep(sleep);
        stream.write(&buf[..bytes_read])?;
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind");
    println!("ready");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err))
                });
            }
        }
    }
}

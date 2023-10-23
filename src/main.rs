use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server_in_rust::ThreadPool;

const PORT: &str = "127.0.0.1:7878";
const THREADNUMBER: usize = 4;

fn main() {
    let listener = match TcpListener::bind(PORT) {
        Ok(tcp) => tcp,
        Err(error) => panic!("Error na criação do listener: {error}"),
    };

    println!("Listening on: {}", PORT);
    let pool = ThreadPool::new(THREADNUMBER);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| {
                handle_connection(stream);
            }),
            Err(error) => panic!("{error}"),
        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents: String = match fs::read_to_string(filename) {
        Ok(string_file) => string_file,
        Err(error) => panic!("Não foi possivel ler o arquivo Html {}", error),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

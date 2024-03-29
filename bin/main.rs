use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("192.168.15.189:7878").unwrap();

    let pool = ThreadPool::new(4);

    let mut cnt: i32 = 0;

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        cnt += 1;
        println!("execute {}",  cnt);

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");

}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

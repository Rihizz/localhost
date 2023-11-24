use std::{ fs, io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream } };

fn main() {
    let listener_8888 = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind 8888");
    let listener_7878 = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind 7878");

    let mut incoming_7878 = listener_8888.incoming().peekable();
    let mut incoming_8888 = listener_7878.incoming().peekable();

    loop {
        if let Some(Ok(stream_7878)) = incoming_7878.next() {
            handle_connection(stream_7878);
        }

        if let Some(Ok(stream_8888)) = incoming_8888.next() {
            handle_connection(stream_8888);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    /*     let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from stream");
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello")
        .expect("Failed to write to stream"); */
}

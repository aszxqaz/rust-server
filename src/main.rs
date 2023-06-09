use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());

    let listener = TcpListener::bind(String::from_iter(["127.0.0.1:", &port])).unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTPS/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTPS/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTPS/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

mod tests {
    #[test]
    fn correct_address() {
        let port = std::env::var("PORT").unwrap_or_else(|_| "34254".to_string());

        assert_eq!(String::from_iter(["0.0.0.0:", &port]), "0.0.0.0:34254");
    }
}

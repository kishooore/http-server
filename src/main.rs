use std::fs;
use std::io::BufRead;

use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::net::TcpListener;

struct TcpServer {
    _host: String,
    _port: u16
}

impl TcpServer {
    pub fn new(host: String, port: u16) -> Self {
        Self {_host: host, _port: port}
    }

    fn start(&self) {
        let address = format!("{}:{}", self._host, self._port);
        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_request(stream);
        }
    }

    fn handle_request(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn main() {
    let host = String::from("127.0.0.1");
    let port: u16 = 8080;
    let server = TcpServer::new(host, port);
    server.start();
}


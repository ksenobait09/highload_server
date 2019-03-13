use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::net::TcpStream;
use std::str;

use super::config::Config;
use super::request::*;

static CRLF: &'static str = "\r\n";
static HTTP: &'static str = "HTTP/1.1 ";


pub struct Server {
    pub config: Config,
}

impl Server {
    pub fn new() -> Server {
        Server {
            config: Config::new(),
        }
    }
    #[allow(unused, unused_mut)]
    pub fn handle_request(document_root: &str, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => return,
            Ok(..) => {}
            Err(_e) => return,
        };

        let buffer = str::from_utf8(&buffer).unwrap();
        let mut split = buffer.split("\r\n");
        let mut request_raw = split.next().unwrap();

        let mut res = create_response_for_request(document_root, request_raw);
        let res = match res {
            Some(r) => r,
            None => return,
        };

        println!("=========================================\n\
        REQUEST: {:?}\n\
        RESPONSE:\n\
        file: {:?} \n\
        status: {:?}\n\
        headers: {:?}\n\
        =========================================",request_raw, res.file, res.status, res.headers);
        res.send(&mut stream)
    }
}


pub struct Response {
    pub headers: Vec<String>,
    pub status: Option<u32>,
    pub file: Option<File>,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: vec![],
            status: None,
            file: None,
        }
    }

    pub fn send(self, ref mut stream: &mut TcpStream) {
        let mut buf = String::new();
        buf.push_str(HTTP);
        buf.push_str(self.status.unwrap().to_string().as_str());
        buf.push_str(" anything");
        buf.push_str(CRLF);
        for h in self.headers {
            buf.push_str(h.as_str());
            buf.push_str(CRLF);
        }
        buf.push_str(CRLF);

        stream.write(buf.as_bytes()).unwrap();

        match self.file {
            Some(mut f) => {
                let mut buf = [0; 1024 * 1024];
                let mut n: u64 = 0;
                loop {
                    match f.read(&mut buf).unwrap() {
                        0 => {
                            break;
                        }
                        i => {
                            n += i as u64;
                            stream.write(&buf[..i]).unwrap();
                            f.seek(SeekFrom::Start(n as u64)).unwrap();
                        }
                    }
                }
            }
            None => {}
        }
        stream.flush().unwrap();
    }
}
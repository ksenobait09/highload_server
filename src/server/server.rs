use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use super::request::*;
use super::config::Config;


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
    fn handle_request(document_root: &str, mut stream: TcpStream) {
        let mut buffer: String = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        let mut split = buffer.split("\r\n");
        let mut request_raw = split.next().unwrap();

        let mut req = create_response_for_request(document_root, request_raw);
        let req = match req {
            Some(r) => r,
            None => return,
        };

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

    pub fn send(&self, ref mut stream: &mut TcpStream) {
        let mut buf = String::new();
        buf.push_str(HTTP);
        buf.push_str();
        // TODO:: Доделать отправку запроса
//        buf.push_str(CRLF);
//        for h in self.headers {
//            let h_value = match &h {
//                &Headers::ContentLength(ref v) => &v,
//                &Headers::ContentType(ref v) => &v,
//                &Headers::Date(ref v) => &v,
//                &Headers::Connection(ref v) => &v,
//                &Headers::Server(_) => "RST",
//            };
//            buf.push_str(format!("{}{}", h.get_message().unwrap(),  &&h_value).as_str());
//            buf.push_str(CRLF);
//        }
//        buf.push_str(CRLF);
//        stream.write(buf.as_bytes()).unwrap();
//        match self.file {
//            Some(mut f) => {
////                buf.push_str(CRLF);
//                let mut buf = [0; 1024 * 1024];
//                let mut n: u64 = 0;
//                loop {
//                    match f.read(&mut buf).unwrap() {
//                        0 => { break; }
//                        i => {
//                            n += i as u64;
////                println!("{}", i);
//                            stream.write(&buf[..i]).unwrap();
//                            f.seek(SeekFrom::Start(n as u64));
//                        }
//                    }
//                }
//            }
//            None => {}
//        }
//        stream.flush().unwrap();
    }
}
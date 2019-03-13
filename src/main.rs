use std::net::TcpListener;
use highload_server::server::server::Server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let server = Server::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        Server::handle_request(server.config.document_root.clone().as_str(), stream);
    }
}

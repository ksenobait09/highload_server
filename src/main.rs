use highload_server::server::server::Server;

fn main() {
    let server = Server::new();
    server.run()
}

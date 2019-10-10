pub mod models;
pub mod persistence;
pub mod server;
pub mod http;

fn main() {
    server::start_server();
}

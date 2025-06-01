use crate::server::server_connect::{connect, logging, disconnect};

pub fn connect_helper() {
    println!("Conectando no servidor pelo helper");
    connect();
    logging();
    disconnect();
}
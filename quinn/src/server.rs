extern crate env_logger;
extern crate quinn;
extern crate rustls;
extern crate tokio;

use rustls::internal::pemfile;
use std::{fs::File, io::BufReader};

fn main() {

    let psk : Vec<u8> = vec![
        0x11, 0x11, 0x11, 0x11,
        0x22, 0x22, 0x22, 0x22,
        0x33, 0x33, 0x33, 0x33,
        0x44, 0x44, 0x44, 0x44,
        0x55, 0x55, 0x55, 0x55,
        0x66, 0x66, 0x66, 0x66,
        0x77, 0x77, 0x77, 0x77,
        0x88, 0x88, 0x88, 0x88
    ];

    let key = {
        let f = File::open("server.key").expect("cannot open 'server.key'");
        let mut reader = BufReader::new(f);
        pemfile::rsa_private_keys(&mut reader).expect("cannot read private keys")
    };

    let tls_config = quinn::tls::build_server_config_psk(
        key[0].clone(),
        psk
    ).unwrap();

    env_logger::init();
    tokio::run(quinn::Server::new("0.0.0.0", 4433, tls_config).unwrap());
}

extern crate env_logger;
extern crate quinn;
extern crate rustls;
extern crate tokio;

use rustls::internal::pemfile;
use std::{fs::File, io::BufReader};
use std::rc::Rc;

fn main() {

    let key = {
        let f = File::open("server.rsa").expect("cannot open 'server.key'");
        let mut reader = BufReader::new(f);
        pemfile::rsa_private_keys(&mut reader).expect("cannot read private keys")
    };

    assert!(key.len() > 0);

    let certs = {
        let f = File::open("server.chain").expect("cannot open 'server.crt'");
        let mut reader = BufReader::new(f);
        pemfile::certs(&mut reader).expect("cannot read certificates")
    };

    let tls_config = quinn::tls::build_server_config_psk(
        certs,
        key[0].clone(),
    ).unwrap();

    /*
    // dirty!

    let rct = Rc::new(tls_config);
    let rft = unsafe {
        &(*Rc::into_raw(rct))
    };
    */

    env_logger::init();

    let server = quinn::Server::new("0.0.0.0", 4433, tls_config).unwrap();
    tokio::run(server);
}


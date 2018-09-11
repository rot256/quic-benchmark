extern crate futures;
extern crate quinn;

use futures::Future;
use std::env;

// const ITERATIONS : usize = 100000;
const ITERATIONS : usize = 10000;

fn main() {

    let server = env::args().nth(1).expect("need server name as an argument");
    let config = quinn::tls::build_client_config_psk();

    let mut i = 0;

    while i < ITERATIONS {

        println!("{}", i);

        let res = quinn::Client::connect_with_tls_config(&server, 4433, config.clone())
            .unwrap()
            .and_then(|_| {
                println!("client is connected");
                futures::future::ok(())
            })
            .wait();


        i += match res {
            Ok(_)   => 1,
            Err(_)  => {
                0
            }
        };
    }
}

extern crate futures;
extern crate quinn;

use futures::Future;
use std::env;

use std::time::Duration;
use std::thread;


// const ITERATIONS : usize = 100000;
const ITERATIONS : usize = 2;

fn main() {

    let server = env::args().nth(1).expect("need server name as an argument");
    let config = quinn::tls::build_client_config_psk();

    let mut i = 0;

    while i < ITERATIONS {

        println!("{}", i);

        let res = quinn::Client::connect_with_tls_config(&server, 4433, &config)
            .unwrap()
            .and_then(|conn| {
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

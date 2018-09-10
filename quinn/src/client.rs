extern crate futures;
extern crate quinn;

use futures::Future;
use std::env;

const ITERATIONS : usize = 100000;

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

    let server = env::args().nth(1).expect("need server name as an argument");

    let mut i = 0;

    while i < ITERATIONS {

        println!("{}", i);

        let config = quinn::tls::build_client_config_psk(psk.clone());

        let res = quinn::Client::connect_with_tls_config(&server, 4433, config)
            .unwrap()
            .and_then(|_| {
                println!("client is connected");
                futures::future::ok(())
            })
            .wait();


        i += match res {
            Ok(_)   => 1,
            Err(_)  => {
                println!("{:?}", res);
                0
            }
        };
    }
}

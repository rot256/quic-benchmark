extern crate futures;
extern crate ninn;
extern crate tokio;
extern crate untrusted;
extern crate tokio_timer;

use std::env;
use futures::*;

const ITERATIONS : usize = 100000;

fn main() {
    let server_pk = [0x33, 0x2b, 0x2f, 0x56, 0xbb, 0x4e, 0x28, 0x4a, 0x2e, 0x87, 0xe7, 0x69, 0x0d, 0x51, 0xf1, 0x29, 0x14, 0xa5, 0x9b, 0x3b, 0x8e, 0x03, 0x56, 0xd8, 0x23, 0xe0, 0x32, 0x61, 0x0a, 0xfd, 0xd6, 0x61];

    assert_eq!(server_pk.len(), 32);

    let server = env::args().nth(1).expect("need server name as an argument");
    let mut i = 0;

    while i < ITERATIONS {
        println!("{}", i);
        let res = ninn::Client::connect(&server, 8888, server_pk, None)
                .unwrap()
                .and_then(|_conn| {
                    Ok(())
                }).wait();
        i += match res {
            Ok(_) => 1,
            _     => 0,
        };
    }
}

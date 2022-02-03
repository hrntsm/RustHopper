extern crate base64;

use std::fs::File;
use std::io::Read;
use base64::{decode, encode};

fn main() {
    let mut file = File::open("definitions/sum.gh").unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);

    let encoded = encode(&buf);
    println!("byte\n{:?}", buf);
    println!("base64\n{:?}", encoded);
}

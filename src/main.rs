#![allow(non_snake_case)]
mod helpers;
use helpers::tencryptimage;
use helpers::imagedecrypttext;

fn main() {
    tencryptimage("ENCRYPTED TEXT", "picture.png", "encrypted.png");
    println!("DECRYPTED: {}", imagedecrypttext("encrypted.png"));
}


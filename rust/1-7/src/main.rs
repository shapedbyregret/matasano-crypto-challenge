extern crate serialize;
extern crate openssl;

use std::io::File;
use openssl::crypto::symm;


// Base64 decoding
fn unb64(b64_str:&str) -> Vec<u8> {
    use serialize::base64::{FromBase64};
    
    let byte_str = b64_str.from_base64().unwrap();
    return byte_str;
}

fn main() {
    // Decrypt AES-128 ECB
    let key = "YELLOW SUBMARINE";

    // Load file into memory
    let mut file = File::open(&Path::new("7.txt")).read_to_string().unwrap();
    let mut file_bytes:Vec<u8> = unb64(file.as_slice());

    let decoder = symm::Crypter::new(symm::AES_128_ECB);
    decoder.init(symm::Decrypt, key.as_bytes(), Vec::new());
    let answer = decoder.update(file_bytes.as_mut_slice());
    let ascii_answer:String = answer.iter().map(|x| *x as char).collect();
    println!("{}", ascii_answer);

}

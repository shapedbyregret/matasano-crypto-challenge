extern crate serialize;

use std::str;

fn hex2b64(hexString:&str) -> String {
  use serialize::base64::{ToBase64, MIME};
  use serialize::hex::{FromHex};
  let config = MIME;

  let byteVec:Vec<u8> = hexString.from_hex().unwrap();
  let asciiStr = str::from_utf8(byteVec.as_slice());
  println!("{}", asciiStr);
  let b64Str = asciiStr.unwrap().as_bytes().to_base64(config);

  return b64Str;
}

fn main() {
  let hexString = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let answer = hex2b64(hexString);

  println!("{}", answer);
}
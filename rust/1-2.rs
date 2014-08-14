extern crate serialize;

use std::str;

fn unhexlify(hex_string:&str) -> String {
  use serialize::base64::{ToBase64, MIME};
  use serialize::hex::{FromHex};
  let config = MIME;

  let byteVec:Vec<u8> = hex_string.from_hex().unwrap();
  let asciiStr = str::from_utf8(byteVec.as_slice());
  println!("{}", asciiStr);
  return String::from_str(asciiStr.unwrap());
}

fn fixed_xor(str1:&str, str2:&str) -> String {
  // Conver the two strings to bytes and zip together.
  let unhex_str1 = unhexlify(str1);
  let unhex_str2 = unhexlify(str2);

  let zipped:Vec<(&u8, &u8)> = unhex_str1.as_bytes().iter().zip(
    unhex_str2.as_bytes().iter()
  ).collect();
  println!("{}", zipped);

  // Iterate over byte tuples and xor values
  let mut xored:Vec<int> = Vec::new();
  for tup in zipped.iter() {
    let x1 = *tup.val0() as int;
    let x2 = *tup.val1() as int;
    xored.push(x1 ^ x2);
  }
  println!("{}", xored);

  // Convert to chars and join into string

  //return str1.as_bytes() ^ str2.as_bytes();
  return String::new();
}

fn main() {
  // Strings provided by challenge.
  // The XOR combination of "str1" and "str2" should be "answer"
  let str1 = "1c0111001f010100061a024b53535009181c";
  let str2 = "686974207468652062756c6c277320657965";
  let answer = "746865206b696420646f6e277420706c6179";
  let myAnswer = fixed_xor(str1, str2);

  //assert!(answer == myAnswer);
  println!("{}", myAnswer);
}
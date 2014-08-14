extern crate serialize;

use std::str;

// Convert ascii string to hex string
fn hexlify(str1:String) -> String {
  use serialize::hex::{ToHex};

  return str1.as_slice().as_bytes().to_hex();
}

// Convert hex string to ascii string
fn unhexlify(hex_string:&str) -> String {
  use serialize::hex::{FromHex};

  let byteVec:Vec<u8> = hex_string.from_hex().unwrap();
  let asciiStr = str::from_utf8(byteVec.as_slice());
  println!("{}", asciiStr);
  return String::from_str(asciiStr.unwrap());
}

fn fixed_xor(str1:&str, str2:&str) -> String {
  // Unhexlify strings
  let unhex_str1 = unhexlify(str1);
  let unhex_str2 = unhexlify(str2);

  // Convert the two strings to bytes and zip together.
  let zipped:Vec<(&u8, &u8)> = unhex_str1.as_bytes().iter().zip(
    unhex_str2.as_bytes().iter()
  ).collect();

  // Iterate over byte tuples and xor values
  let mut xored:Vec<u8> = Vec::new();
  for tup in zipped.iter() {
    let x1 = *tup.val0() as u8;
    let x2 = *tup.val1() as u8;
    xored.push(x1 ^ x2);
  }

  // Convert to chars and join into string
  let asciiStr:String = xored.iter().map(|x| *x as char).collect();
  println!("{}", asciiStr);

  // Convert xored answer back into hex string
  return hexlify(asciiStr);
}

fn main() {
  // Strings provided by challenge.
  // The XOR combination of "str1" and "str2" should be "answer"
  let str1 = "1c0111001f010100061a024b53535009181c";
  let str2 = "686974207468652062756c6c277320657965";
  let answer = "746865206b696420646f6e277420706c6179";
  let myAnswer = fixed_xor(str1, str2);

  println!("{}", myAnswer);
  assert!(answer == myAnswer.as_slice());
}

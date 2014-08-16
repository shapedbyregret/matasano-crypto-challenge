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
  //println!("{}", asciiStr);
  return String::from_str(asciiStr.unwrap());
}

fn single_xor(str1:&str, char1:u8) -> String {
  let unhex_str1 = unhexlify(str1);
  let xored:Vec<u8> = unhex_str1.as_bytes().iter().map(|c| c ^ char1).collect();
  let asciiStr:String = xored.iter().map(|x| *x as char).collect();
  //println!("{}", asciiStr);
  //return hexlify(asciiStr);
  return asciiStr;
}

// Find the likelihood of a string being english using bigrams
// Bigram Frequency
// http://en.wikipedia.org/wiki/Bigram#Bigram_Frequency_in_the_English_language
fn score_string(str1:String) -> int {
  let bigrams = ["th", "he", "in", "er", "an", "re", "nd", "at", "on", "nt",
                 "ha", "es", "st", "en", "ed", "to", "it", "ou", "ea", "hi"];
  let mut score:int = 0;
  for bigram in bigrams.iter() {
    if (str1.as_slice().contains(*bigram)) {
      score = score + 1;
    }
  }
  return score;
}

fn main() {
  // Find a single character to decode the given string. Score the returned
  // string to see if it's the likely answer.
  // Strings provided by challenge.
  let str1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

  let mut max_score = 0;
  let mut min_key:char = 0 as char;
  let mut min_decoded:String = String::new();

  // Iterate over ascii table in search of a key
  for n in range(33 as u8, 127) {
    let decoded = box single_xor(str1, n);
    //println!("{}", decoded);
    let score = score_string(decoded.to_owned());
    //println!("{}", score);
    if (score > max_score) {
      max_score = score;
      min_key = n as char;
      min_decoded = decoded.to_owned();
    }
  }

  println!("{}", max_score);
  println!("{}", min_key);
  println!("{}", min_decoded);
}

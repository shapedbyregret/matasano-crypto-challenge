extern crate serialize;

use std::io::BufferedReader;
use std::io::File;
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
    return String::from_str(asciiStr.unwrap_or(""));
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
        if str1.as_slice().contains(*bigram) {
            score = score + 1;
        }
    }
    return score;
}

fn main() {
    // Find line in file that is single-character xored
    let mut max_score:int = 0;
    let mut cnt:int = 0;
    let mut min_key:char = 0 as char;
    let mut min_decoded:String = String::new();
    let mut orig_line:String = String::new();
    let mut orig_cnt:int = 0;

    let path = Path::new("4.txt");
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines() {
        // Iterate over ascii table in search of a key
        let str1 = line.unwrap_or(String::new());
        for n in range(33 as u8, 127) {
            let decoded = box single_xor(str1.as_slice(), n);
            //println!("{}", decoded);
            let score = score_string(decoded.to_string());
            //println!("{}", score);
            if score > max_score {
                orig_line = str1.to_string();
                orig_cnt = cnt;
                max_score = score;
                min_key = n as char;
                min_decoded = decoded.to_string();
            }
        }
        cnt = cnt + 1;
    }
    
    println!("String: {}", orig_line);
    println!("Line: {}", orig_cnt);
    println!("Score: {}", max_score);
    println!("Key: {}", min_key);
    println!("Decoded: {}", min_decoded);
}

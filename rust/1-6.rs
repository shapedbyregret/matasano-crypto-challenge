extern crate serialize;

use std::collections::{BitvSet, Bitv, bitv};
use std::io::BufferedReader;
use std::io::File;
use std::str;

// Convert ascii string to hex string
fn hexlify(str1:String) -> String {
    use serialize::hex::{ToHex};

    return str1.as_slice().as_bytes().to_hex();
}

fn repeating_xor(str1:&str, key:&str) -> String {
    let mut xored:Vec<u8> = Vec::new();
    let mut cnt:uint = 0;
    for x in str1.as_bytes().iter() {
        xored.push(x ^ key.char_at(cnt) as u8);
        cnt = cnt + 1;
        if cnt >= key.char_len() {
            cnt = 0;
        }
    }
    let asciiStr:String = xored.iter().map(|x| *x as char).collect();
    return hexlify(asciiStr);
}

// Hamming distance is the number of differing *bits*.
fn edit_distance(str1:&str, str2:&str) -> int {
    let mut diff:int = 0;
    let str1bits:Bitv = bitv::from_bytes(str1.as_bytes());
    let str2bits:Bitv = bitv::from_bytes(str2.as_bytes());
    for n in range(0, str1bits.len()) {
        if str1bits.get(n) != str2bits.get(n) {
            diff = diff + 1;
        }
    }
    return diff;
}

fn main() {
    // Break repeating key XOR. Input file is base64.

    // Test edit distance
    let test_edit = edit_distance("this is a test", "wokka wokka!!!");
    println!("Differing Bits: {}", test_edit);
    assert_eq!(37, test_edit);

    let path = Path::new("6.txt");
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines() {
        let str1 = line.unwrap_or(String::new());
    }
}

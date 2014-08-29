extern crate serialize;

use std::collections::{BitvSet, Bitv, bitv, HashMap, Map};
use std::io::BufferedReader;
use std::io::File;
use std::str;

// Convert ascii string to hex string
fn hexlify(ascii_str:String) -> String {
    use serialize::hex::{ToHex};

    return ascii_str.as_slice().as_bytes().to_hex();
}

fn unhexlify(hex_str:&str) -> String {
    use serialize::hex::{FromHex};

    let byteVec:Vec<u8> = hex_str.from_hex().unwrap();
    let ascii_str = str::from_utf8(byteVec.as_slice());
    //println!("{}", asciiStr);
    return String::from_str(ascii_str.unwrap_or(""));
}

fn unb64(b64_str:&str) -> String {
    use serialize::base64::{FromBase64};
    
    let ascii_str:String = String::from_utf8(b64_str.from_base64().unwrap()).unwrap();
    return ascii_str;
}

fn single_xor(str1:&str, char1:u8) -> String {
    //let unhex_str1 = unhexlify(str1);
    let xored:Vec<u8> = str1.as_bytes().iter().map(|c| c ^ char1).collect();
    let asciiStr:String = xored.iter().map(|x| *x as char).collect();
    //println!("{}", asciiStr);
    //return hexlify(asciiStr);
    return asciiStr;
}

fn score_string(str1:String) -> int {
    let bigrams = ["th", "he", "in", "er", "an", "re", "nd", "at", "on", "nt",
                   "ha", "es", "st", "en", "ed", "to", "it", "ou", "ea", "hi"];
    let mut score:int = 0;
    for bigram in bigrams.iter() {
        if str1.as_slice().contains(*bigram) {
            score += 1;
        }
    }
    return score;
}

fn score_string_2(str1:String) -> int {
    //let ETAOIN = "ETAOINSHRDLCUMWFGYPBVKJXQZ";
    let mut freqs = HashMap::new();
    let mut score = 0i;
    freqs.insert("E".to_string(), 12i);
    freqs.insert("T".to_string(), 9i);
    freqs.insert("A".to_string(), 8i);
    freqs.insert("O".to_string(), 8i);
    freqs.insert("I".to_string(), 7i);
    freqs.insert("N".to_string(), 7i);

    for c in str1.as_slice().chars() {
        let c_str = c.to_uppercase().to_string();
        if freqs.contains_key(&c_str) {
            score += *freqs.get_mut(&c_str);
        }
    }

    return score;
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
    return asciiStr;
}

// Hamming distance is the number of differing *bits*.
fn edit_distance(str1:&[u8], str2:&[u8]) -> int {
    let mut diff:int = 0;
    let str1bits:Bitv = bitv::from_bytes(str1);
    let str2bits:Bitv = bitv::from_bytes(str2);
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
    let test_edit = edit_distance("this is a test".as_bytes(),
                                  "wokka wokka!!!".as_bytes());
    println!("Differing Bits: {}", test_edit);
    assert_eq!(37, test_edit);

    // Load file into memory
    let mut file = File::open(&Path::new("6.txt")).read_to_string().unwrap();
    file = unb64(file.as_slice());
    //println!("{}", file);

    // Guess keysize
    let mut key_len:uint = 2 as uint;
    let mut min_dist:f32 = 9999.0;
    for n in range(2, 40) {
        let mut i = 0;
        let mut norm = 0f32;
        // Use average of 20 normalized edit distances.
        loop {
            let sliceA = file.as_bytes().slice(n*i, n*(i+1));
            let sliceB = file.as_bytes().slice(n*(i+1), n*(i+2));
            norm += edit_distance(sliceA, sliceB) as f32 / n as f32;
            i += 2;
            if i > 40 {
                break;
            }
        }
        norm /= 20f32;
        if norm < min_dist {
            min_dist = norm;
            key_len = n;
        }
    }
    println!("Guessed Key Length: {}", key_len);

    // Transpose file into n(key length) number of blocks
    let mut blocks = Vec::from_elem(key_len as uint, "".to_string());
    let mut cnt:uint = 0 as uint;
    for c in file.as_slice().chars() {
        *blocks.get_mut(cnt) = blocks[cnt].clone().append(c.to_string().as_slice());
        cnt = cnt + 1;
        if cnt >= key_len {
            cnt = 0;
        }
    }
    //println!("Blocks: {}", blocks);
    
    let mut scores = Vec::from_elem(key_len, 0i);
    let mut key = Vec::from_elem(key_len as uint, "".to_string());
    let mut decoded_blocks = Vec::from_elem(key_len as uint, "".to_string());
    for n in range(32 as u8, 127) {
        // Decode each block
        for i in range(0, key_len) {
            *decoded_blocks.get_mut(i) = single_xor(blocks[i].as_slice(), n);
            let score = score_string_2(decoded_blocks[i].clone());
            if score > scores[i] {
                *scores.get_mut(i) = score;
                *key.get_mut(i) = (n as char).to_string();
            }
        }
    }
    println!("{}", key.concat());

    let decoded = repeating_xor(file.as_slice(), key.concat().as_slice());
    //println!("{}", decoded);
}

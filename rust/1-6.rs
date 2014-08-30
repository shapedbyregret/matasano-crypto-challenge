extern crate serialize;

use std::collections::{Bitv, bitv, HashMap, Map};
use std::io::File;
use std::iter::AdditiveIterator;
use std::num;


// Base64 decoding
fn unb64(b64_str:&str) -> String {
    use serialize::base64::{FromBase64};
    
    let ascii_str:String = String::from_utf8(b64_str.from_base64().unwrap()).unwrap();
    return ascii_str;
}

// Solve single key xored string
fn single_xor(str1:&str, char1:u8) -> String {
    //let unhex_str1 = unhexlify(str1);
    let xored:Vec<u8> = str1.as_bytes().iter().map(|c| c ^ char1).collect();
    let asciiStr:String = xored.iter().map(|x| *x as char).collect();
    //println!("{}", asciiStr);
    //return hexlify(asciiStr);
    return asciiStr;
}

// Letter frequency score
fn score_string_2(str1:String) -> f32 {
    //let ETAOIN = "ETAOINSHRDLCUMWFGYPBVKJXQZ";
    let mut score;
    let mut pre_freqs:HashMap<String, f32> = HashMap::new();
    pre_freqs.insert("e".to_string(), 12.02);
    pre_freqs.insert("t".to_string(), 9.10);
    pre_freqs.insert("a".to_string(), 8.12);
    pre_freqs.insert("o".to_string(), 7.68);
    pre_freqs.insert("i".to_string(), 7.31);
    pre_freqs.insert("n".to_string(), 6.95);
    pre_freqs.insert("s".to_string(), 6.28);
    pre_freqs.insert("r".to_string(), 6.02);
    pre_freqs.insert("h".to_string(), 5.92);
    pre_freqs.insert("d".to_string(), 4.32);
    pre_freqs.insert("l".to_string(), 3.98);
    pre_freqs.insert("u".to_string(), 2.88);
    pre_freqs.insert("c".to_string(), 2.71);
    pre_freqs.insert("m".to_string(), 2.61);
    pre_freqs.insert("f".to_string(), 2.3);

    let mut freqs:HashMap<String, f32> = pre_freqs.clone();
    for (key, val) in freqs.mut_iter() {
        *val = 0.0;
    }

    // Get frequency of each letter
    for c in str1.as_slice().chars() {
        let c_str = c.to_string();
        if freqs.contains_key(&c_str) {
            *freqs.get_mut(&c_str) += 1.0;
        }
    }

    // Get frequency as percentage
    let str1_len = str1.len() as f32;
    for (key, val) in freqs.mut_iter() {
        *val /= str1_len;
        *val *= 100.0;
    }

    // Compare frequencies
    let mut cmps:Vec<f32> = Vec::new();
    for (key, val) in pre_freqs.iter() {
        if freqs.contains_key(key) {
            cmps.push(num::abs( freqs[*key] - *val ));
        }
    }
    
    // Get score
    if cmps.len() > 0 {
        score = cmps.iter().map(|v| *v).sum();
    } else {
        score = 9999.0;
    }
    
    //println!("{}", score);

    return score;
}

// Solve repeating key xored string
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

// Guess key length
fn guess_key_length(str1:&str) -> uint {
    let mut key_len:uint = 2 as uint;
    let mut min_dist:f32 = 9999.0;
    for n in range(2, 40) {
        let mut i = 0;
        let mut norm = 0f32;
        // Use average of 20 normalized edit distances.
        loop {
            let sliceA = str1.as_bytes().slice(n*i, n*(i+1));
            let sliceB = str1.as_bytes().slice(n*(i+1), n*(i+2));
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
    return key_len;
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

    // Guess keysize
    let key_len = guess_key_length(file.as_slice());
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
    
    // Xor each block and score to guess at key
    let mut scores = Vec::from_elem(key_len, 9999.0);
    let mut key = Vec::from_elem(key_len as uint, "".to_string());
    let mut decoded_blocks = Vec::from_elem(key_len as uint, "".to_string());
    for n in range(32 as u8, 127) {
        // Decode each block
        for i in range(0, key_len) {
            *decoded_blocks.get_mut(i) = single_xor(blocks[i].as_slice(), n);
            let score = score_string_2(decoded_blocks[i].clone());
            // Lower score is better.
            // Save lowest scores and corresponding char
            if score < scores[i] {
                *scores.get_mut(i) = score;
                *key.get_mut(i) = (n as char).to_string();
            }
            
        }
        
    }
    println!("Guessed Key: {}", key.concat());

    // Use repeating key xor to see if guessed key outputs sensible text.
    let decoded = repeating_xor(file.as_slice(), key.concat().as_slice());
    println!("{}", decoded);
}

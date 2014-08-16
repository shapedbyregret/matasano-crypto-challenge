extern crate serialize;

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

fn main() {
    // Implement repeating-key XOR using "ICE"
    let str1 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let answer = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let key = "ICE";
    let xored = repeating_xor(str1, key);
    println!("{}", xored);
    assert_eq!(xored.as_slice(), answer);

    // Read entire file as string
    // Ensure you are using unix line endings and not windows line endings
    /*
    let file = File::open(&Path::new("5.txt")).read_to_string();
    let xored2 = repeating_xor(file.unwrap().as_slice(), key);
    println!("{}", xored2);
    assert_eq!(xored2, answer);
    */
}

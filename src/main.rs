mod formatters;
mod detector;
use formatters::{hex_to_bytes,bytes_to_hex};
use detector::find_best_english_string;
use std::io::{BufReader,BufRead};
use std::fs::File;


fn fixed_xor(buffer1: &[u8], buffer2:&[u8]) -> Vec<u8>{
	buffer1.iter().zip(buffer2.iter()).map(|(x,y)| x ^ y).collect()
}

fn single_byte_xor(buffer: &[u8], key: u8) -> Vec<u8>{
	buffer.iter().map(|x| x ^ key).collect()
}
// 			let candidates: Vec<String> = (0..255).filter_map(|x| String::from_utf8(single_byte_xor(&buf1,x)).ok()).collect();

fn repeating_key_xor(buffer: &[u8], key: &str) -> Vec<u8>{
	let repeating_key =std::iter::repeat(key).take(buffer.len()).collect::<String>();
	buffer.iter().zip(repeating_key.bytes()).map(|(x,y)| x ^ y).collect()
}

fn main() {
	let msg = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal".to_vec();
	let encrypted = bytes_to_hex(&repeating_key_xor(&msg,"ICE"));
	let decrypted = repeating_key_xor(&hex_to_bytes(&encrypted),"ICE");
    println!("{:?}", String::from_utf8(decrypted));
}


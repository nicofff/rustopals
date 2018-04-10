mod formatters;
mod detector;
use formatters::{hex_to_bytes,bytes_to_hex,b64_to_bytes};
use detector::find_best_english_string;
use std::io::prelude::*;
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

fn hamming_distance(buf1: &[u8],buf2: &[u8]) -> usize{
	assert!(buf1.len() == buf2.len());
	buf1.iter().zip(buf2.iter()).map(|(x,y)| x ^ y).fold(0,|acc,x| acc + x.count_ones() as usize)
}

fn get_rxor_key_size(buffer: &[u8]) -> usize {
	let mut min_distance: f32 = 999999.0;
	let mut min_key_size = 0;
	for key_size in 2..64{
		let chunks: Vec<&[u8]> = buffer.chunks(key_size).filter(|x| x.len() == key_size).collect();
	    let distance: f32 = chunks[1..].iter().map(|x| hamming_distance(chunks[0],x) as f32 *1000.0 ).sum::<f32>() / (key_size * (chunks.len() -1)) as f32;
	    //println!("{:?} {:?}", key_size,distance );
	    if distance < min_distance {
	    	min_distance = distance;
	     	min_key_size = key_size;
	    }
	}
	min_key_size
}

fn split_in_nth(buffer: &[u8], n:usize) -> Vec<Vec<u8>> {
	let mut output: Vec<Vec<u8>> = Vec::new();
	for (ix,elem) in buffer.iter().enumerate(){
		if ix < n {
			output.push(Vec::new());
		}
		output[ix % n].push(*elem);
	}
	assert!(output.len()==n);
	output
}

fn break_repeating_xor(buf: &[u8])-> (String,String) {
	let key_size = get_rxor_key_size(buf);
    let mut key = String::new();
    for partial in split_in_nth(buf,key_size){
    	let candidates: Vec<String> = (0..255).filter_map(|x| String::from_utf8(single_byte_xor(&partial,x)).ok()).collect();
    	if let Some((_,_,ix)) = find_best_english_string(candidates){
    		key += &String::from_utf8([ix].to_vec()).unwrap();
    	}   	
    }
    let plaintext = String::from_utf8(repeating_key_xor(buf,&key)).unwrap();
    (key,plaintext)
}

fn main() {
	let mut file = File::open("src/6.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let msg = b64_to_bytes(&contents.replace('\n', ""));
    let (key,plaintext) = break_repeating_xor(&msg);
    println!("{:?}", key );
	println!("{}", plaintext);
}


// fn main() {
// 	let msg = b"Burning 'em, if you ain't quick and nimble
// I go crazy when I hear a cymbal".to_vec();
// 	let encrypted = bytes_to_hex(&repeating_key_xor(&msg,"ICE"));
// 	let decrypted = repeating_key_xor(&hex_to_bytes(&encrypted),"ICE");
//     println!("{:?}", String::from_utf8(decrypted));
// }


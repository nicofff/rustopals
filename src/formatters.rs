extern crate base64;

pub fn hex_to_bytes(buffer: &str) -> Vec<u8> {
    let input_chars: Vec<_> = buffer.chars().collect();

    input_chars.chunks(2).map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap();
        let second_byte = chunk[1].to_digit(16).unwrap();
        ((first_byte << 4) | second_byte) as u8
    }).collect()
}

pub fn bytes_to_hex(buffer: &[u8]) -> String {
	buffer.iter().map(|x| format!("{:0>2x}", x)).collect::<String>()
}

pub fn bytes_to_b64(buffer: &[u8]) -> String {
	base64::encode(&buffer)
}

pub fn b64_to_bytes(buffer: &str) -> Vec<u8> {
	base64::decode(&buffer).unwrap()
}

pub fn hex_to_b64(buffer: &str) -> String {
	bytes_to_b64(&hex_to_bytes(buffer))
}

pub fn b64_to_hex(buffer: &str) -> String {
	bytes_to_hex(&b64_to_bytes(buffer))
}


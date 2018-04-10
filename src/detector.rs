use std::str;
use std::collections::HashMap;
use std;
fn get_letter_score(c: char) -> f32 {
	match c {
		'a'=>7.52766,
		'e'=>7.0925,
		'o'=>5.17,
		'r'=>4.96032,
		'i'=>4.69732,
		's'=>4.61079,
		'n'=>4.56899,
		'1'=>4.35053,
		't'=>3.87388,
		'l'=>3.77728,
		'2'=>3.12312,
		'm'=>2.99913,
		'd'=>2.76401,
		'0'=>2.74381,
		'c'=>2.57276,
		'p'=>2.45578,
		'3'=>2.43339,
		'h'=>2.41319,
		'b'=>2.29145,
		'u'=>2.10191,
		'k'=>1.96828,
		'4'=>1.94265,
		'5'=>1.88577,
		'g'=>1.85331,
		'9'=>1.79558,
		'6'=>1.75647,
		'8'=>1.66225,
		'7'=>1.621,
		'y'=>1.52483,
		'f'=>1.2476,
		'w'=>1.24492,
		'j'=>0.836677,
		'v'=>0.833626,
		'z'=>0.632558,
		'x'=>0.573305,
		'q'=>0.346119,
		'A'=>0.130466,
		'S'=>0.108132,
		'E'=>0.0970865,
		'R'=>0.08476,
		'B'=>0.0806715,
		'T'=>0.0801223,
		'M'=>0.0782306,
		'L'=>0.0775594,
		'N'=>0.0748134,
		'P'=>0.073715,
		'O'=>0.0729217,
		'I'=>0.070908,
		'D'=>0.0698096,
		'C'=>0.0660872,
		'H'=>0.0544319,
		'G'=>0.0497332,
		'K'=>0.0460719,
		'F'=>0.0417393,
		'J'=>0.0363083,
		'U'=>0.0350268,
		'W'=>0.0320367,
		'.'=>0.0316706,
		'!'=>0.0306942,
		'Y'=>0.0255073,
		'*'=>0.0241648,
		'@'=>0.0238597,
		'V'=>0.0235546,
		'-'=>0.0197712,
		'Z'=>0.0170252,
		'Q'=>0.0147064,
		'X'=>0.0142182,
		'_'=>0.0122655,
		'$'=>0.00970255,
		'#'=>0.00854313,
		','=>0.00323418,
		'/'=>0.00311214,
		'+'=>0.00231885,
		'?'=>0.00207476,
		';'=>0.00207476,
		'^'=>0.00195272,
		'%'=>0.00170863,
		'~'=>0.00152556,
		'='=>0.00140351,
		'&'=>0.00134249,
		'`'=>0.00115942,
		'\\'=>0.00115942,
		')'=>0.00115942,
		']'=>0.0010984,
		'['=>0.0010984,
		':'=>0.000549201,
		'<'=>0.000427156,
		'('=>0.000427156,
		'>'=>0.000183067,
		'"'=>0.000183067,
		'|'=>0.000122045,
		'{'=>0.000122045,
		'\''=>0.000122045,
		'}'=>0.00006,
		_ => 999999999.0
	}
}

fn char_ocurrences(string: &str) -> HashMap<char,u32> {
	let mut counter = HashMap::new();
	for c in string.chars() {
		*counter.entry(c).or_insert(0) += 1;
	}	
	counter
}
fn score_english_string(string: &str) -> f32 {
	let ocurrences = char_ocurrences(string);
	let mut score = 0.0;
	let len = string.len() as f32;
	for (c, count) in &ocurrences {
		if c.is_whitespace() {
			continue;
		}
		//println!("{:?} {:?} {:?}",c,count,score );
		let freq = *count as f32 *100.0  / len;
		score += (freq - get_letter_score(*c)).powi(2);
	}
	score
}

pub fn find_best_english_string(candidates: Vec<String>) -> Option<(String,f32,u8)> {
	let mut min_score = std::f32::INFINITY;
	let mut best_guess: Option<(String,f32,u8)> = None;
	for (ix,candidate) in candidates.iter().enumerate(){
		let score = score_english_string(&candidate);
		//println!("{:?} {:?}",score,candidate );
		if score < min_score{
			//println!("{:?} {:?}",score,candidate );
			min_score = score;
			best_guess = Some((candidate.to_string(),score,ix as u8));
		}
	}
	best_guess
}
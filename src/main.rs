use std::env;
mod cvt;
use cvt::RomajiCvt;

fn convert(input: String) -> Option<String> {
    let converter = RomajiCvt::new();
    if input.len() == input.chars().filter(|c| c.is_ascii_alphabetic() || '\'' == *c).count() {
        converter.from_romaji(input)
    } else {
        converter.to_romaji(input)
    }
}
fn main() {
    let word = env::args().skip(1).next().expect("invalid input");
    println!("{}", convert(word).expect("invalid input"));
}

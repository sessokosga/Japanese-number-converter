mod japanese_number;
use japanese_number::JapaneseNumber;

fn main() {
    // let num = 2_1234_5678;
    let num = 1000_0000_0000_0000_0000;
    let result = JapaneseNumber::convert(num);
    println!("Result : {}  =>  {}  =>  {}  =>  {}",result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
}

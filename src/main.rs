mod japanese_number;
use japanese_number::JapaneseNumber;

fn main() {
    // let num = 2_0000_0000;
    let num = 1_0001;    
    let result = JapaneseNumber::convert(num);
    println!("{}  =>  {}  =>  {}  =>  {}",result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
}

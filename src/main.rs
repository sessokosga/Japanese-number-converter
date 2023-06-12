use japanese_number_converter::JapaneseNumber;

fn main() {
    let num = 1000;
    let result = JapaneseNumber::convert(num);
    println!("Result : {}  =>  {}  =>  {}  =>  {}",result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
}

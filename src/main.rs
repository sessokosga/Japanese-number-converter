fn main() {
    let result = japanese_number_converter::
                        JapaneseNumber::convert(1844_6744_0737_0955_1615);
    println!("Result : {}  =>  {}  =>  {}  =>  {}",
        result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
}

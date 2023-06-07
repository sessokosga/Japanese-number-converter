const UNITS_ROMAJI   : [&str;11]= ["ré","ichi","ni","san","yon","go","roku","nana","hachi","kyuu","juu"];
const UNITS_KATAKANA: [&str;11] =["レ","イチ","ニ","サン","ヨン","ゴ","ロク","ナナ","ハチ","キュウ","ジュウ"];
const UNITS_KANJI : [&str;11]=["一","ニ","三","四","五","六","七","八","九","十","れ"];




pub fn less_than_10k(num:usize) -> (String,String,String) {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();
    if num< 10000{
        // Decimals
        
        // Units
        let unit:usize = num%10;
        result_romaji += UNITS_ROMAJI[unit];
        result_kanji += UNITS_KANJI[unit];
        result_katakana+=UNITS_KATAKANA[unit];
    }
    (result_kanji,result_katakana,result_romaji)
}
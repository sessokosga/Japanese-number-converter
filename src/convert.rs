const UNITS_ROMAJI: [&str; 11] = [
    "ré", "ichi", "ni", "san", "yon", "go", "roku", "nana", "hachi", "kyuu", "juu",
];
const UNITS_KATAKANA: [&str; 11] = [
    "レ",
    "イチ",
    "ニ",
    "サン",
    "よん",
    "ゴ",
    "ロク",
    "ナナ",
    "ハチ",
    "キュウ",
    "ジュウ",
];
const UNITS_KANJI: [&str; 11] = [
    "れ", "一", "ニ", "三", "四", "五", "六", "七", "八", "九", "十",
];

pub fn less_than_11(number: usize) -> (String, String, String) {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();
    if number <= 10 {
        result_romaji += UNITS_ROMAJI[number];
        result_kanji += UNITS_KANJI[number];
        result_katakana += UNITS_KATAKANA[number];
    }

    (result_kanji, result_katakana, result_romaji)
}

pub fn less_than_100(number: usize) -> (String, String, String) {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();
    if number < 10000 {
        // Decimals
        if number < 100 && number > 10 {
            let decimal = number / 10;
            result_romaji += UNITS_ROMAJI[decimal];
            result_romaji += " ";
            result_romaji += UNITS_ROMAJI[10];
            result_romaji += " ";

            result_kanji += UNITS_KANJI[decimal];
            result_kanji += UNITS_KANJI[10];
            result_romaji += " ";

            result_katakana += UNITS_KATAKANA[decimal];
            result_katakana += " ";
            result_katakana += UNITS_KATAKANA[10];
            result_katakana += " ";

            // Units
            let left = number - decimal * 10;
            if left > 0 {
                let res = less_than_11(left);
                result_kanji += &res.0;
                result_katakana += &res.1;
                result_romaji += &res.2;
            }
        }
    }
    (result_kanji, result_katakana, result_romaji)
}

pub fn less_than_1000(number: usize) -> (String, String, String) {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    if number < 1000 {
        let hundred = number / 100;
        
        // Prevent ichi hyaku
        if hundred >1{
            result_romaji += UNITS_ROMAJI[hundred];
            result_romaji += " ";

            result_kanji += UNITS_KANJI[hundred];

            result_katakana += UNITS_KATAKANA[hundred];
            result_katakana += " ";
        }

        result_romaji += "hyaku";
        result_romaji += " ";

        
        result_kanji += "百";
        result_romaji += " ";

        
        result_katakana += "ヒャク";
        result_katakana += " ";

        let left = number - hundred * 100;
        if left > 0 {
            let res = less_than_100(number % 100);
            result_kanji += &res.0;
            result_katakana += &res.1;
            result_romaji += &res.2;
        }
    }

    (result_kanji, result_katakana, result_romaji)
}

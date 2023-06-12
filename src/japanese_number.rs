pub struct JapaneseNumber{
    arabiasuji:usize,
    kanji:String,
    romaji:String,
    katakana:String
}

impl JapaneseNumber {
    pub fn convert(number:usize) -> JapaneseNumber{
        convert(number)
    }
    pub fn arabiasuji(&self) -> usize {
        self.arabiasuji
    }
    pub fn kanji(&self) -> &String {
        &self.kanji
    }
    pub fn romaji(&self) -> &String {
        &self.romaji
    }
    pub fn katakana(&self) -> &String {
        &self.katakana
    }
    
}
const PACKS_ROMAJI: [&str; 5] = ["man", "oku", "chō", "kei", "gai"];
const PACKS_KANJI: [&str; 5] = ["万", "億", "兆", "京", "垓"];
const PACKS_KATAKANA: [&str; 5] = ["マン", "オク", "チョオ", "ケイ", "ガイ"];

const UNITS_ROMAJI: [&str; 11] = [
    "ré", "ichi", "ni", "san", "yon", "go", "roku", "nana", "hachi", "kyū", "jū",
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

fn less_than_11(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();
    if number <= 10 {
        result_romaji += UNITS_ROMAJI[number];
        result_kanji += UNITS_KANJI[number];
        result_katakana += UNITS_KATAKANA[number];
    }

    JapaneseNumber { arabiasuji: number, kanji: result_kanji, romaji: result_romaji, katakana: result_katakana }
}

fn less_than_100(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    // Decimals
    if number < 100 {
        let decimal = number / 10;

        // Prevent ichi jū
        if decimal > 0 {
            if decimal > 1 {
                result_romaji += UNITS_ROMAJI[decimal];
                result_romaji += " ";

                result_kanji += UNITS_KANJI[decimal];

                result_katakana += UNITS_KATAKANA[decimal];
                result_katakana += " ";
            }

            result_romaji += UNITS_ROMAJI[10];
            result_romaji += " ";

            result_kanji += UNITS_KANJI[10];
            result_romaji += " ";

            result_katakana += UNITS_KATAKANA[10];
            result_katakana += " ";
        }
        // Units
        let left = number - decimal * 10;
        if left > 0 {
            let res = less_than_11(left);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;
        }
    }

    JapaneseNumber { arabiasuji: number, kanji: result_kanji, romaji: result_romaji, katakana: result_katakana }
}

fn less_than_1000(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    if number < 1000 {
        let hundred = number / 100;
        // Prevent "hyaku jū" when got "10" as input
        if hundred > 0 {
            // Prevent "ichi hyaku"
            if hundred > 1 {
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
        }
        let left = number - hundred * 100;
        if left > 0 {
            let res = less_than_100(number % 100);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;
        }
    }

    JapaneseNumber { arabiasuji: number, kanji: result_kanji, romaji: result_romaji, katakana: result_katakana }
}

fn less_than_10k(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    if number < 10000 {
        let thousand = number / 1000;
        // Prevent "sen jū" when got "10" as input
        if thousand > 0 {
            // Prevent "ichi sen"
            if thousand > 1 {
                result_romaji += UNITS_ROMAJI[thousand];
                result_romaji += " ";

                result_kanji += UNITS_KANJI[thousand];

                result_katakana += UNITS_KATAKANA[thousand];
                result_katakana += " ";
            }

            result_romaji += "sen";
            result_romaji += " ";

            result_kanji += "千";
            result_romaji += " ";

            result_katakana += "セン";
            result_katakana += " ";
        }

        let left = number - thousand * 1000;
        if left > 0 {
            let res = less_than_1000(number % 1000);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;
        }
    }
    JapaneseNumber { arabiasuji: number, kanji: result_kanji, romaji: result_romaji, katakana: result_katakana }
}

fn convert(number: usize) -> JapaneseNumber {
    let mut result_katakana = String::new();
    let mut result_romaji = String::new();
    let mut result_kanji = String::new();

    let mut num = number;
    let sep = number / 10000;
    let mut i = 0;
    
    if sep < 1_0000_0000 {
        while num > 0 {
            let res = less_than_10k(num % 10000);
            result_kanji += &res.kanji;
            result_katakana += &res.katakana;
            result_romaji += &res.romaji;

            result_katakana += " ";
            result_romaji += " ";

            if i > 0 {
                result_kanji += PACKS_KANJI[i - 1];
                result_katakana += PACKS_KATAKANA[i - 1];
                result_romaji += PACKS_ROMAJI[i - 1];
            }

            i += 1;
            num = num / 10000;
        }
    }

    JapaneseNumber { arabiasuji: number, kanji: result_kanji, romaji: result_romaji, katakana: result_katakana }
}

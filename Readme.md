## Converts a number into japanese

This converter takes a number in input and return a result of type `JapaneseNumber`. This type has three properties:

-   `arabiasu` : the number given asinput
-   `kanji` : the number in kanji format
-   `katakana` : the number in katakana format
-   `romaji` : the number in roman format


### Example

```
let result = japanese_number_converter::
            JapaneseNumber::convert(100);
println!("Result : {}  =>  {}  =>  {}  =>  {}",
    result.arabiasuji(),result.kanji(),result.katakana(),result.romaji());
    // Result : 100  =>  百  =>  ヒャク   =>  hyaku  
```

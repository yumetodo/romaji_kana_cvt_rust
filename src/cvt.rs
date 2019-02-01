mod jis_x_4063_2000;
extern crate unicode_normalization;

// use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RomajiCvt {
    from_romaji_table: [HashMap<&'static str, &'static str>; 2],
    to_romaji_table: HashMap<&'static str, &'static str>,
    ctab: String,
    two_glyph_second_list: String
}
impl RomajiCvt {
    const VOWEL: &'static str = "aiueo";
    pub fn new() -> Self {
        Self {
            from_romaji_table: jis_x_4063_2000::make_from_romaji_table(),
            to_romaji_table: jis_x_4063_2000::make_to_romaji_table(),
            ctab: jis_x_4063_2000::make_ctab(),
            two_glyph_second_list: jis_x_4063_2000::make_two_glyph_second_list()
        }
    }

    fn is_consonant(&self, c: char) -> bool {
        self.ctab.contains(&[c][..])
    }
    fn front_2_chars_are_equal(s: &str) -> Option<bool> {
        let mut it = s.chars().take(2);
        let c1 = it.next()?;
        let c2 = it.next()?;
        Some(c1 == c2)
    }
    fn from_romaji_impl(&self, s: &str) -> Option<String> {
        match s.len() {
            3 | 4 => {
                if Self::front_2_chars_are_equal(s)? {
                    let c1 = s.chars().next()?;
                    if 'n' == c1 {
                        self.from_romaji_table[1].get(&s[2..]).map(|converted| 'ん'.to_string() + converted)
                    } else if self.is_consonant(c1) {
                        self.from_romaji_table[1].get(&s[1..]).map(|converted| 'っ'.to_string() + converted)
                    } else {
                        None
                    }
                } else {
                    self.from_romaji_table[1].get(s).map(|converted| converted.to_string())
                }
            },
            2 => {
                self.from_romaji_table[1].get(s).map(|converted| converted.to_string())
            },
            1 => {
                self.from_romaji_table[0].get(s).map(|converted| converted.to_string())
            }
            _ => {
                if Self::front_2_chars_are_equal(s)? {
                    let c1 = s.chars().next()?;
                    if 'n' == c1 {
                        self.from_romaji_impl(&s[2..]).map(|converted| ['ん'.to_string(), converted].concat())
                    } else if self.is_consonant(c1) {
                        self.from_romaji_impl(&s[1..]).map(|converted| ['っ'.to_string(), converted].concat())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
    pub fn from_romaji(&self, input: String) -> Option<String> {
        let mut re = String::with_capacity(input.len() * 2);
        let mut prev_i = 0;
        for (index, _) in input.match_indices(|c| Self::VOWEL.contains(&[c][..])) {
            let s = self.from_romaji_impl(&input[prev_i..index + 1])?;
            re += &s;
            prev_i = index + 1;
        }
        if prev_i != input.len() {
            let s = self.from_romaji_impl(&input[prev_i..])?;
            re += &s;
        }
        Some(re)
    }
    pub fn to_romaji(&self, input: String) -> Option<String> {
        //apply NFC uniform.
        //ex.) で(U+3066, U+3099) => で(U+3067)
        let s = input.nfc().collect::<String>();
        let mut re = String::with_capacity(input.len() * 3);
        let mut prev_c = '\0';
        let mut prev_is_sokuonn = false;
        for c in s.chars() {
            if '\0' == prev_c {
                prev_c = c;
                continue;
            }
            if 'っ' == prev_c {
                prev_c = c;
                prev_is_sokuonn = true;
                continue;
            }
            let (key, next_c) = if self.two_glyph_second_list.contains(&[c][..]) {
                (format!("{}{}", prev_c, c), '\0')
            } else {
                (prev_c.to_string(), c)
            };
            let key_str: &str = &key;
            let append = self.to_romaji_table.get(key_str)?;
            if prev_is_sokuonn {
                let double_char = append.chars().next()?;
                re.push(double_char);
                prev_is_sokuonn = false;
            }
            re += append;
            prev_c = next_c;
        }
        if '\0' != prev_c {
            let key: &str = &prev_c.to_string();
            re += self.to_romaji_table.get(key)?;
        }
        Some(re)
    }
}

mod test {
    #[test]
    fn from_romaji() {
        let cvt = super::RomajiCvt::new();
        assert_eq!(Some("ありきたり".to_string()), cvt.from_romaji("arikitari".to_string()));
        assert_eq!(Some("んなばかな".to_string()), cvt.from_romaji("nnnabakana".to_string()));
        assert_eq!(Some("なんてこったい".to_string()), cvt.from_romaji("nanntekottai".to_string()));
        assert_eq!(Some("しったこっちゃない".to_string()), cvt.from_romaji("sittakottyanai".to_string()));
    }
    #[test]
    fn to_romaji() {
        let cvt = super::RomajiCvt::new();
        assert_eq!(Some("arikitari".to_string()), cvt.to_romaji("ありきたり".to_string()));
        assert_eq!(Some("nnnabakana".to_string()), cvt.to_romaji("んなばかな".to_string()));
        assert_eq!(Some("nanntekottai".to_string()), cvt.to_romaji("なんてこったい".to_string()));
        assert_eq!(Some("sittakottyanai".to_string()), cvt.to_romaji("しったこっちゃない".to_string()));
    }
}

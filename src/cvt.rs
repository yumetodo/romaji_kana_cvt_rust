use std::collections::HashMap;
mod jis_x_4063_2000;

#[derive(Debug)]
pub struct RomajiCvt {
    from_romaji_table: [HashMap<&'static str, &'static str>; 2]
}
impl RomajiCvt {
    const VOWEL: &'static str = "aiueo";
    pub fn new() -> Self {
        Self {
            from_romaji_table: jis_x_4063_2000::make_from_romaji_table()
        }
    }

    fn is_consonant(c: char) -> bool {
        const CTAB: &str = "kstnhmyrwzjpbcgf";
        CTAB.contains(&[c][..])
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
                    } else if Self::is_consonant(c1) {
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
                    } else if Self::is_consonant(c1) {
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
        Some(input + "a")
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
}

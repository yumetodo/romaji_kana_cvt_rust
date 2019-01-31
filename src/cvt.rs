use std::collections::HashMap;
mod jis_x_4063_2000;

#[derive(Debug)]
pub struct RomajiCvt {
    from_romaji_table: [HashMap<&'static str, &'static str>; 2]
}
impl RomajiCvt {
    pub fn new() -> Self {
    }

    }
    pub fn from_romaji(&self, input: String) -> Option<String> {
    }
    pub fn to_romaji(&self, input: String) -> Option<String> {
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

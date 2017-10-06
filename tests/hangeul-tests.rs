
extern crate hangeul;

#[cfg(test)]
pub mod tests {
    use hangeul;
    use hangeul::Codeblock;
    use hangeul::Character;

    #[test]
    pub fn test_const() {
        assert_eq!(hangeul::ModernSyllable::START_POINT, 0xac00);
        assert_eq!(hangeul::ModernSyllable::END_POINT, 0xd7a3);

        assert_eq!(hangeul::Choseong::START_POINT, 0x1100);
        assert_eq!(hangeul::Choseong::END_POINT, 0x1112);

        assert_eq!(hangeul::Choseong::COUNT, 19);
        assert_eq!(hangeul::Jungseong::COUNT, 21);
        assert_eq!(hangeul::Jongseong::COUNT, 27);
    }

    #[test]
    pub fn test_syllable() {
        let input_str = "아희방맣희";
        let chars: Vec<char> = input_str.chars().collect();
        for (i, c) in input_str.chars().enumerate() {
            let esyl = hangeul::ModernSyllable::from_char(c).unwrap();
            let tesyl: &hangeul::Syllable = &esyl;
            println!("{:?} {:?}", tesyl.char(), esyl);

            let lsyl = hangeul::LazySyllable::from_char(c).unwrap();
            let tlsyl: &hangeul::Syllable = &lsyl;
            println!("{:?} {:?}", tlsyl.char(), lsyl);

            assert_eq!(tesyl.char().unwrap(), *chars.get(i).unwrap());
            assert_eq!(tlsyl.char().unwrap(), *chars.get(i).unwrap());
            assert_eq!(tesyl.char(), tlsyl.char());
            assert_eq!(tesyl.choseong(), tlsyl.choseong());
            assert_eq!(tesyl.jungseong(), tlsyl.jungseong());
            assert_eq!(tesyl.jongseong(), tlsyl.jongseong());
        }
    }
}

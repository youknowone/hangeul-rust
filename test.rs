
extern crate hangeul;

#[cfg(test)]
pub mod tests {
    use hangeul;

    #[test]
    pub fn test_const() {
        assert!(hangeul::syllable_start as u32 == 0xac00);
        assert!(hangeul::syllable_end as u32 == 0xd7a3);

        assert!(hangeul::jamo_initial_start as u32 == 0x1100);
        assert!(hangeul::jamo_initial_modern_end as u32 == 0x1112);
    }

    #[test]
    pub fn test_syllable() {
        let input_str = "아희방맣희";
        let chars: Vec<char> = input_str.chars().collect();
        for (i, c) in input_str.chars().enumerate() {
            let esyl = hangeul::Syllable::from_char(c).unwrap();
            let tesyl: &hangeul::SyllableTrait = &esyl;
            println!("{} {:?}", tesyl.char(), esyl);

            let lsyl = hangeul::LazySyllable::from_char(c).unwrap();
            let tlsyl: &hangeul::SyllableTrait = &lsyl;
            println!("{} {:?}", tlsyl.char(), lsyl);

            assert_eq!(tesyl.char(), *chars.get(i));
            assert_eq!(tlsyl.char(), *chars.get(i));
            assert_eq!(tesyl.char(), tlsyl.char());
            assert_eq!(tesyl.initial(), tlsyl.initial());
            assert_eq!(tesyl.peak(), tlsyl.peak());
            assert_eq!(tesyl.final(), tlsyl.final());
        }
    }
}

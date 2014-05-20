
extern crate hangeul;

#[cfg(test)]
pub mod tests {
    use hangeul;

    #[test]
    pub fn test_const() {
        assert_eq!(hangeul::syllable_start as u32, 0xac00);
        assert_eq!(hangeul::syllable_end as u32, 0xd7a3);

        assert_eq!(hangeul::jamo_initial_start as u32, 0x1100);
        assert_eq!(hangeul::jamo_initial_end as u32, 0x1112);

        assert_eq!(hangeul::initial_count, 19);
        assert_eq!(hangeul::peak_count, 21);
        assert_eq!(hangeul::final_count, 27);
    }

    #[test]
    pub fn test_syllable() {
        let input_str = "아희방맣희";
        let chars: Vec<char> = input_str.chars().collect();
        for (i, c) in input_str.chars().enumerate() {
            let esyl = hangeul::ConcreteSyllable::from_char(c).unwrap();
            let tesyl: &hangeul::Syllable = &esyl;
            println!("{} {:?}", tesyl.char(), esyl);

            let lsyl = hangeul::LazySyllable::from_char(c).unwrap();
            let tlsyl: &hangeul::Syllable = &lsyl;
            println!("{} {:?}", tlsyl.char(), lsyl);

            assert_eq!(tesyl.char().unwrap(), *chars.get(i));
            assert_eq!(tlsyl.char().unwrap(), *chars.get(i));
            assert_eq!(tesyl.char(), tlsyl.char());
            assert_eq!(tesyl.initial(), tlsyl.initial());
            assert_eq!(tesyl.peak(), tlsyl.peak());
            assert_eq!(tesyl.final0(), tlsyl.final0());
            assert_eq!(tesyl.final(), tlsyl.final());
            assert_eq!(tesyl.NFD(), tlsyl.NFD());
        }
    }
}

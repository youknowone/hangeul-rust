
mod hangeul;

pub fn test_const() {
    assert!(hangeul::hangeul::syllable_start as u32 == 0xac00);
    assert!(hangeul::hangeul::syllable_end as u32 == 0xd7a3);

    assert!(hangeul::hangeul::jamo_initial_start as u32 == 0x1100);
    assert!(hangeul::hangeul::jamo_initial_modern_end as u32 == 0x1112);
}

fn test_jamo() {
    use hangeul::hangeul::Hangeul;
    let input_str = "아희방맣희";
    for c in input_str.chars() {
        let hangeul: hangeul::hangeul::HangeulSyllable = hangeul::hangeul::Hangeul::from_char(c);
        println!("{:?}", hangeul);
        println!("{}{}{}", hangeul.initial_jamo().unwrap(), hangeul.vowel_jamo().unwrap(), hangeul.final_jamo().unwrap_or(' '));
    }
}

fn main() {
    use hangeul::hangeul::Hangeul;
    test_const();
    test_jamo();
}

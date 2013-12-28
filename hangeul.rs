
pub mod hangeul {
    use std::char::from_u32;

    pub static initial_count: uint = 19;
    pub static vowel_count: uint = 21;
    pub static final_count: uint = 27;
    pub static final0_count: uint = final_count + 1;

    pub static syllable_start: char = '가'; // U+AC00
    pub static syllable_end: char = '힣'; // U+D7A3

    pub static jamo_initial_start: char = 'ᄀ'; // U+1100
    pub static jamo_initial_modern_end: char = 'ᄒ'; // U+1112
    pub static jamo_initial_end: char = 'ᅞ'; // U+115E
    //static jamo_initial_filler: char = from_u32(0x115f).unwrap();
    //static jamo_vowel_filler: char = from_u32(0x1160).unwrap();
    pub static jamo_vowel_start: char = 'ᅡ';
    pub static jamo_vowel_modern_end: char = 'ᅵ'; // U+1175
    pub static jamo_vowel_end: char = 'ᆧ';
    pub static jamo_final_start: char = 'ᆨ'; // U+11A8
    pub static jamo_final_modern_end: char = 'ᇂ'; // U+11C2
    pub static jamo_final_end: char = 'ᇿ'; // U+11FF
    pub static jamo_start: char = jamo_initial_start;
    pub static jamo_end: char = jamo_final_end;

    pub static compat_modern_jamo_start: char = 'ㄱ'; // U+3131
    pub static compat_modern_jamo_end: char = 'ㅣ'; // U+3163
    pub static compat_ancient_jamo_start: char = 'ㅥ'; // U+3163
    pub static compat_ancient_jamo_end: char = 'ㆎ'; // U+318E
    pub static compat_jamo_start: char = compat_modern_jamo_start;
    pub static compat_jamo_end: char = compat_ancient_jamo_end;

    pub enum Modernity {
        Modern = 0,
        Ancient = 1,
    }

    pub enum Class {
        NonHangeul,
        Syllable,
        Jamo(Modernity),
        CompatibillityJamo(Modernity),
    }

    impl Class {
        fn of_char(data: char) -> Class {
            match data {
                syllable_start..syllable_end => {
                    return Syllable
                }
                jamo_start..jamo_end => {
                    match data {
                        'ᄀ'..'ᄒ' | 'ᅡ'..'ᅵ' | 'ᆨ'..'ᇂ' => {
                            return Jamo(Modern)
                        }
                        _ => {
                            return Jamo(Ancient)
                        }
                    }
                }
                compat_modern_jamo_start..compat_modern_jamo_end => {
                    return CompatibillityJamo(Modern)
                }
                compat_ancient_jamo_start..compat_ancient_jamo_end => {
                    return CompatibillityJamo(Ancient)
                }
                _ => {
                    return NonHangeul
                }
            }
        }
    }

    pub trait Hangeul {
        fn from_char(c: char) -> Self;
        fn class(&self) -> Class;
        fn is_modern(&self) -> bool;

        fn initial_code(&self) -> Option<u8>;
        fn vowel_code(&self) -> Option<u8>;
        fn final_code(&self) -> Option<u8>;

        fn _jamo(&self, code: Option<u8>, offset: char) -> Option<char> {
            match code {
                Some(value) => from_u32(offset as u32 + value as u32),
                None => None,
            }
        }

        fn initial_jamo(&self) -> Option<char> {
            let code = self.initial_code();
            let offset = jamo_initial_start;
            self._jamo(code, offset)
        }

        fn vowel_jamo(&self) -> Option<char> {
            let code = self.vowel_code();
            let offset = jamo_vowel_start;
            self._jamo(code, offset)
        }

        fn final_jamo(&self) -> Option<char> {
            let code = self.final_code();
            let offset = jamo_final_start;
            self._jamo(code, offset)
        }
    }

    pub struct HangeulSyllable {
        data: char,
        initial: u8,
        vowel: u8,
        final: u8,
    }

    impl Hangeul for HangeulSyllable {
        fn from_char(c: char) -> HangeulSyllable {
            let code = c as uint;
            let mut base = code - (syllable_start as uint);
            let final = (base % final0_count) as u8;
            base /= final0_count;
            let vowel = (base % vowel_count) as u8;
            base /= vowel_count;
            let initial = (base % initial_count) as u8;
            return HangeulSyllable { data: c, initial: initial, vowel: vowel, final: final };
        }

        fn class(&self) -> Class {
            return Syllable;
        }

        fn is_modern(&self) -> bool {
            return true;
        }

        fn initial_code(&self) -> Option<u8> {
            return Some(self.initial);
        }

        fn vowel_code(&self) -> Option<u8> {
            return Some(self.vowel);
        }

        fn final_code(&self) -> Option<u8> {
            return if self.final > 0 { Some(self.final - 1) } else { None };
        }
    }

    struct HangeulJamoInitial {
        data: char,
        initial: u8,
        is_modern: bool,
    }

    impl Hangeul for HangeulJamoInitial {
        fn from_char(c: char) -> HangeulJamoInitial {
            let code = c as uint;
            let offset = code - (jamo_initial_start as uint);
            let is_modern = offset <= initial_count;
            return HangeulJamoInitial { data: c, initial: offset as u8, is_modern: is_modern }
        }

        fn class(&self) -> Class {
            return Jamo(if self.is_modern { Modern } else { Ancient })
        }

        fn is_modern(&self) -> bool {
            return self.is_modern
        }

        fn initial_code(&self) -> Option<u8> {
            return Some(self.initial);
        }

        fn vowel_code(&self) -> Option<u8> { None }

        fn final_code(&self) -> Option<u8> { None }
    }
}

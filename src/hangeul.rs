#![crate_name="hangeul"]
#![crate_type="lib"]
//#![license="BSD simplified"]

/*! Tools to manipulate hangeul data in unicode.
 */
#[macro_use] extern crate enum_primitive;
extern crate num;
use ::num::FromPrimitive;


pub trait Character {
    fn from_char(c: char) -> Option<Self> where Self: Sized {
        Self::from_u32(c as u32)
    }
    fn from_u32(v: u32) -> Option<Self> where Self: Sized;
    fn char(&self) -> Option<char>;
}

/** Supports conversions between unicode <-> manipulable representation.

    There are supposed unicode code point for character type.

    - Syllable types are mapped to syllable ranges.
    - Jamo types are mapped to jamo ranges.
    - Compatitble jamo and ancient characters are out of this trait.
 */
pub trait Codeblock: Character {
    /// Unicode start char of code block
    const START_CHAR: char;
    /// Unicode end char of code block
    const END_CHAR: char;
    /// Unicode start point of code block
    const START_POINT: u32 = Self::START_CHAR as u32;
    /// Unicode end point of code block
    const END_POINT: u32 = Self::END_CHAR as u32;
    /// Total count of code block
    const COUNT: usize = (Self::END_POINT - Self::START_POINT + 1) as usize;
}

/** Supports unicode jamo ranges.
 */
fn _char_from_u32<T>(v: u32, base: u32) -> Option<T> where T: ::num::FromPrimitive {
    let opt: Option<T> = FromPrimitive::from_u32(v - base as u32);
    opt
}

/** Order of modern hangeul choseongs.
 */
enum_from_primitive! {
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Choseong {
    Giyeok = 0,
    SsangGiyeok = 1,
    Nieun = 2,
    Digeut = 3,
    SsangDigeut = 4,
    Rieul = 5,
    Mieum = 6,
    Bieup = 7,
    SsangBieup = 8,
    Siot = 9,
    SsangSiot = 10,
    Ieung = 11,
    Jieut = 12,
    SsangJieut = 13,
    Chieut = 14,
    Kieuk = 15,
    Tieut = 16,
    Pieup = 17,
    Hieut = 18,
}
}
pub const CHOSEONG_FILLER: char = '\u{115f}';

impl Codeblock for Choseong {
    const START_CHAR: char = '\u{1100}';
    const END_CHAR: char = '\u{1112}';
}

impl Character for Choseong {
    fn from_u32(v: u32) -> Option<Choseong> {
        _char_from_u32(v, Choseong::START_POINT)
    }

    fn char(&self) -> Option<char> {
        let value = *self as u32;
        std::char::from_u32(Choseong::START_POINT + value)
    }
}

/** Order of modern hangeul vowels.
 */
enum_from_primitive! {
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Jungseong {
    A = 0,
    Ae = 1,
    Ya = 2,
    Yae = 3,
    Eo = 4,
    E = 5,
    Yeo = 6,
    Ye = 7,
    O = 8,
    Wa = 9,
    Wae = 10,
    Oe = 11,
    Yo = 12,
    U = 13,
    Weo = 14,
    We = 15,
    Wi = 16,
    Yu = 17,
    Eu = 18,
    Ui = 19,
    I = 20,
}
}
/// Unicode filler character for jamo jungseong
pub const JUNGSEONG_FILLER: char = '\u{1160}';

impl Codeblock for Jungseong {
    const START_CHAR: char = '\u{1161}';
    const END_CHAR: char = '\u{1175}';
}

impl Character for Jungseong {
    fn from_u32(v: u32) -> Option<Jungseong> {
        _char_from_u32(v, Jungseong::START_POINT)
    }

    fn char(&self) -> Option<char> {
        let value = self.clone() as u32;
        std::char::from_u32(Jungseong::START_POINT + value)
    }
}

/** Order of modern hangeul jongseongs.
 */
enum_from_primitive! {
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Jongseong {
    Giyeok = 1,
    SsangGiyeok = 2,
    GiyeokSiot = 3,
    Nieun = 4,
    NieunJieut = 5,
    NieunHieut = 6,
    Digeut = 7,
    Rieul = 8,
    RieulGiyeok = 9,
    RieulMieum = 10,
    RieulBieup = 11,
    RieulSiot = 12,
    RieulTieut = 13,
    RieulPieup = 14,
    RieulHieut = 15,
    Mieum = 16,
    Bieup = 17,
    BieupSiot = 18,
    Siot = 19,
    SsangSiot = 20,
    Ieung = 21,
    Jieut = 22,
    Chieut = 23,
    Kieuk = 24,
    Tieut = 25,
    Pieup = 26,
    Hieut = 27,
}
}
pub const JONGSEONG_COUNT_WITH_EMPTY: usize = Jongseong::COUNT + 1;


impl Codeblock for Jongseong {
    const START_CHAR: char = '\u{11a8}';
    const END_CHAR: char = '\u{11c2}';
}

impl Character for Jongseong {
    fn from_u32(v: u32) -> Option<Jongseong> {
        _char_from_u32(v, (Jongseong::START_POINT) - 1)
    }

    fn char(&self) -> Option<char> {
        let value = self.clone() as u32;
        std::char::from_u32(Jongseong::START_POINT + value)
    }
}

pub enum Alphabet {
    Choseong(Choseong),
    Jungseong(Jungseong),
    Jongseong(Jongseong),
}

/** Supports syllable handling in alphabet level.
 */
pub trait Syllable: Character {
    /// choseong
    fn choseong(&self) -> Option<Choseong>;
    /// jungseong
    fn jungseong(&self) -> Option<Jungseong>;
    /// jongseong
    fn jongseong(&self) -> Option<Jongseong>;

}

/** Unicode syllable range equivalent representation.
 */
#[derive(Debug)]
pub struct ModernSyllable {
    choseong: Choseong,
    jungseong: Jungseong,
    jongseong: Option<Jongseong>,
}

impl ModernSyllable {
    pub fn choseong(&self) -> Choseong {
        self.choseong
    }

    pub fn jungseong(&self) -> Jungseong {
        self.jungseong
    }

    pub fn jongseong(&self) -> Option<Jongseong> {
        self.jongseong
    }
}

impl Codeblock for ModernSyllable {
    const START_CHAR: char = '\u{ac00}';
    const END_CHAR: char = '\u{d7a3}';
}

impl Character for ModernSyllable {
    fn from_u32(code: u32) -> Option<ModernSyllable> {
        let mut base = code - ModernSyllable::START_POINT;
        if base < ModernSyllable::COUNT as u32 {
            let jongseong: Option<Jongseong> = FromPrimitive::from_u32(base % JONGSEONG_COUNT_WITH_EMPTY as u32);
            base /= JONGSEONG_COUNT_WITH_EMPTY as u32;
            let jungseong: Jungseong = FromPrimitive::from_u32(base % Jungseong::COUNT as u32).unwrap();
            base /= Jungseong::COUNT as u32;
            let choseong: Choseong = FromPrimitive::from_u32(base % Choseong::COUNT as u32).unwrap();

            Some(ModernSyllable { choseong: choseong, jungseong: jungseong, jongseong: jongseong })
        } else {
            None
        }
    }

    fn char(&self) -> Option<char> {
        let jongseong_value = match self.jongseong {
            Some(jongseong) => jongseong as u32,
            None => 0
        };
        std::char::from_u32(ModernSyllable::START_CHAR as u32 + jongseong_value +
            JONGSEONG_COUNT_WITH_EMPTY as u32 * (self.jungseong as u32 + (Jungseong::COUNT as u32 * self.choseong as u32)))
    }
}

impl Syllable for ModernSyllable {
    fn choseong(&self) -> Option<Choseong> {
        Some(self.choseong())
    }

    fn jungseong(&self) -> Option<Jungseong> {
        Some(self.jungseong())
    }

    fn jongseong(&self) -> Option<Jongseong> {
        self.jongseong()
    }

}


/** Unicode syllable range equivalent representation, but calculates values dynamically.
 */
#[derive(Debug)]
pub struct LazySyllable {
    data: u32,
}

impl Character for LazySyllable {
    fn from_u32(code: u32) -> Option<LazySyllable> {
        Some(LazySyllable { data: code })
    }

    fn char(&self) -> Option<char> {
        std::char::from_u32(self.data)
    }
}

impl Syllable for LazySyllable {
    fn choseong(&self) -> Option<Choseong> {
        let code = (self.data - ModernSyllable::START_CHAR as u32) / (Jungseong::COUNT * JONGSEONG_COUNT_WITH_EMPTY) as u32;
        let char: Option<Choseong> = FromPrimitive::from_u32(code);
        char
    }

    fn jungseong(&self) -> Option<Jungseong> {
        let code = (self.data - ModernSyllable::START_CHAR as u32) / JONGSEONG_COUNT_WITH_EMPTY as u32 % Jungseong::COUNT as u32;
        let char: Option<Jungseong> = FromPrimitive::from_u32(code);
        char
    }

    fn jongseong(&self) -> Option<Jongseong> {
        let code = (self.data - ModernSyllable::START_CHAR as u32) % JONGSEONG_COUNT_WITH_EMPTY as u32;
        let char: Option<Jongseong> = FromPrimitive::from_u32(code);
        char
    }

}

/** Syllable builder with Syllable trait.
 */
pub struct SyllableBuilder {
    pub choseong: Option<Choseong>,
    pub jungseong: Option<Jungseong>,
    pub jongseong: Option<Jongseong>,
}

impl SyllableBuilder {
    pub fn new() -> SyllableBuilder {
        SyllableBuilder { choseong: None, jungseong: None, jongseong: None, }
    }

    pub fn from_alphabet(
            choseong: Option<Choseong>, jungseong: Option<Jungseong>, jongseong: Option<Jongseong>)
            -> SyllableBuilder {
        SyllableBuilder { choseong: choseong, jungseong: jungseong, jongseong: jongseong, }
    }
}

impl Character for SyllableBuilder {
    fn from_u32(code: u32) -> Option<SyllableBuilder> {
        let opt = ModernSyllable::from_u32(code);
        match opt {
            Some(syl) => {
                let syltr: &Syllable = &syl;
                Some(SyllableBuilder {
                    choseong: syltr.choseong(),
                    jungseong: syltr.jungseong(),
                    jongseong: syltr.jongseong(),
                })
            }
            None => None
        }
    }

    fn char(&self) -> Option<char> {
        match (self.choseong, self.jungseong) {
            (Some(choseong), Some(jungseong)) => {
                ModernSyllable { choseong: choseong, jungseong: jungseong, jongseong: self.jongseong }.char()
            },
            _ => None,
        }
    }
}

impl Syllable for SyllableBuilder {
    fn choseong(&self) -> Option<Choseong> {
        self.choseong
    }

    fn jungseong(&self) -> Option<Jungseong> {
        self.jungseong
    }

    fn jongseong(&self) -> Option<Jongseong> {
        self.jongseong
    }

}

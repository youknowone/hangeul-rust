#![crate_id="hangeul"]
#![crate_type="lib"]

pub static initial_count: uint = 19;
pub static vowel_count: uint = 21;
pub static final_count: uint = 27;
pub static final0_count: uint = final_count + 1;

pub static syllable_start: char = '가'; // U+AC00
pub static syllable_end: char = '힣'; // U+D7A3
pub static syllable_count: uint = syllable_end as uint - syllable_start as uint;

pub static jamo_initial_start: char = 'ᄀ'; // U+1100
pub static jamo_initial_modern_end: char = 'ᄒ'; // U+1112
pub static jamo_initial_end: char = 'ᅞ'; // U+115E
//static jamo_initial_filler: char = from_u32(0x115f).unwrap();
//static jamo_vowel_filler: char = from_u32(0x1160).unwrap();
pub static jamo_peak_start: char = 'ᅡ';
pub static jamo_peak_modern_end: char = 'ᅵ'; // U+1175
pub static jamo_peak_end: char = 'ᆧ';
pub static jamo_final0_start: char = 'ᆧ'; // U+11A7
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

#[experimental]
#[deriving(Eq)]
pub enum Modernity {
    Modern = 0,
    Ancient = 1,
}

#[experimental]
pub enum DataClass {
    NonHangeul,
    Syllable,
    SyllableNFD,
    Jamo(Modernity),
    CompatibillityJamo(Modernity),
}

impl DataClass {
    pub fn of_char(data: char) -> DataClass {
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

pub trait Conversion {
    fn from_char(c: char) -> Option<Self>;
    fn char(&self) -> char;
}

pub trait Jamo: FromPrimitive {
    fn from_char(c: char, base: char) -> Option<Self> {
        let opt: Option<Self> = FromPrimitive::from_u32(c as u32 - base as u32);
        opt
    }
}

#[deriving(Eq)]
#[deriving(FromPrimitive)]
#[deriving(Show)]
pub enum Initial {
    InitialGiyeok = 0,
    InitialSsangGiyeok = 1,
    InitialNieun = 2,
    InitialDigeut = 3,
    InitialSsangDigeut = 4,
    InitialRieul = 5,
    InitialMieum = 6,
    InitialBieup = 7,
    InitialSsangBieup = 8,
    InitialSiot = 9,
    InitialSsangSiot = 10,
    InitialIeung = 11,
    InitialJieut = 12,
    InitialSsangJieut = 13,
    InitialChieut = 14,
    InitialKieuk = 15,
    InitialTieut = 16,
    InitialPieup = 17,
    InitialHieut = 18,
}

impl Jamo for Initial { }

impl Conversion for Initial {
    fn from_char(c: char) -> Option<Initial> {
        Jamo::from_char(c, jamo_initial_start)
    }

    fn char(&self) -> char {
        std::char::from_u32(jamo_initial_start as u32 + *self as u32).unwrap()
    }
}

#[deriving(Eq)]
#[deriving(FromPrimitive)]
#[deriving(Show)]
pub enum Vowel {
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

impl Jamo for Vowel { }

impl Conversion for Vowel {
    fn from_char(c: char) -> Option<Vowel> {
        Jamo::from_char(c, jamo_peak_start)
    }

    fn char(&self) -> char {
        std::char::from_u32(jamo_peak_start as u32 + *self as u32).unwrap()
    }
}

#[deriving(Eq)]
#[deriving(FromPrimitive)]
#[deriving(Show)]
pub enum Final {
    FinalBlank = 0,
    FinalGiyeok = 1,
    FinalSsangGiyeok = 2,
    FinalGiyeokSiot = 3,
    FinalNieun = 4,
    FinalNieunJieut = 5,
    FinalNieunHieut = 6,
    FinalDigeut = 7,
    FinalRieul = 8,
    FinalRieulGiyeok = 9,
    FinalRieulMieum = 10,
    FinalRieulBieup = 11,
    FinalRieulSiot = 12,
    FinalRieulTieut = 13,
    FinalRieulPieup = 14,
    FinalRieulHieut = 15,
    FinalMieum = 16,
    FinalBieup = 17,
    FinalBieupSiot = 18,
    FinalSiot = 19,
    FinalSsangSiot = 20,
    FinalIeung = 21,
    FinalJieut = 22,
    FinalChieut = 23,
    FinalKieuk = 24,
    FinalTieut = 25,
    FinalPieup = 26,
    FinalHieut = 27,
}

impl Jamo for Final { }

impl Conversion for Final {
    fn from_char(c: char) -> Option<Final> {
        Jamo::from_char(c, jamo_final0_start)
    }

    fn char(&self) -> char {
        if *self != FinalBlank {
            std::char::from_u32(jamo_final0_start as u32 + *self as u32).unwrap()
        } else {
            '\0'
        }
    }
}

pub enum Alphabet {
    Initial(Initial),
    Vowel(Vowel),
    Final(Final),
}


pub trait SyllableTrait: Conversion {
    fn initial(&self) -> Initial;
    fn peak(&self) -> Vowel;
    fn final(&self) -> Final;
}

pub struct Syllable {
    initial: Initial,
    peak: Vowel,
    final: Final,
}

impl Syllable {
    pub fn from_char(c: char) -> Option<Syllable> {
        Conversion::from_char(c)
    }

    pub fn char(&self) -> char {
        let t: &SyllableTrait = self;
        t.char()
    }
}

impl Conversion for Syllable {
    fn from_char(c: char) -> Option<Syllable> {
        let code = c as uint;
        let mut base = code - (syllable_start as uint);
        if base < syllable_count {
            let final: Final = FromPrimitive::from_uint(base % final0_count).unwrap();
            base /= final0_count;
            let peak: Vowel = FromPrimitive::from_uint(base % vowel_count).unwrap();
            base /= vowel_count;
            let initial: Initial = FromPrimitive::from_uint(base % initial_count).unwrap();

            Some(Syllable { initial: initial, peak: peak, final: final })
        } else {
            None
        }
    }

    fn char(&self) -> char {
        std::char::from_u32(syllable_start as u32 + self.final as u32 +
            final0_count as u32 * (self.peak as u32 + (vowel_count as u32 * self.initial as u32)))
        .unwrap()
    }
}

impl SyllableTrait for Syllable {
    fn initial(&self) -> Initial {
        self.initial
    }

    fn peak(&self) -> Vowel {
        self.peak
    }

    fn final(&self) -> Final {
        self.final
    }
}


pub struct LazySyllable {
    data: u32,
}

impl LazySyllable {
    pub fn from_char(c: char) -> Option<Syllable> {
        Conversion::from_char(c)
    }
}

impl Conversion for LazySyllable {
    fn from_char(c: char) -> Option<LazySyllable> {
        Some(LazySyllable { data: c as u32 })
    }

    fn char(&self) -> char {
        std::char::from_u32(self.data).unwrap()
    }
}

impl SyllableTrait for LazySyllable {
    fn initial(&self) -> Initial {
        let code = (self.data - syllable_start as u32) / (vowel_count * final0_count) as u32;
        let char: Initial = FromPrimitive::from_u32(code).unwrap();
        char
    }

    fn peak(&self) -> Vowel {
        let code = (self.data - syllable_start as u32) / final0_count as u32 % vowel_count as u32;
        let char: Vowel = FromPrimitive::from_u32(code).unwrap();
        char
    }

    fn final(&self) -> Final {
        let code = (self.data - syllable_start as u32) % (initial_count * vowel_count) as u32;
        let char: Final = FromPrimitive::from_u32(code).unwrap();
        char
    }
}


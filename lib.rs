#![crate_id="hangeul"]
#![crate_type="lib"]
#![license="BSD simplified"]

/*! Tools to manipulate hangeul data in unicode.
 */

/// Unicode start point of syllable range
pub static syllable_start: char = '가'; // U+AC00
/// Unicode end point of syllable range
pub static syllable_end: char = '힣'; // U+D7A3
/// Size of syllable range in Unicode
pub static syllable_count: uint = syllable_end as uint - syllable_start as uint + 1;

/// Unicode start point of jamo initial range
pub static jamo_initial_start: char = 'ᄀ'; // U+1100
/// Unicode end point of jamo initial range
pub static jamo_initial_end: char = 'ᄒ'; // U+1112
/// Unicode start point of ancient jamo initial range
pub static jamo_initial_ancient_start: char = '\u1113';
/// Unicode end point of ancient jamo initial range
pub static jamo_initial_ancient_end: char = '\u115e';

/// Unicode filler character for jamo initial
pub static jamo_initial_filler: char = '\u115f';
/// Unicode filler character for jamo peak
pub static jamo_peak_filler: char = '\u1160';

/// Unicode start point of jamo peak range
pub static jamo_peak_start: char = 'ᅡ';
/// Unicode end point of jamo peak range
pub static jamo_peak_end: char = 'ᅵ'; // U+1175
/// Unicode start point of ancient jamo peak range
pub static jamo_peak_ancient_start: char = '\u1176';
/// Unicode start point of ancient jamo peak range
pub static jamo_peak_ancient_end: char = 'ᆧ';

/// Virtual Unicode start point of jamo final range supposing there is 'no final' character
pub static jamo_final0_start: char = '\u11a7';
/// Unicode start point of jamo final range
pub static jamo_final_start: char = 'ᆨ'; // U+11A8
/// Unicode end point of jamo final range
pub static jamo_final_end: char = 'ᇂ'; // U+11C
/// Unicode start point of ancient jamo final range
pub static jamo_final_ancient_start: char = '\u11c3';
/// Unicode end point of ancient jamo final range
pub static jamo_final_ancient_end: char = '\u11ff';

/// Unicode start point of jamo range
pub static jamo_start: char = jamo_initial_start;
/// Unicode end point of jamo range
pub static jamo_end: char = jamo_final_ancient_end;

/// Size of jamo initial range in Unicode
pub static initial_count: uint = jamo_initial_end as uint - jamo_initial_start as uint + 1; // 19
/// Size of jamo peak range in Unicode
pub static peak_count: uint = jamo_peak_end as uint - jamo_peak_start as uint + 1; // 21
/// Size of jamo final range in Unicode
pub static final_count: uint = jamo_final_end as uint - jamo_final_start as uint + 1; //27
/// Virtual size of jamo final range in Unicode supposing there is 'no final' character
pub static final0_count: uint = final_count + 1;
/// Number of vowels
pub static vowel_count: uint = peak_count;

pub static compat_modern_jamo_start: char = 'ㄱ'; // U+3131
pub static compat_modern_jamo_end: char = 'ㅣ'; // U+3163
pub static compat_ancient_jamo_start: char = 'ㅥ'; // U+3164
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
    Syllable,
    SyllableNFD,
    Jamo(Modernity),
    CompatibillityJamo(Modernity),
}

#[experimental]
impl DataClass {
    pub fn of_char(data: char) -> Option<DataClass> {
        match data {
            syllable_start..syllable_end => Some(Syllable),
            jamo_start..jamo_end => {
                Some(match data {
                    jamo_initial_start .. jamo_initial_end |
                    jamo_peak_start .. jamo_peak_end |
                    jamo_final_start .. jamo_final_end => {
                        Jamo(Modern)
                    }
                    _ => {
                        Jamo(Ancient)
                    }
                })
            },
            compat_modern_jamo_start..compat_modern_jamo_end => Some(CompatibillityJamo(Modern)),
            compat_ancient_jamo_start..compat_ancient_jamo_end => Some(CompatibillityJamo(Ancient)),
            _ => None,
        }
    }
}

/** Supports conversions between unicode <-> manipulable representation.

    There are supposed unicode code point for character type.

    - Syllable types are mapped to syllable ranges.
    - Jamo types are mapped to jamo ranges.
    - Compatitble jamo and ancient characters are out of this trait.
 */
pub trait Conversion {
    fn from_char(c: char) -> Option<Self> {
        Conversion::from_u32(c as u32)
    }
    fn from_u32(v: u32) -> Option<Self>;
    fn char(&self) -> Option<char>;
}

/** Supports unicode jamo ranges.
 */
pub trait Jamo: FromPrimitive {
    fn _from_u32(v: u32, base: char) -> Option<Self> {
        let opt: Option<Self> = FromPrimitive::from_u32(v - base as u32);
        opt
    }
}

/** Order of modern hangeul initials.
 */
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
    fn from_u32(v: u32) -> Option<Initial> {
        Jamo::_from_u32(v, jamo_initial_start)
    }

    fn char(&self) -> Option<char> {
        std::char::from_u32(jamo_initial_start as u32 + *self as u32)
    }
}

/** Order of modern hangeul vowels.
 */
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
    fn from_u32(v: u32) -> Option<Vowel> {
        Jamo::_from_u32(v, jamo_peak_start)
    }

    fn char(&self) -> Option<char> {
        std::char::from_u32(jamo_peak_start as u32 + *self as u32)
    }
}

/** Order of modern hangeul finals.
 */
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
    fn from_u32(v: u32) -> Option<Final> {
        if v == jamo_final0_start as u32 {
            None
        } else {
            Jamo::_from_u32(v, jamo_final0_start)
        }
    }

    fn char(&self) -> Option<char> {
        if *self != FinalBlank {
            std::char::from_u32(jamo_final0_start as u32 + *self as u32)
        } else {
            None
        }
    }
}

#[experimental]
pub enum Alphabet {
    Initial(Initial),
    Vowel(Vowel),
    Final(Final),
}

/** Supports syllable handling in alphabet level.
 */
pub trait Syllable: Conversion {
    /// initial
    fn initial(&self) -> Option<Initial>;
    /// peak
    fn peak(&self) -> Option<Vowel>;
    /// final
    fn final(&self) -> Option<Final>;
    /// final but return FinalBlank if final does not exist, never None
    fn final0(&self) -> Final;

    fn NFD(&self) -> Option<String>;
}

/** Unicode syllable range equivalent representation.
 */
pub struct ConcreteSyllable {
    initial: Initial,
    peak: Vowel,
    final: Final,
}

impl ConcreteSyllable {
    pub fn from_char(c: char) -> Option<ConcreteSyllable> {
        Conversion::from_char(c)
    }

    pub fn from_u32(code: u32) -> Option<ConcreteSyllable> {
        Conversion::from_u32(code)
    }

    pub fn char(&self) -> Option<char> {
        let t: &Syllable = self;
        t.char()
    }

    pub fn initial(&self) -> Initial {
        self.initial
    }

    pub fn peak(&self) -> Vowel {
        self.peak
    }

    pub fn final0(&self) -> Final {
        self.final
    }
}

impl Conversion for ConcreteSyllable {
    fn from_u32(code: u32) -> Option<ConcreteSyllable> {
        let mut base = code - (syllable_start as u32);
        if base < syllable_count as u32 {
            let final: Final = FromPrimitive::from_u32(base % final0_count as u32).unwrap();
            base /= final0_count as u32;
            let peak: Vowel = FromPrimitive::from_u32(base % vowel_count as u32).unwrap();
            base /= vowel_count as u32;
            let initial: Initial = FromPrimitive::from_u32(base % initial_count as u32).unwrap();

            Some(ConcreteSyllable { initial: initial, peak: peak, final: final })
        } else {
            None
        }
    }

    fn char(&self) -> Option<char> {
        std::char::from_u32(syllable_start as u32 + self.final as u32 +
            final0_count as u32 * (self.peak as u32 + (vowel_count as u32 * self.initial as u32)))
    }
}

impl Syllable for ConcreteSyllable {
    fn initial(&self) -> Option<Initial> {
        Some(self.initial())
    }

    fn peak(&self) -> Option<Vowel> {
        Some(self.peak())
    }

    fn final(&self) -> Option<Final> {
        if self.final0() == FinalBlank {
            None
        } else {
            Some(self.final0())
        }
    }

    fn final0(&self) -> Final {
        self.final0()
    }

    fn NFD(&self) -> Option<String> {
        let initial = self.initial();
        let peak = self.peak();
        let final = self.final0();

        let mut buf = String::new();
        buf.push_char(initial.char().unwrap());
        buf.push_char(peak.char().unwrap());
        if final != FinalBlank {
            buf.push_char(final.char().unwrap());
        }
        Some(buf)
    }
}


/** Unicode syllable range equivalent representation, but calculates values dynamically.
 */
pub struct LazySyllable {
    data: u32,
}

impl LazySyllable {
    pub fn from_char(c: char) -> Option<LazySyllable> {
        Conversion::from_char(c)
    }
}

impl Conversion for LazySyllable {
    fn from_u32(code: u32) -> Option<LazySyllable> {
        Some(LazySyllable { data: code })
    }

    fn char(&self) -> Option<char> {
        std::char::from_u32(self.data)
    }
}

impl Syllable for LazySyllable {
    fn initial(&self) -> Option<Initial> {
        let code = (self.data - syllable_start as u32) / (vowel_count * final0_count) as u32;
        let char: Option<Initial> = FromPrimitive::from_u32(code);
        char
    }

    fn peak(&self) -> Option<Vowel> {
        let code = (self.data - syllable_start as u32) / final0_count as u32 % vowel_count as u32;
        let char: Option<Vowel> = FromPrimitive::from_u32(code);
        char
    }

    fn final(&self) -> Option<Final> {
        match self.final0() {
            FinalBlank => None,
            any => Some(any),
        }
    }

    fn final0(&self) -> Final {
        let code = (self.data - syllable_start as u32) % final0_count as u32;
        let char: Option<Final> = FromPrimitive::from_u32(code);
        char.unwrap()
    }

    fn NFD(&self) -> Option<String> {
        match ConcreteSyllable::from_u32(self.data) {
            Some(syl) => {
                let syllable: &Syllable = &syl;
                syllable.NFD()
            }
            None => None,
        }
    }
}

/** Syllable builder with Syllable trait.
 */
pub struct SyllableBuilder {
    pub initial: Option<Initial>,
    pub peak: Option<Vowel>,
    pub final: Option<Final>,
}

impl SyllableBuilder {
    pub fn new() -> SyllableBuilder {
        SyllableBuilder { initial: None, peak: None, final: None, }
    }

    pub fn from_alphabet(
            initial: Option<Initial>, peak: Option<Vowel>, final: Option<Final>)
            -> SyllableBuilder {
        SyllableBuilder { initial: initial, peak: peak, final: final, }
    }
}

impl Conversion for SyllableBuilder {
    fn from_u32(code: u32) -> Option<SyllableBuilder> {
        let opt = ConcreteSyllable::from_u32(code);
        match opt {
            Some(syl) => {
                let syltr: &Syllable = &syl;
                Some(SyllableBuilder {
                    initial: syltr.initial(),
                    peak: syltr.peak(),
                    final: syltr.final(),
                })
            }
            None => None
        }
    }

    fn char(&self) -> Option<char> {
        match self.initial {
            Some(initial) => match self.peak {
                Some(peak) => match self.final {
                    Some(final) => {
                        ConcreteSyllable { initial: initial, peak: peak, final: final }.char()
                    }
                    None => {
                        ConcreteSyllable { initial: initial, peak: peak, final: FinalBlank }.char()
                    }
                },
                None => None,
            },
            None => None,
        }
    }
}

impl Syllable for SyllableBuilder {
    fn initial(&self) -> Option<Initial> {
        self.initial
    }

    fn peak(&self) -> Option<Vowel> {
        self.peak
    }

    fn final(&self) -> Option<Final> {
        match self.final {
            Some(FinalBlank) => None,
            _ => self.final,
        }
    }

    fn final0(&self) -> Final {
        match self.final {
            Some(c) => c,
            None => FinalBlank,
        }
    }

    fn NFD(&self) -> Option<String> {
        let initial = self.initial();
        let peak = self.peak();
        let final = self.final();

        if initial == None && peak == None && final == None {
            None
        } else {
            let mut buf = String::new();
            buf.push_char(match initial {
                Some(c) => c.char().unwrap(),
                None => jamo_initial_filler,
            });
            buf.push_char(match peak {
                Some(c) => c.char().unwrap(),
                None => jamo_peak_filler,
            });
            if final != None {
                buf.push_char(final.unwrap().char().unwrap());
            }
            Some(buf.into_owned())
        }
    }
}

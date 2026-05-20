#![allow(unused)]

use std::ops::BitOr;

const INVERTER: TrunicSegments = TrunicSegments(1 << 11);

/// # Bit-layout
///
/// The bits are arranged like `0b0000_i_cccccc_vvvvv`
///
/// The `i`-bit marks an inversion from CV to VC
///
/// The 6 `c`-bits mark consonant segments arranged counter-clockwise:
/// - The first 3 `c`-bits mark the top consonant segments right to left
/// - The last 3 `c`-bits mark the bottom consonant segments left to right
///
/// The 5 `v`-bits mark vowel segments arranged counter-clockwise:
/// - The first 2 `v`-bits mark the top vowel segments right to left
/// - The middle `v`-bit marks the left-most vowel segment
/// - The last 2 `v`-bits mark the bottom vowel segments left to right
#[derive(Clone, Copy)]
pub struct TrunicSegments(u16);
impl BitOr for TrunicSegments {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        TrunicSegments(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone)]
pub enum TrunicSymbol {
    V(TrunicVowel),
    C(TrunicConsonant),
    CV(TrunicConsonant, TrunicVowel),
    VC(TrunicVowel, TrunicConsonant),
}
impl TrunicSymbol {
    pub fn segments(&self) -> TrunicSegments {
        match self {
            TrunicSymbol::V(v) => v.segments(),
            TrunicSymbol::C(c) => c.segments(),
            TrunicSymbol::CV(c, v) => v.segments() | c.segments(),
            TrunicSymbol::VC(v, c) => INVERTER | v.segments() | c.segments(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrunicVowel {
    /// `/a/` as in `/kˈat/` (cat)
    Ae,
    /// `/ɒ/` as in `/ˈɒn/` (on)
    Oh,
    /// `/ɪ/` as in `/ʃˈɪp/` (ship)
    Ih,
    /// `/ɛ/` as in `/bˈɛd/` (bed)
    Eh,
    /// `/ʊ/` as in `/ɡˈʊd/` (good)
    Uo,
    /// `/ə/` as in `/tˈiːtʃə/` (teacher)
    Er,
    /// `/iː/` as in `/ʃˈiːp/` (sheep)
    Ii,
    /// `/uː/` as in `/ʃˈuːt/` (shoot)
    Uu,
    /// `/ɜː/` as in `/bˈɜːd/` (bird)
    Ir,
    /// `/ɔː/` as in `/dˈɔː/` (door)
    Oa,
    /// `/ɑː/` as in `/fˈɑː/` (far)
    Aa,
    /// `/iə/` as in `/hˈiə/` (here)
    Ie,
    /// `/eɪ/` as in `/wˈeɪt/` (wait)
    Ei,
    /// `/aɪ/` as in `/mˈaɪ/` (my)
    Ai,
    /// `/ʌ/` as in `/ˈʌp/` (up)
    Ah,
    /// `/aʊ/` as in `/kˈaʊ/` (cow)
    Ao,
    /// `/əʊ/` as in `/ʃˈəʊ/` (show)
    Ou,
    /// `/eə/` as in `/hˈeə/` (hair)
    Ea,
}
impl TrunicVowel {
    fn segments(&self) -> TrunicSegments {
        match self {
            TrunicVowel::Ae => TrunicSegments(0b11100),
            TrunicVowel::Oh => TrunicSegments(0b01100),
            TrunicVowel::Ih => TrunicSegments(0b00011),
            TrunicVowel::Eh => TrunicSegments(0b00111),
            TrunicVowel::Uo => TrunicSegments(0b00110),
            TrunicVowel::Er => TrunicSegments(0b11000),
            TrunicVowel::Ii => TrunicSegments(0b01111),
            TrunicVowel::Uu => TrunicSegments(0b11110),
            TrunicVowel::Ir => TrunicSegments(0b10111),
            TrunicVowel::Oa => TrunicSegments(0b11101),
            TrunicVowel::Aa => TrunicSegments(0b11011),
            TrunicVowel::Ie => TrunicSegments(0b01101),
            TrunicVowel::Ei => TrunicSegments(0b01000),
            TrunicVowel::Ai => TrunicSegments(0b10000),
            TrunicVowel::Ah => TrunicSegments(0b00010),
            TrunicVowel::Ao => TrunicSegments(0b00001),
            TrunicVowel::Ou => TrunicSegments(0b11111),
            TrunicVowel::Ea => TrunicSegments(0b00101),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrunicConsonant {
    /// `/m/` as in `/mˈan/` (man)
    Mxx,
    /// `/n/` as in `/nˈaʊ/` (now)
    Nxx,
    /// `/ŋ/` as in `/sˈɪŋ/` (car)
    Ngx,
    /// `/p/` as in `/pˈiː/` (pea)
    Pxx,
    /// `/b/` as in `/bˈəʊt/` (boat)
    Bxx,
    /// `/t/` as in `/tˈiː/` (tea)
    Txx,
    /// `/d/` as in `/dˈɒɡ/` (dog)
    Dxx,
    /// `/k/` as in `/kˈɑː/` (car)
    Kxx,
    /// `/ɡ/` as in `/ɡˈəʊ/` (go)
    Gxx,
    /// `/dʒ/` as in `/dʒˈuːn/` (June)
    Dzh,
    /// `/tʃ/` as in `/tʃˈiːz/` (cheese)
    Tsh,
    /// `/f/` as in `/flˈaɪ/` (fly)
    Fxx,
    /// `/v/` as in `/vˈɪdɪəʊ/` (video)
    Vxx,
    /// `/θ/` as in `/θˈɪnk/` (think)
    ThV,
    /// `/ð/` as in `/ðˈɪs/` (this)
    ThU,
    /// `/s/` as in `/sˈiː/` (see)
    Sxx,
    /// `/z/` as in `/zˈuː/` (zoo)
    Zxx,
    /// `/ʃ/` as in `/ʃˈal/` (shall)
    Shx,
    /// `/ʒ/` as in `/vˈɪʒən/` (vision)
    Zhx,
    /// `/h/` as in `/hˈat/` (hat)
    Hxx,
    /// `/ɹ/` as in `/ɹˈɛd/` (red)
    Rxx,
    /// `/j/` as in `/jˈɛs/` (yes)
    Jxx,
    /// `/w/` as in `/wˈɛt/` (wet)
    Wxx,
    /// `/l/` as in `/lˈʌv/` (love)
    Lxx,
}
impl TrunicConsonant {
    fn segments(&self) -> TrunicSegments {
        match self {
            TrunicConsonant::Mxx => TrunicSegments(0b000101 << 5),
            TrunicConsonant::Nxx => TrunicSegments(0b001101 << 5),
            TrunicConsonant::Ngx => TrunicSegments(0b111111 << 5),
            TrunicConsonant::Pxx => TrunicSegments(0b100010 << 5),
            TrunicConsonant::Bxx => TrunicSegments(0b010001 << 5),
            TrunicConsonant::Txx => TrunicSegments(0b101010 << 5),
            TrunicConsonant::Dxx => TrunicSegments(0b010101 << 5),
            TrunicConsonant::Kxx => TrunicSegments(0b110001 << 5),
            TrunicConsonant::Gxx => TrunicSegments(0b100011 << 5),
            TrunicConsonant::Dzh => TrunicSegments(0b010100 << 5),
            TrunicConsonant::Tsh => TrunicSegments(0b001010 << 5),
            TrunicConsonant::Fxx => TrunicSegments(0b100110 << 5),
            TrunicConsonant::Vxx => TrunicSegments(0b011001 << 5),
            TrunicConsonant::ThV => TrunicSegments(0b111010 << 5),
            TrunicConsonant::ThU => TrunicSegments(0b010111 << 5),
            TrunicConsonant::Sxx => TrunicSegments(0b110110 << 5),
            TrunicConsonant::Zxx => TrunicSegments(0b011011 << 5),
            TrunicConsonant::Shx => TrunicSegments(0b101111 << 5),
            TrunicConsonant::Zhx => TrunicSegments(0b111101 << 5),
            TrunicConsonant::Hxx => TrunicSegments(0b010011 << 5),
            TrunicConsonant::Rxx => TrunicSegments(0b110010 << 5),
            TrunicConsonant::Jxx => TrunicSegments(0b011010 << 5),
            TrunicConsonant::Wxx => TrunicSegments(0b101000 << 5),
            TrunicConsonant::Lxx => TrunicSegments(0b010010 << 5),
        }
    }
}

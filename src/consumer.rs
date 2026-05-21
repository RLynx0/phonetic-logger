use phonetisaurus_g2p::PhonetizationResult;
use tokio::sync::mpsc::Receiver;

use crate::{
    Packet,
    consumer::trunic::{TrunicConsonant, TrunicSymbol, TrunicVowel},
};

mod trunic;

pub async fn consumer(mut rx: Receiver<Packet>) -> anyhow::Result<()> {
    while let Some(packet) = rx.recv().await {
        let text = match packet {
            Packet::Chars(text) => text,
            Packet::Phonetic(res) => consume_phonemized(&res).into(),
        };
        for ch in text.chars() {
            print!("{ch}");
        }
    }
    Ok(())
}

fn consume_phonemized(phonemized: &PhonetizationResult) -> String {
    let phonetic_word = &phonemized.phonemes;
    match to_trunic(phonetic_word) {
        v if v.is_empty() => format!("/{phonetic_word}/"),
        v => v.iter().map(|s| format!("{s:?}")).collect(),
    }
}

fn to_trunic(phonemes: &str) -> Vec<TrunicSymbol> {
    let mut symbols = Vec::new();
    let mut last = None;

    for symbol in to_trunic_first_pass(phonemes) {
        match (last, symbol) {
            (None, any) => last = Some(any),
            (Some(TrunicSymbol::C(c)), TrunicSymbol::V(v)) => last = Some(TrunicSymbol::CV(c, v)),
            (Some(TrunicSymbol::V(v)), TrunicSymbol::C(c)) => last = Some(TrunicSymbol::VC(v, c)),
            (Some(old), new) => {
                symbols.push(old);
                last = Some(new);
            }
        }
    }
    if let Some(symbol) = last {
        symbols.push(symbol);
    }

    symbols
}

fn to_trunic_first_pass(phonemes: &str) -> Vec<TrunicSymbol> {
    let mut symbols = Vec::new();
    let mut i = 0usize;
    macro_rules! parse {
        ($p: expr => $v: expr) => {
            if (phonemes[i..].starts_with($p)) {
                symbols.push($v);
                i += $p.len();
                continue;
            }
        };
    }

    while i < phonemes.len() {
        // Vowels
        parse!("iː" => TrunicSymbol::V(TrunicVowel::Ii));
        parse!("uː" => TrunicSymbol::V(TrunicVowel::Uu));
        parse!("ɜː" => TrunicSymbol::V(TrunicVowel::Ir));
        parse!("ɔː" => TrunicSymbol::V(TrunicVowel::Oa));
        parse!("ɑː" => TrunicSymbol::V(TrunicVowel::Aa));
        parse!("iə" => TrunicSymbol::V(TrunicVowel::Ie));
        parse!("eɪ" => TrunicSymbol::V(TrunicVowel::Ei));
        parse!("aɪ" => TrunicSymbol::V(TrunicVowel::Ai));
        parse!("aʊ" => TrunicSymbol::V(TrunicVowel::Ao));
        parse!("əʊ" => TrunicSymbol::V(TrunicVowel::Ou));
        parse!("eə" => TrunicSymbol::V(TrunicVowel::Ea));
        parse!("a" => TrunicSymbol::V(TrunicVowel::Ae));
        parse!("ɒ" => TrunicSymbol::V(TrunicVowel::Oh));
        parse!("ɪ" => TrunicSymbol::V(TrunicVowel::Ih));
        parse!("ɛ" => TrunicSymbol::V(TrunicVowel::Eh));
        parse!("ʊ" => TrunicSymbol::V(TrunicVowel::Uo));
        parse!("ə" => TrunicSymbol::V(TrunicVowel::Er));
        parse!("ʌ" => TrunicSymbol::V(TrunicVowel::Ah));

        // Consonants
        parse!("dʒ" => TrunicSymbol::C(TrunicConsonant::Dzh));
        parse!("tʃ" => TrunicSymbol::C(TrunicConsonant::Tsh));
        parse!("m" => TrunicSymbol::C(TrunicConsonant::Mxx));
        parse!("n" => TrunicSymbol::C(TrunicConsonant::Nxx));
        parse!("ŋ" => TrunicSymbol::C(TrunicConsonant::Ngx));
        parse!("p" => TrunicSymbol::C(TrunicConsonant::Pxx));
        parse!("b" => TrunicSymbol::C(TrunicConsonant::Bxx));
        parse!("t" => TrunicSymbol::C(TrunicConsonant::Txx));
        parse!("d" => TrunicSymbol::C(TrunicConsonant::Dxx));
        parse!("k" => TrunicSymbol::C(TrunicConsonant::Kxx));
        parse!("ɡ" => TrunicSymbol::C(TrunicConsonant::Gxx));
        parse!("f" => TrunicSymbol::C(TrunicConsonant::Fxx));
        parse!("v" => TrunicSymbol::C(TrunicConsonant::Vxx));
        parse!("θ" => TrunicSymbol::C(TrunicConsonant::ThV));
        parse!("ð" => TrunicSymbol::C(TrunicConsonant::ThU));
        parse!("s" => TrunicSymbol::C(TrunicConsonant::Sxx));
        parse!("z" => TrunicSymbol::C(TrunicConsonant::Zxx));
        parse!("ʃ" => TrunicSymbol::C(TrunicConsonant::Shx));
        parse!("ʒ" => TrunicSymbol::C(TrunicConsonant::Zhx));
        parse!("h" => TrunicSymbol::C(TrunicConsonant::Hxx));
        parse!("ɹ" => TrunicSymbol::C(TrunicConsonant::Rxx));
        parse!("j" => TrunicSymbol::C(TrunicConsonant::Jxx));
        parse!("w" => TrunicSymbol::C(TrunicConsonant::Wxx));
        parse!("l" => TrunicSymbol::C(TrunicConsonant::Lxx));

        // Fallback
        if let Some(next_phoneme) = phonemes[i..].chars().next() {
            // eprintln!("Could not parse {next_phoneme:?}");
            i += next_phoneme.len_utf8();
        }
    }

    symbols
}

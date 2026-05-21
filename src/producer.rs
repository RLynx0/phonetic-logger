use std::sync::Arc;

use anyhow::anyhow;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    combinator::{fail, map},
    multi::many0,
};
use phonetisaurus_g2p::PhonetisaurusModel;
use tokio::sync::mpsc::Sender;

use crate::{PHONETISAURUS_MODEL, Packet};

pub async fn producer(input: String, tx: Sender<Packet>) -> anyhow::Result<()> {
    let phonemizer = PhonetisaurusModel::try_from(PHONETISAURUS_MODEL)?;
    for part in split_text(&input)? {
        tx.send(match part {
            TextPart::NotAlphabetic(text) => Packet::Chars(text),
            TextPart::Alphabetic(word) => to_phoneme_packet(&phonemizer, &word),
        })
        .await?
    }
    Ok(())
}

fn to_phoneme_packet(phonemizer: &PhonetisaurusModel, word: &str) -> Packet {
    match phonemizer.phonemize_word(word) {
        Ok(res) => Packet::Phonetic(res),
        Err(e) => {
            eprintln!("{e}");
            Packet::Chars(word.into())
        }
    }
}

fn split_text(input: &str) -> anyhow::Result<Vec<TextPart>> {
    let res: IResult<&str, Vec<TextPart>> = many0(alt((
        map(
            map(take_while1(|c: char| !c.is_alphabetic()), Arc::from),
            TextPart::NotAlphabetic,
        ),
        map(
            map(
                take_while1(|c: char| c.is_alphabetic() || c == '\''),
                Arc::from,
            ),
            TextPart::Alphabetic,
        ),
        fail(),
    )))
    .parse(input);
    match res {
        Err(e) => Err(anyhow!(e.to_string())),
        Ok((_, parts)) => Ok(parts),
    }
}

#[derive(Debug, Clone)]
enum TextPart {
    Alphabetic(Arc<str>),
    NotAlphabetic(Arc<str>),
}

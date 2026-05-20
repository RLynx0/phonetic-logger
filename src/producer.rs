use std::sync::Arc;

use anyhow::anyhow;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    combinator::{fail, map},
    multi::many0,
};
use piper_phoneme_streaming::{FullG2p, Language};
use tokio::sync::mpsc::Sender;

pub async fn producer(input: String, tx: Sender<Arc<str>>) -> anyhow::Result<()> {
    let g2p = FullG2p::new(Language::English)?;
    for part in split_text(&input)? {
        tx.send(match part {
            TextPart::NotAlphabetic(text) => text,
            TextPart::Alphabetic(word) => to_phoneme_str(&g2p, word),
        })
        .await?
    }
    Ok(())
}

fn to_phoneme_str(g2p: &FullG2p, text: Arc<str>) -> Arc<str> {
    match g2p.g2p(&text) {
        Err(_) => text.clone(),
        Ok(phonemes) => phonemes.iter().map(|p| p.token).collect::<String>().into(),
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

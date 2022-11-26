use std::io::{Read, Write};

use bitvec::vec::BitVec;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub mod collection;
pub mod copy;
pub mod text;
pub mod url;

const URL_SAFE_ENGINE: base64::engine::fast_portable::FastPortable =
    base64::engine::fast_portable::FastPortable::from(
        &base64::alphabet::URL_SAFE,
        base64::engine::fast_portable::NO_PAD,
    );

pub fn all_cards() -> Vec<Card> {
    let card_db = include_str!("../snap.db");
    let cards: Vec<InternalCard> = serde_json::from_str(card_db).unwrap();

    cards.into_iter().map(Into::into).collect()
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Card {
    pub id: usize,
    pub name: &'static str,
    pub cost: i8,
    pub power: i8,
    pub ability: Option<&'static str>,
    pub pool: Pool,
    pub art: Option<String>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Pool {
    Pool1,
    Pool2,
    Pool3,
    Pool4,
    Pool5,
    Unreleased,
    Unknown,
}

/// This maps from a card in snap.db to a card in the game.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct InternalCard {
    cid: usize,
    name: &'static str,
    //#[serde(rename = "type")]
    //card_type: String,
    cost: Option<i8>,
    power: Option<i8>,
    ability: &'static str,
    //flavor: String,
    art: String,
    //alternate_art: String,
    //url: String,
    //status: String,
    //carddefid: String,
    //variants: Vec<InternalVariant>,
    //source: Option<String>,
    source_slug: Option<&'static str>,
    //tags: Vec<InternalTag>,
    //rarity: Option<String>,
    //rarity_slug: Option<String>,
    //difficulty: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct InternalVariant {
    cid: usize,
    vid: usize,
    art: String,
    art_filename: String,
    rarity: String,
    rarity_slug: String,
    variant_order: String,
    status: String,
    full_description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct InternalTag {
    tag_id: usize,
    tag: String,
    tag_slug: String,
}

impl From<InternalCard> for Card {
    fn from(card: InternalCard) -> Self {
        Card {
            id: card.cid,
            name: card.name,
            cost: card.cost.unwrap_or_default(),
            power: card.power.unwrap_or_default(),
            ability: if card.ability.is_empty() {
                None
            } else {
                Some(card.ability)
            },
            art: if card.art.is_empty() {
                None
            } else {
                Some(card.art)
            },
            pool: match card.source_slug {
                Some("recruit-season") => Pool::Pool1,
                Some("pool-1") => Pool::Pool1,
                Some("pool-2") => Pool::Pool2,
                Some("pool-3") => Pool::Pool3,
                Some("pool-4") => Pool::Pool4,
                Some("season-pass") => Pool::Pool5,
                Some("pool-5") => Pool::Pool5,
                None => Pool::Unreleased,
                Some("not-available") => Pool::Unreleased,
                _ => Pool::Unknown,
            },
        }
    }
}

pub fn uncompress_bitvec(compressed: &str) -> BitVec {
    let compressed = base64::decode_engine(compressed, &URL_SAFE_ENGINE).unwrap();
    let mut decoder = flate2::read::DeflateDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    let mut contains = BitVec::new();

    for byte in decompressed {
        for bit in (0..8).rev() {
            contains.push((byte >> bit) & 1 == 1);
        }
    }

    contains
}

/// given a bitvec, returns a gzipped base64 string
pub fn compress_bitvec(bitvec: &BitVec) -> String {
    let mut compressed = Vec::new();
    {
        let mut encoder =
            flate2::write::DeflateEncoder::new(&mut compressed, flate2::Compression::default());

        let compressed: Vec<u8> = bitvec
            .chunks(8)
            .map(|window| window.iter().fold(0, |acc, bit| (acc << 1) | (*bit as u8)))
            .collect_vec();
        encoder.write_all(&compressed).unwrap();
    }
    base64::encode_engine(&compressed, &URL_SAFE_ENGINE)
}

fn get_local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

/// Storages the a BitVec of cards into local storage, using web-sys.
pub fn store<T: Serialize>(key: impl AsRef<str>, value: &T) {
    let value = serde_json::to_string(value).unwrap();
    get_local_storage().set_item(key.as_ref(), &value).unwrap();
}

/// Loads the BitVec of cards from local storage, using web-sys.
pub fn load<T: DeserializeOwned>(key: impl AsRef<str>) -> Option<T> {
    let contains = get_local_storage().get_item(key.as_ref()).ok()??;
    let contains: T = serde_json::from_str(&contains).unwrap();
    Some(contains)
}

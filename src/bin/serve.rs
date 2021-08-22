#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
// #[macro_use]
extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;

use std::{collections::BTreeMap, fs::File, io::BufReader, time::Instant};

use rhyme_es::{Entry, WordRepo};
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use syllabize_es::{RhymeType, Word};

struct Config {
    words: WordRepo,
}

fn main() {
    let mut f = BufReader::new(File::open("rhyme.db").unwrap());
    let words: WordRepo = bincode::deserialize_from(&mut f).unwrap();

    rocket::ignite()
        .manage(Config { words })
        .mount("/api", routes![rhyme])
        .launch();
}

#[get("/<tipo>/<palabra>/<numero_de_silabas>/<bando>")]
fn rhyme(
    config: State<Config>,
    tipo: String,
    palabra: String,
    numero_de_silabas: u8,
    bando: u8,
) -> Json<Res> {
    let start = Instant::now();
    let w: Word = palabra.as_str().trim().into();
    let rhyming_type = if tipo == "a" {
        RhymeType::Assonant
    } else {
        RhymeType::Consonant
    };
    let mut rhyming_words = vec![];
    for (k, v) in config.words.iter() {
        let z: Word = k.as_str().into();
        if w.rhymes_with(&z, rhyming_type) {
            rhyming_words.extend_from_slice(&v[..]);
        }
    }

    rhyming_words.sort_by(|a, b| {
        b.freq
            .partial_cmp(&a.freq)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut res = BTreeMap::new();

    for rwd in rhyming_words {
        if bando == 0 || freq2band(rwd.freq) >= bando {
            let pwd: Word = rwd.word.as_str().into();
            let the_entry = res.entry(pwd.syllables.len()).or_insert_with(Vec::new);
            the_entry.push(rwd);
        }
    }

    let mut res2 = vec![];
    for (k, v) in res.iter() {
        if numero_de_silabas == 0 || *k as u8 == numero_de_silabas {
            res2.push(ResBySyllableCount {
                syllable_count: *k as u8,
                words: v.clone(),
            });
        }
    }
    let duration = Instant::now() - start;
    Json(Res {
        contents: res2,
        duration: duration.as_secs_f64(),
    })
}

#[derive(Serialize)]
struct Res {
    contents: Vec<ResBySyllableCount>,
    duration: f64,
}

#[derive(Serialize)]
struct ResBySyllableCount {
    syllable_count: u8,
    words: Vec<Entry>,
}

/// https://public.oed.com/how-to-use-the-oed/key-to-frequency/
/// Doesn't seem to work with Spanish, hence modifications
pub fn freq2band(freq: f32) -> u8 {
    if freq >= 100.0 {
        5
    } else if freq >= 1.0 {
        4
    } else if freq >= 0.01 {
        3
    } else if freq > 0.0 {
        2
    } else {
        1 
    }
}

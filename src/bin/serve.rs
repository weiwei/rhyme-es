#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
// #[macro_use]
// #[macro_use] extern crate serde_derive;

use std::{fs::File, io::BufReader, time::Instant};

use rhyme_es::{Entry, WordRepo};
use rocket::http::Method;
use rocket::State;
use rocket_cors::AllowedOrigins;
use serde::Serialize;
use rocket::serde::json::Json;
use syllabize_es::{RhymeOptions, Word};

struct Config {
    words: WordRepo,
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let mut f = BufReader::new(File::open("rhyme.db").unwrap());
    let words: WordRepo = bincode::deserialize_from(&mut f).unwrap();

    rocket::build()
        .manage(Config { words })
        .mount("/api", routes![consonant_rhyme, assonant_rhyme])
        .attach(cors)
}

#[get("/c/<palabra>?<nsyl>&<freq>&<yeismo>&<seseo>&<bv>")]
async fn consonant_rhyme(
    config: &State<Config>,
    palabra: String,
    nsyl: Option<u8>,
    freq: Option<u8>,
    yeismo: Option<bool>,
    seseo: Option<bool>,
    bv: Option<bool>,
) -> Json<Res> {
    let start = Instant::now();
    let w: Word = palabra.as_str().into();
    let nsyl = nsyl.unwrap_or(0);
    let freq = freq.unwrap_or(0);
    let yeismo = yeismo.unwrap_or(false);
    let seseo = seseo.unwrap_or(false);
    let bv = bv.unwrap_or(false);
    let opts = RhymeOptions {
        yeismo,
        seseo,
        b_equals_v: bv,
    };
    let mut rhyming_words = vec![];
    for (k, v) in config.words.iter() {
        let z: Word = k.as_str().into();
        if w.rhymes_with(&z, Some(opts)) {
            rhyming_words.extend_from_slice(&v[..]);
        }
    }

    rhyming_words.sort_by(|a, b| {
        b.freq
            .partial_cmp(&a.freq)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut res = vec![];

    for rwd in rhyming_words {
        if freq == 0 || freq2band(rwd.freq) >= freq {
            if nsyl == 0 || rwd.nsyl == nsyl {
                res.push(rwd);
            }
        } else {
            break;
        }
    }

    let duration = Instant::now() - start;
    Json(Res {
        contents: res,
        duration: duration.as_secs_f64(),
    })
}

#[get("/a/<palabra>?<nsyl>&<freq>")]
fn assonant_rhyme(
    config: &State<Config>,
    palabra: String,
    nsyl: Option<u8>,
    freq: Option<u8>,
) -> Json<Res> {
    let start = Instant::now();
    let nsyl = nsyl.unwrap_or(0);
    let freq = freq.unwrap_or(0);
    let w: Word = palabra.as_str().into();
    let mut rhyming_words = vec![];
    for (k, v) in config.words.iter() {
        let z: Word = k.as_str().into();
        if w.assonant_rhymes_with(&z) {
            rhyming_words.extend_from_slice(&v[..]);
        }
    }

    rhyming_words.sort_by(|a, b| {
        b.freq
            .partial_cmp(&a.freq)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut res = vec![];

    for rwd in rhyming_words {
        if freq == 0 || freq2band(rwd.freq) >= freq {
            if nsyl == 0 || rwd.nsyl == nsyl {
                res.push(rwd);
            }
        } else {
            break;
        }
    }

    let duration = Instant::now() - start;
    Json(Res {
        contents: res,
        duration: duration.as_secs_f64(),
    })
}

#[derive(Serialize)]
struct Res {
    contents: Vec<Entry>,
    duration: f64,
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

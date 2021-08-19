/// Serialize dictionary and frequency data to the disk.
use regex::Regex;
use rhyme_es::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufWriter};
use std::path::Path;
use std::time::Instant;

use syllabize_es::Word;

/// The ratio to normalized frequency, i.e. frequency per 1 million words from
/// text. Note that the RAE file already have normalized frequency but it's only
/// rounded to 2 decimals, which is not quite enough here.
const RATIO: f32 = 152.55832;

/// Reads frequency data
fn read_freq() -> HashMap<String, f32> {
    // List contains non-Spanish words that needs to be filtered out.
    let reg = Regex::new("^[a-záéíóúñ]+$").unwrap();
    let mut frequencies = HashMap::new();
    let start = Instant::now();
    if let Ok(lines) = read_lines("CREA_TOTAL.TXT") {
        // TODO: Hardcoded file position
        println!("Reading freq list...");
        for line in lines.flatten() {
            let line_content: Vec<&str> = line.split('\t').collect();
            let word = line_content[1].trim();
            if !reg.is_match(word) {
                continue;
            }
            let freq: f32 = if line_content.len() > 2 {
                line_content[2]
                    .trim()
                    .replace(",", "")
                    .parse::<f32>()
                    .unwrap()
            } else {
                0.0
            };
            frequencies.insert(word.to_string(), freq / RATIO);
        }
    }
    println!("Done in {:?}", Instant::now() - start);
    frequencies
}

/// Reads dictionary data. Note that hunspell has dictionaries for different
/// countries, but here only the Peninsula dictionary is used.
/// TODO: Support different dictionaries
fn read_dictionary() -> HashSet<String> {
    let mut words = HashSet::new();
    let start = Instant::now();
    if let Ok(lines) = read_lines("words.txt") {
        // TODO: Remove hardcode
        println!("Reading dictionary...");
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            words.insert(line.to_owned());
        }
    }
    println!("Done in {:?}", Instant::now() - start);
    words
}

fn main() {
    let words = read_dictionary();
    let freq_map = read_freq();

    println!("Merging...");
    let start = Instant::now();
    // Words are a hashmap with the rhyming part as the key
    let mut all_words: HashMap<String, Vec<Entry>> = HashMap::new();
    for w in words {
        let the_word: Word = w.as_str().into();
        let freq = freq_map.get(&w).unwrap_or(&0.0);
        let the_entry = all_words.entry(the_word.rhyme()).or_insert_with(Vec::new);
        the_entry.push(Entry {
            word: w,
            freq: *freq,
        });
    }
    let mut f = BufWriter::new(File::create("rhyme.db").unwrap());
    bincode::serialize_into(&mut f, &all_words).unwrap();
    println!("Done in {:?}", Instant::now() - start);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

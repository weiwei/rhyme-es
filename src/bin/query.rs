/// Load serialized data, search for rhyming words and print the results.
use rhyme_es::WordRepo;
use std::fs::File;
use std::io::{self, BufReader, *};
use std::time::Instant;
use syllabize_es::Word;

fn main() {
    let start = Instant::now();
    let mut f = BufReader::new(File::open("rhyme.db").unwrap());
    let all_words: WordRepo = bincode::deserialize_from(&mut f).unwrap();
    println!("{:?}", Instant::now() - start);
    loop {
        print!("Your word: ");
        io::stdout().flush().unwrap();
        let mut x = String::with_capacity(128);
        io::stdin().read_line(&mut x).expect("Error reading input");
        let w: Word = x.trim().into();
        let mut rhyming_words = vec![];
        for (k, v) in all_words.iter() {
            let z: Word = k.as_str().into();
            if w.rhymes_with(&z, None) {
                rhyming_words.extend_from_slice(&v[..]);
            }
        }

        rhyming_words.sort_by(|a, b| {
            b.freq
                .partial_cmp(&a.freq)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let res = rhyming_words
            .iter()
            .map(|x| x.word.to_owned())
            .collect::<Vec<String>>()
            .join(", ");

        println!("{}", res);
    }
}

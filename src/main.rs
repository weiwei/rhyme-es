use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, *};
use std::path::Path;
use syllabize_es::{RhymeType, Word};

fn main() {
    let reg = Regex::new("^[a-záéíóúñ]+$").unwrap();
    let matches = App::new("rhyme")
        .version("1.0")
        .author("Weiwei Wang <gastlygem@gmail.com>")
        .about("Find rhyming words")
        .arg(Arg::with_name("word").value_name("WORD").takes_value(true))
        .arg(
            Arg::with_name("resource")
                .value_name("RESOURCE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threshold")
                .value_name("THRESHOLD")
                .takes_value(true),
        )
        .arg(Arg::with_name("assonants").short("a"))
        .arg(Arg::with_name("looping").short("l"))
        .get_matches();

    let word: Word = matches.value_of("word").unwrap().into();

    let resource = matches.value_of("resource").unwrap();
    let threshold = matches
        .value_of("threshold")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let contain_assonants = if matches.is_present("assonants") {
        RhymeType::Assonant
    } else {
        RhymeType::Consonant
    };

    let looping = matches.is_present("looping");

    // File hosts must exist in current path before this produces output
    let mut todo: HashMap<String, Vec<Entry>> = HashMap::new();
    let path = Path::new(resource);
    let mut recs: HashMap<usize, Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let x: Vec<&str> = ip.split('\t').collect();
                let y = if x.len() > 1 { x[1] } else { x[0] };
                if y.chars().count() < 2 || !reg.is_match(y) {
                    continue;
                }
                let the_word: Word = y.into();
                let z = if x.len() > 2 {
                    x[2].trim().replace(",", "").parse::<usize>().unwrap()
                } else {
                    1
                };
                let mut the_entry = todo.entry(the_word.rhyme()).or_insert(vec![]);
                the_entry.push(Entry {
                    word: &the_word,
                    freq: z,
                });
                // if word.rhymes_with(&the_word, contain_assonants) {
                //     let z = x[2].trim().replace(",", "").parse::<u32>().unwrap();
                //     let cl = the_word.syllables.len();
                //     match recs.get_mut(&cl) {
                //         Some(v) => {
                //             v.push(y.to_string());
                //         }
                //         None => {
                //             recs.insert(cl, vec![y.to_string()]);
                //         }
                //     }
                //     if z < threshold {
                //         break;
                //     }
                //     // println!("{}", y);
                // }
            }
        }
    }

    let res = todo
        .get(&word.rhyme())
        .unwrap()
        .iter()
        .map(|e| e.word.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("{:?}", res);

    if looping {
        loop {
            print!("Your word: ");
            io::stdout().flush().unwrap();
            let mut x = String::with_capacity(128);
            io::stdin().read_line(&mut x).expect("Error reading input");
            let x = x.trim();
            let w: Word = x.into();
            let res = todo.get(&w.rhyme());

            match res {
                Some(v) => {
                    let res = v
                        .iter()
                        .map(|e| e.word.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    println!("{:?}", res);
                }
                None => println!("No rhyming word found"),
            }
        }
    }

    // let mut s = recs.keys().map(|x| x.to_owned()).collect::<Vec<usize>>();
    // s.sort();

    // for i in s.iter() {
    //     println!(
    //         "Words with {} syllable{}:",
    //         i,
    //         if i == &1 { "" } else { "s" }
    //     );
    //     println!("{}\n", recs.get(i).unwrap().join(", "));
    // }

    // for (k, v) in recs.iter() {
    //     println!("Words with {} syllable{}:", k, if k == &1 { "" } else { "s"});
    //     println!("{}", v.join(", "));
    // }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Entry<'a> {
    pub word: &'a Word,
    pub freq: usize,
}

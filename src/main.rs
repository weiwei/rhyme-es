use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use syllabize_es::Word;

fn main() {
    let reg = Regex::new("^[a-záéíóúñ]+$").unwrap();
    let matches = App::new("rhyme")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("WORD").value_name("WORD").takes_value(true))
        .arg(
            Arg::with_name("RESOURCE")
                .value_name("RESOURCE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("THRESHOLD")
                .value_name("THRESHOLD")
                .takes_value(true),
        )
        .get_matches();

    let word: Word = matches.value_of("WORD").unwrap().into();
    let rm = word.rhyme();

    let resource = matches.value_of("RESOURCE").unwrap();
    let threshold = matches
        .value_of("THRESHOLD")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // File hosts must exist in current path before this produces output
    let path = Path::new(resource);
    let mut recs: HashMap<usize, Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let x: Vec<&str> = ip.split('\t').collect();
                let y = x[1];
                if y.chars().count() < 2 || !reg.is_match(y) {
                    continue;
                }
                let the_word: Word = y.into();
                let rm2 = the_word.rhyme();
                if rm == rm2 {
                    let z = x[2].trim().replace(",", "").parse::<u32>().unwrap();
                    let cl = the_word.syllables.len();
                    match recs.get_mut(&cl) {
                        Some(v) => {
                            v.push(y.to_string());
                        }
                        None => {
                            recs.insert(cl, vec![y.to_string()]);
                        }
                    }
                    if z < threshold {
                        break;
                    }
                    // println!("{}", y);
                }
            }
        }
    }

    let mut s = recs.keys().map(|x| x.to_owned()).collect::<Vec<usize>>();
    s.sort();

    for i in s.iter() {
        
        println!("Words with {} syllable{}:", i, if i == &1 { "" } else { "s"});
        println!("{}\n", recs.get(i).unwrap().join(", "));
    }

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

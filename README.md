# rhyme-es

Find rhyming words in Spanish.

## Resources

Already prepared in the repo, but in case I forget:

Frequency list: [RAE Corpus](https://corpus.rae.es/lfrecuencias.html), `./CREA_total.TXT`. Note that the list is iso-8859-1 encoded, It ahs to be converted to UTF8.

Dictionary: hunspell dictionaries from https://github.com/wooorm/dictionaries, extracted to word list with `unmunch`, `./words.txt`.

## Usage

```sh
cargo run --release --bin init
cargo run --release --bin query
```

## Features

* [] Order by number of syllables
* [x] Order by frequency
* [] Ability to choose dictionary
* [] Speed
* [] Metric syllables
* [] Assonant rhymes
* [] homophonous consonants, namely `ll` and `y`

And the assorted functionalities that exist in https://buscapalabras.com.ar/rimas.php

## Performance

`query` binary takes 100ms to be ready. Memory footprint stays below 50M.

## Used

`syllabize-es` crate, my own dog food.
`serde` with `bincode`, good stuff.
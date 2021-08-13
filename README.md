# rhyme-es

Find rhyming words in Spanish.

## Features

* Ability to specify range of words, e.g., 1000, 10000, or more.
* Order by number of syllables
* Order by frequency

## Implementation

The word list is from the [RAE Corpus](https://corpus.rae.es/lfrecuencias.html).
Note that the list is iso-8859-1 encoded, I've converted them to UTF8.

0. Hit cache for the word. if found, output.
1. Search the list, find the rhyming words, ordered by freq.
2. Syllabize the words and group them.
3. Cache the words in db (Q: what DB?)

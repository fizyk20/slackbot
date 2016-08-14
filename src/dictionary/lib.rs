use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn to_4u8(x: u32) -> [u8; 4] {
    let mut result = [0; 4];
    result[0] = (x & 0xFF) as u8;
    result[1] = ((x >> 8) & 0xFF) as u8;
    result[2] = ((x >> 16) & 0xFF) as u8;
    result[3] = ((x >> 24) & 0xFF) as u8;
    result
}

fn to_u32(x: &[u8]) -> Option<u32> {
    if x.len() < 4 {
        return None;
    }
    Some(x[0] as u32 + ((x[1] as u32) << 8) + ((x[2] as u32) << 16) + ((x[3] as u32) << 24))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Word {
    Start1,
    Start2,
    Word(u32),
    End
}

impl Word {
    pub fn into_bytes(&self) -> [u8; 5] {
        let mut result = [0; 5];
        match *self {
            Word::Start1 => { result[0] = 1; },
            Word::Start2 => { result[0] = 2; },
            Word::End => { result[0] = 0xFF; },
            Word::Word(i) => {
                let i_bytes = to_4u8(i);
                for j in 0..4 {
                    result[j+1] = i_bytes[j];
                }
            }
        }
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Word> {
        if bytes.len() < 5 {
            return None;
        }

        match bytes[0] {
            0 => Some(Word::Word(to_u32(&bytes[1..5]).unwrap())),
            1 => Some(Word::Start1),
            2 => Some(Word::Start2),
            0xFF => Some(Word::End),
            _ => None
        }
    }
}

pub type Entry = (Word, Word);

pub struct Result {
    word: Word,
    chance: u32
}

pub struct Dictionary {
    words: Vec<String>,
    dict: HashMap<Entry, Vec<Result>>
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            words: Vec::new(),
            dict: HashMap::new()
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        // push number of words
        result.extend_from_slice(&to_4u8(self.words.len() as u32));
        // push each word preceded by its length
        for word in (&self.words).into_iter() {
            let bytes = word.as_bytes();
            result.extend_from_slice(&to_4u8(bytes.len() as u32));
            result.extend_from_slice(bytes);
        }
        // write dict
        // first, the number of entries
        result.extend_from_slice(&to_4u8(self.dict.len() as u32));
        // now the entries
        for key in self.dict.keys() {
            // first, the key
            result.extend_from_slice(&key.0.into_bytes());
            result.extend_from_slice(&key.1.into_bytes());
            // second, possible results
            let data = self.dict.get(key).unwrap();
            // vec length
            result.extend_from_slice(&to_4u8(data.len() as u32));
            // and entries
            for entry in data.into_iter() {
                result.extend_from_slice(&entry.word.into_bytes());
                result.extend_from_slice(&to_4u8(entry.chance));
            }
        }

        result
    }

    fn from_bytes(bytes: &[u8]) -> Option<Dictionary> {
        let num_words = to_u32(&bytes[0..4]).unwrap();
        let mut words = Vec::new();
        // read words
        let mut cursor = 4;
        for _ in 0..num_words {
            let word_length = to_u32(&bytes[cursor..cursor+4]).unwrap() as usize;
            cursor += 4;
            if let Ok(word) = ::std::str::from_utf8(&bytes[cursor..cursor+word_length]) {
                words.push(word.to_string());
            }
            else {
                return None;
            }
            cursor += word_length;
        }
        // read entry map
        let num_entries = to_u32(&bytes[cursor..cursor+4]).unwrap();
        let mut hashmap = HashMap::new();
        cursor += 4;
        for _ in 0..num_entries {
            // first entry word
            let word1;
            if let Some(word) = Word::from_bytes(&bytes[cursor..cursor+5]) {
                word1 = word;
            }
            else {
                return None;
            }
            cursor += 5;
            // second entry word
            let word2;
            if let Some(word) = Word::from_bytes(&bytes[cursor..cursor+5]) {
                word2 = word;
            }
            else {
                return None;
            }
            cursor += 5;
            let num_results = to_u32(&bytes[cursor..cursor+4]).unwrap();
            cursor += 4;
            let mut results = Vec::new();
            for _ in 0..num_results {
                let word;
                if let Some(w) = Word::from_bytes(&bytes[cursor..cursor+5]) {
                    word = w;
                }
                else {
                    return None;
                }
                cursor += 5;
                let chance = to_u32(&bytes[cursor..cursor+4]).unwrap();
                cursor += 4;
                results.push(Result { word: word, chance: chance });
            }
            hashmap.insert((word1, word2), results);
        }
        Some(Dictionary {
            words: words,
            dict: hashmap
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = try!(File::create(path));
        let bytes = self.to_bytes();
        try!(file.write(&bytes));
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Dictionary> {
        let mut file = try!(File::open(path));
        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));
        if let Some(dict) = Dictionary::from_bytes(&bytes) {
            Ok(dict)
        }
        else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid dictionary input"))
        }
    }
}


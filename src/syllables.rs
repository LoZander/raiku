use std::{collections::HashSet, ops::Range};
use itertools::Itertools;


//const CONSONANTS: [char; 18] = ['b','c','d','f','h','k','l','m','n','p','q','r','s','t','v','w','x','z'];
const VOWELS: [char; 6] = ['a','e','i','o','u','y'];

type Syllable = Range<usize>;

#[derive(Clone)]
#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub syllables: Vec<Syllable>,
}

impl From<String> for Word {
    fn from(value: String) -> Self {
        Self::from(value)
    }
}

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        Self::from(value)
    }
}

impl From<char> for Word {
    fn from(value: char) -> Self {
        Self::from(value)
    }
}

impl From<Box<str>> for Word {
    fn from(value: Box<str>) -> Self {
        Self::from(value)
    }
}

impl From<&mut str> for Word {
    fn from(value: &mut str) -> Self {
        Self::from(value)
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let syllables = self.syllables.iter()
                .map(|x| self.text[x.clone()].to_string())
                .reduce(|acc, x| format!("{}, {}", acc, x))
                .unwrap_or_default();
        write!(f, "{} [{}]", self.text, syllables)
    }
}

impl Word {
    pub fn from<T: Into<String>>(from: T) -> Self {
        let s: String = from.into();
        let vowels = HashSet::from(VOWELS);

        if s.len() == 1 && vowels.contains(&s.chars().next().unwrap()) {
            return Word{text: s.clone(), syllables: vec![0..s.len()]}
        }

        let (syllables, _ , _) = s.to_lowercase()
            .chars()
            .enumerate()
            .tuple_windows()
            .fold((vec![], 0, 1), |(mut acc, i, j), ((_,elem), (m,next))| {
                let is_vowel = vowels.contains(&elem);
                let next_is_vowel = vowels.contains(&next);
                let next_is_last = m == s.len() - 1;
                match (is_vowel, next_is_vowel, next_is_last) {
                    (_, _, true) => {acc.push(i..j + 1); (acc, 0, 0)},
                    (true, false, _) => {acc.push(i..j); (acc, j, j + 1)}
                    _ => (acc, i, j + 1)
            }});

        Word {text: s.clone(), syllables}.trim_syllables(|text,_,b| !text[b.clone()].chars().any(|x| vowels.contains(&x)))
                                            .trim_syllables(|text,a,b| 
                                                text[b.clone()].ends_with('e') & 
                                                !text[a.start..b.end].ends_with("aide") &
                                                !text[b.clone()].ends_with("ie"))
    }

    fn trim_syllables<F>(&self, predicate: F) -> Self 
        where F: Fn(String, &Syllable, &Syllable) -> bool
    {
        let text = self.text.clone();
        let syllables = match &self.syllables[..] {
            [x @ .., a, b] if predicate(text.clone(), a, b) => {
                let mut y = x.to_vec();
                y.push(a.start..b.end);
                y
            }
            _ => self.syllables.clone()
        };
        Word {
            text,
            syllables,
        }
    }
}

pub struct Sentence(Vec<Word>);
impl Sentence {
    pub fn new() -> Self {
        Sentence(Vec::new())
    }

    pub fn push(&mut self, item: Word) {
        self.0.push(item)
    }

    pub fn from<T: Into<String>>(value: T) -> Self {
        let s: String = value.into();
        let collection = s.split_ascii_whitespace();
        Self::from_collection(collection)
    }

    pub fn from_collection<I>(value: I) -> Self 
        where 
            I: IntoIterator,
            I::Item: Into<Word>,
    {
        Sentence(value.into_iter().map(<I as IntoIterator>::Item::into).collect())
    }
}

impl From<String> for Sentence {
    fn from(value: String) -> Self {
        Self::from(value)
    }
}

impl From<&str> for Sentence {
    fn from(value: &str) -> Self {
        Self::from(value)
    }
}

impl From<Box<str>> for Sentence {
    fn from(value: Box<str>) -> Self {
        Self::from(value)
    }
}

impl From<&mut str> for Sentence {
    fn from(value: &mut str) -> Self {
        Self::from(value)
    }
}

impl From<Vec<Word>> for Sentence {
    fn from(value: Vec<Word>) -> Self {
        Self::from_collection(value)
    }
}

impl From<&[Word]> for Sentence {
    fn from(value: &[Word]) -> Self {
        Self::from_collection(value.to_owned())
    }
}

impl From<Vec<String>> for Sentence {
    fn from(value: Vec<String>) -> Self {
        Self::from_collection(value)
    }
}

impl From<&[String]> for Sentence {
    fn from(value: &[String]) -> Self {
        Self::from_collection(value.to_owned())
    }
}

impl From<Vec<&str>> for Sentence {
    fn from(value: Vec<&str>) -> Self {
        Self::from_collection(value)
    }
}

impl From<&[&str]> for Sentence {
    fn from(value: &[&str]) -> Self {
        Self::from_collection(value.to_owned())
    }
}

impl From<Vec<Box<str>>> for Sentence {
    fn from(value: Vec<Box<str>>) -> Self {
        Self::from_collection(value)
    }
}

impl From<&[Box<str>]> for Sentence {
    fn from(value: &[Box<str>]) -> Self {
        Self::from_collection(value.to_owned())
    }
}

impl From<Vec<&mut str>> for Sentence {
    fn from(value: Vec<&mut str>) -> Self {
        Self::from_collection(value)
    }
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|x| x.to_string()).reduce(|acc,x| format!("{}, {}", acc, x)).unwrap_or_default())
    }
}

impl IntoIterator for Sentence {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Word> for Sentence {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        let mut c = Sentence::new();

        for i in iter {
            c.0.push(i);
        }

        c
    }
}
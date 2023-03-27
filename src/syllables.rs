use std::{collections::HashSet, ops::Range};
use itertools::Itertools;


//const CONSONANTS: [char; 18] = ['b','c','d','f','h','k','l','m','n','p','q','r','s','t','v','w','x','z'];
const VOWELS: [char; 6] = ['a','e','i','o','u','y'];

type Syllable = Range<usize>;

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub syllables: Vec<Syllable>,
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

pub fn syllables<'a, T: Into<String>> (input: T) -> Sentence {
    let s: String = input.into();
    s.split_ascii_whitespace()
     .into_iter()
     .map(str::to_string)
     .map(syllables_word)
     .collect()
}

fn syllables_word<T: Into<String>> (input: T) -> Word {
    let s: String = input.into();
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

    Word {text: s.clone(), syllables}.trim_syllables(|text,a,b| text[b.clone()].ends_with('e') & !text[a.start..b.end].ends_with("ide"))
                                     .trim_syllables(|text,_,b| !text[b.clone()].chars().any(|x| vowels.contains(&x)))
}
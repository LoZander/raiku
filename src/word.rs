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

#[cfg(test)]
mod tests {
    use crate::word::Word;

    #[test]
    fn made_is_1_syllable_test() {
        let actual = Word::from("made").syllables.len();

        assert_eq!(actual, 1)
    }

    #[test]
    fn haide_is_2_syllables_test() {
        let actual = Word::from("haide").syllables.len();
        assert_eq!(actual, 2)
    }

    #[test]
    fn hide_is_1_syllable_test() {
        let actual = Word::from("hide").syllables.len();
        assert_eq!(actual, 1)
    }

    #[test]
    fn jamie_is_2_syllables_test() {
        let actual = Word::from("jamie").syllables.len();
        assert_eq!(actual, 2)
    }

    #[test]
    fn fly_is_1_syllable_test() {
        let actual = Word::from("fly").syllables.len();
        assert_eq!(actual, 1)
    }

    #[test]
    fn flywheel_is_2_syllables_test() {
        let actual = Word::from("flywheel").syllables.len();
        assert_eq!(actual, 2)
    }
}
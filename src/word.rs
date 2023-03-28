use std::{collections::HashSet, ops::Range};
use itertools::Itertools;

const VOWELS: [char; 6] = ['a','e','i','o','u','y'];

type Syllable = Range<usize>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Word {
    pub text: String,
    pub syllables: Vec<Syllable>,
}

impl From<String> for Word {
    fn from(value: String) -> Self {
        Self::from(value)
    }
}

impl From<Word> for String {
    fn from(value: Word) -> Self {
        value.text
    }
}

impl<T: From<char>> From<Word> for Vec<T> {
    fn from(value: Word) -> Self {
        value.text.chars().map(char::into).collect()
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

impl From<Word> for Box<str> {
    fn from(value: Word) -> Self {
        value.text.into_boxed_str()
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

        Word {text: s, syllables}.trim_syllables(|text,_,b| !text[b.clone()].chars().any(|x| vowels.contains(&x)))
                                            .trim_syllables(|text,a,b| 
                                                text[b.clone()].ends_with('e') & 
                                                !text[a.start..b.end].ends_with("aide") &
                                                !text[b.clone()].ends_with("ie"))
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn syllable_count(&self) -> usize {
        self.syllables.len()
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

    #[test]
    fn word_len_test() {
        let word: Word = "longevity".into();
        assert_eq!(word.len(), 9)
    }

    #[test]
    fn word_syllable_count() {
        let word: Word = "wilderness".into();
        assert_eq!(word.syllable_count(), 3)
    }

    #[test]
    fn word_from_string_test() {
        let word: Word = String::from("wilderness").into();
        assert_eq!(word.text, "wilderness");
        assert_eq!(word.syllable_count(), 3)
    }

    #[test]
    fn word_from_str_ref_test() {
        let word: Word = "longevity".into();
        assert_eq!(word.text, "longevity");
        assert_eq!(word.syllable_count(), 4)
    }

    #[test]
    fn word_from_char_test() {
        let word: Word = 'i'.into();
        assert_eq!(word.text, "i");
        assert_eq!(word.syllable_count(), 1)
    }

    #[test]
    fn word_from_boxed_str() {
        let boxed: Box<str> = String::from("wilderness").into_boxed_str();
        let word: Word = boxed.into();

        assert_eq!(word.text, "wilderness");
        assert_eq!(word.syllable_count(), 3)
    }

    #[test]
    fn word_from_mut_str() {
        let mut string = String::from("wilderness");
        let mut_str: &mut str = string.as_mut_str();
        let word: Word = mut_str.into();

        assert_eq!(word.text, "wilderness");
        assert_eq!(word.syllable_count(), 3)
    }
}
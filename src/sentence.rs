use std::{slice::SliceIndex, ops::Index};

use crate::word::Word;

pub struct Sentence(Vec<Word>);
impl Sentence {
    pub fn new() -> Self {
        Sentence(Vec::new())
    }

    pub fn push(&mut self, item: Word) {
        self.0.push(item)
    }

    pub fn get<I: SliceIndex<[Word]>>(&self, index: I) -> Option<&I::Output>  {
        self.0.get(index)
    }

    fn from<T: Into<String>>(value: T) -> Self {
        let s: String = value.into();
        let collection = s.split_ascii_whitespace();
        Self::from_collection(collection)
    }

    fn from_collection<I>(value: I) -> Self 
        where 
            I: IntoIterator,
            I::Item: Into<Word>,
    {
        Sentence(value.into_iter().map(<I as IntoIterator>::Item::into).collect())
    }

    pub fn word_count(self) -> usize {
        self.0.len()
    }

    pub fn syllable_count(self) -> usize {
        self.0.iter().map(|x| x.syllables.len()).sum()
    }

}

impl Default for Sentence {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Sentence {
    type Output = Word;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<String> for Sentence {
    fn from(value: String) -> Self {
        Self::from(value)
    }
}

impl From<Sentence> for String {
    fn from(value: Sentence) -> Self {
        value.into_iter().map(Word::into).reduce(|acc,x| format!("{} {}", acc, x)).unwrap_or_default()
    }
}

impl<T: From<Word>> From<Sentence> for Vec<T> {
    fn from(value: Sentence) -> Self {
        value.into_iter().map(Word::into).collect()
    }
}

impl<T: Into<Word>> From<Vec<T>> for Sentence {
    fn from(value: Vec<T>) -> Self {
        Self::from_collection(value)
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

impl From<Sentence> for Box<str> {
    fn from(value: Sentence) -> Self {
        let string: String = value.into();
        string.into_boxed_str()
    }
}

impl From<&mut str> for Sentence {
    fn from(value: &mut str) -> Self {
        Self::from(value)
    }
}

impl<const N: usize, T: Into<Word> + Clone> From<[T; N]> for Sentence {
    fn from(value: [T; N]) -> Self {
        value.to_vec().into()
    }
}

impl<T: Into<Word> + Clone> From<&[T]> for Sentence {
    fn from(value: &[T]) -> Self {
        value.to_vec().into()
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

#[cfg(test)]
mod tests {
    use crate::word::Word;

    use super::Sentence;

    #[test]
    fn sentence_from_string_test() {
        let sen: Sentence = String::from("can savannah have the greenest eyes?").into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_words_test() {
        let words: Vec<Word> = vec!["can".into(), "savannah".into(), "have".into(), "the".into(), "greenest".into(), "eyes?".into()];
        let sen: Sentence = words.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_str_test() {
        let sen: Sentence = "can savannah have the greenest eyes?".into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_box_str_test() {
        let boxed: Box<str> = "can savannah have the greenest eyes?".into();
        let sen: Sentence = boxed.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_mut_str_test() {
        let mut string = "can savannah have the greenest eyes?".to_string();
        let mut_str = string.as_mut_str();
        let sen: Sentence = mut_str.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_string_vec_test() {
        let strings: Vec<String> = vec![
            "can".into(), "savannah".into(), 
            "have".into(), "the".into(), 
            "greenest".into(), "eyes?".into()];
        let sen: Sentence = strings.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_str_vec_test() {
        let strs: Vec<&str> = vec!["can", "savannah", "have", "the", "greenest", "eyes?"];
        let sen: Sentence = strs.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_str_array_ref_test() {
        let strs: &[&str] = &["can", "savannah", "have", "the", "greenest", "eyes?"];
        let sen: Sentence = strs.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }

    #[test]
    fn sentence_from_str_array_test() {
        let strs = ["can", "savannah", "have", "the", "greenest", "eyes?"];
        let sen: Sentence = strs.into();
        assert_eq!(sen[0], "can".into());
        assert_eq!(sen[1], "savannah".into());
        assert_eq!(sen[2], "have".into());
        assert_eq!(sen[3], "the".into());
        assert_eq!(sen[4], "greenest".into());
        assert_eq!(sen[5], "eyes?".into())
    }
}
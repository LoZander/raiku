use crate::word::Word;

pub struct Sentence(Vec<Word>);
impl Sentence {
    pub fn new() -> Self {
        Sentence(Vec::new())
    }

    pub fn push(&mut self, item: Word) {
        self.0.push(item)
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

impl From<Vec<String>> for Sentence {
    fn from(value: Vec<String>) -> Self {
        Self::from_collection(value)
    }
}

impl From<Vec<Word>> for Sentence {
    fn from(value: Vec<Word>) -> Self {
        Sentence(value)
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

impl From<&[Word]> for Sentence {
    fn from(value: &[Word]) -> Self {
        Self::from_collection(value.to_owned())
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

#[cfg(test)]
mod tests {
    use crate::word::Word;

    use super::Sentence;

    /* #[test]
    fn sentence_from_string_test() {
        let sen: Sentence = String::from("can savannah have the greenest eyes?").into();
        assert!(matches!(strs,
            &["can", "savannah", "have", "the", "greenest", "eyes?"]
        ))
    } */
}
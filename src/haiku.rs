use crate::sentence::Sentence;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Haiku(pub String, pub String, pub String);

impl std::fmt::Display for Haiku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.0, self.1, self.2)
    }
}

impl Haiku {
    pub fn from<T: Into<Sentence>>(value: T) -> Option<Haiku> {
        let words: Sentence = value.into();

        let (sen1, rest) = verse(words, 5)?;
        let (sen2, rest) = verse(rest, 7)?;
        let (sen3, rest) = verse(rest, 5)?;

        match rest.word_count() {
            0 => Some(Haiku(sen1.trim().into(),sen2.trim().into(),sen3.trim().into())),
            _ => None
        }
    }
}

fn verse(words: Sentence, n: u32) -> Option<(String, Sentence)> {
    let (verse, rest, _) = words.into_iter()
         .fold(Some((String::new(), Sentence::new(), 0)), |option, word| {
            option.and_then(|(acc, mut rest, count)| {
                let syllables = count + word.syllables.len() as u32;
                match (count, syllables) {
                    (c, s) if c < n && n < s => None,
                    (c, _) if c == n => {
                        rest.push(word);
                        Some((acc, rest, c))
                    }
                    (_, s) => Some((acc + " " + &word.text, rest, s)),
                }
            })
        })?;
    Some((verse,rest))
}

#[cfg(test)]
mod tests {
    use crate::haiku::{Haiku};

    #[test]
    fn haiku_test() {
        let Haiku(sen1, sen2, sen3) = Haiku::from("this is a test of the way haiku is made here that is kind of cool").unwrap();

        assert_eq!(sen1, "this is a test of");
        assert_eq!(sen2, "the way haiku is made here");
        assert_eq!(sen3, "that is kind of cool")
    }

    #[test]
    fn test_no_haiku_from_long_sentence() {
        let haiku = Haiku::from("this is a test of the way haiku is made here that is kind of cool bla bla bla");
        assert!(matches!(haiku, None))
    }
}
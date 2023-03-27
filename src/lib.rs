use std::fmt::Display;

use syllables::Sentence;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod syllables;

pub struct Haiku(String, String, String);

impl Display for Haiku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.0, self.1, self.2)
    }
}

pub fn haiku<T: Into<String>>(text: T) -> Option<Haiku> {
    let words = syllables::syllables(text);
    println!("{words}");
    let (sen1, words) = verse(words, 5)?;
    let (sen2, words) = verse(words, 7)?;
    let (sen3, _) = verse(words, 5)?;
    Some(Haiku(sen1,sen2,sen3))
}

fn verse(words: Sentence, n: u32) -> Option<(String, Sentence)> {
    let (verse, rest, _) = words.into_iter()
         .fold(Some((String::new(), Sentence::new(), 0)), |option, word| {
            option.and_then(|(acc, mut rest, count)| {
                let syllables = count + word.syllables.len() as u32;
                println!("count = {:?}, syllables = {:?}", count, word.syllables.iter().map(|x| word.text[x.clone()].to_string()).collect::<Vec<String>>());
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
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

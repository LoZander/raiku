use std::{collections::HashSet, ops::Range};
use itertools::Itertools;


const CONSONANTS: [char; 18] = ['b','c','d','f','h','k','l','m','n','p','q','r','s','t','v','w','x','z'];
const VOWELS: [char; 6] = ['a','e','i','o','u','y'];

type Syllable = Range<usize>;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Word {
    pub text: String,
    pub syllables: Vec<Syllable>,
}

pub fn syllables<'a, T: Into<String>> (input: T) -> Vec<Word> {
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
    let consonants = HashSet::from(CONSONANTS);

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
    let syllables = match &syllables[..] {
        [.., b] if s[b.clone()].ends_with("ie") => syllables,
        [x @ .., a, b] if s[b.clone()].ends_with('e') => {
            let mut y = x.to_vec();
            y.push(a.start..b.end);
            y
        }
        [x @ .., a, b] => {
            println!("x = {x:?}, a = {a:?}, b = {b:?}");
            println!("a = {:?}, b = {:?}", &s[a.clone()], &s[b.clone()]);
            syllables
        }
        _ => syllables
    };
    Word{text: s, syllables}
}
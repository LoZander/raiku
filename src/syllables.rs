use std::collections::HashSet;
use itertools::Itertools;


const CONSONANTS: [char; 18] = ['b','c','d','f','h','k','l','m','n','p','q','r','s','t','v','w','x','z'];
const VOWELS: [char; 6] = ['a','e','i','o','u','y'];

pub fn syllables<T: Into<String>> (input: T) -> u32 {
    input.into()
         .split_ascii_whitespace()
         .into_iter()
         .map(syllables_word)
         .fold(0,|acc,x| acc + x)
}

fn syllables_word (input: &str) -> u32 {
    let vowels = HashSet::from(VOWELS);
    let consonants = HashSet::from(CONSONANTS);

    if input.len() == 1 && vowels.contains(&input.chars().next().unwrap()) {
        return 1
    }
    
    input.to_lowercase()
        .chars()
        .enumerate()
        .tuple_windows()
        .fold(0, |acc, ((_,elem),(j,next))| {
            let is_vowel = vowels.contains(&elem);
            let next_is_last = j == input.len() - 1;
            let next_is_consonant = consonants.contains(&next);
            match (is_vowel, next_is_consonant, next_is_last) {
                (_, false, true) => acc + 1,
                (true, true, _) => acc + 1,
                _ => acc
            }
        })
}
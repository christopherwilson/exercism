use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let target_freqs_base = char_freqs(&word);
    let mut anagrams = HashSet::new();

    for possible_anagram in possible_anagrams {
        let candidate = possible_anagram.to_lowercase();
        if candidate.chars().count() != word.chars().count()
            || candidate == word {
                continue
        }

        let mut target_freqs = target_freqs_base.clone();
        let mut is_anagram = true;

        for letter in candidate.chars() {
            if let Some(count) = target_freqs.get_mut(&letter) {
                if *count == 0 {
                    is_anagram = false;
                    break
                } else {
                    *count -= 1;
                }
            } else {
                is_anagram = false;
                break
            }
        }
        if is_anagram {
            anagrams.insert(*possible_anagram);
        }
    }
    anagrams
}

fn char_freqs(word: &String) -> HashMap<char, usize> {
    let mut freqs = HashMap::new();
    for letter in word.chars() {
        freqs.entry(letter).and_modify(|count| *count += 1).or_insert(1);
    };
    freqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_freqs_hello() {
        let mut expected = HashMap::new();
        
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 2);
        expected.insert('o', 1);

        assert_eq!(expected, char_freqs(&"hello".to_string()));
    }
}

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! # closestmatch-rs
//! Inspired by closestmatch a golang library.
//! This library fuzzy matches an input string to a list of target strings.
extern crate rayon;

use std::collections::HashMap;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
/// The ```ClosestMatch``` struct stores informations about the dictionary of known words and the different sizes for the bags of words.
pub struct ClosestMatch {
    substrings: HashMap<String, HashSet<String>>,
    substring_sizes: Vec<usize>
}

#[derive(Debug)]
struct SplitWord {
    word: String,
    substrings: HashSet<String>
}

#[derive(Debug)]
struct ScoreValue {
    word: String,
    score: f32
}

fn split_word(word: String, sizes: &Vec<usize>) -> SplitWord {
    let mut substrings: HashSet<String> = HashSet::new();
    for size in sizes {
        if *size > word.len() {
            continue;
        }
        for x in 0..(word.len() - size + 1) {
            let sub = word[x..(x+size)].to_string().to_lowercase();
            substrings.insert(sub);
        }
    }
    return SplitWord{ word: word, substrings: substrings };
}

/// The function ```new``` takes a dictionary of known words with type ```Vec<String>``` and the 
/// different sizes of bag of words with type ```Vec<usize>```.
/// It returns a ClosestMatch object.
pub fn new(dictionary: Vec<String>, sizes: Vec<usize>) -> ClosestMatch {
    let mut substrings: HashMap<String, HashSet<String>> = HashMap::new();
    let splitwords: Vec<SplitWord> = dictionary.par_iter()
      .map( |possible| split_word(possible.to_lowercase(), &sizes))
      .collect();
    for splitword in splitwords  {
        substrings.insert(splitword.word, splitword.substrings);
    }
    return ClosestMatch{ substrings: substrings, substring_sizes: sizes };
}

fn evaluate(word_subs: &HashSet<String>, possible: String, possible_subs: &HashSet<String>) -> ScoreValue {
    let mut count = 0;
    let len_sum = word_subs.len() + possible_subs.len();
    for sub in word_subs {
        if possible_subs.contains(sub) {
            count += 1;
        }
    }
    let score = (count as f32) / (len_sum as f32);
    return ScoreValue{word: possible, score: score}; 
}

fn max_score(a: ScoreValue, b: ScoreValue) -> ScoreValue {
    if a.score <= b.score {
        return b;
    }
    return a;
}
 
impl ClosestMatch {
     /// The function ```get_closest``` takes a word with type ```String``` and
     /// returns the closest word in the dictionary of known words.
     pub fn get_closest(&self, word: String) -> Option<String> {
         let word_subs = split_word(word, &self.substring_sizes).substrings;
         let best = self.substrings.par_iter()
            .map(|(possible, possible_subs)| evaluate(&word_subs, possible.to_lowercase(), possible_subs))
            .reduce_with(|a, b| max_score(a, b));
         match best {
             Some(expr) => Some(expr.word),
             None => None,
         }
     }
}

#[cfg(test)]
mod tests {
    use new;
    #[test]
    fn it_works() {
        let cm = new(["hello".to_string(), "bullo".to_string(), "hello world".to_string()].to_vec(), [1,2,3].to_vec());
        let closest = cm.get_closest("hlo".to_string());
        println!("{:?}", closest);
    }
}

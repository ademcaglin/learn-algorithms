use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn diff(a: String, b: String) -> String {
  let mut a_arr: Vec<char> = a.chars().collect();
  let mut b_arr: Vec<char> = b.chars().collect();
  for a_c in a.chars() {
    if b_arr.iter().any(|&x| x == a_c) {
      a_arr
        .iter()
        .position(|&n| n == a_c)
        .map(|e| a_arr.remove(e));
      b_arr
        .iter()
        .position(|&n| n == a_c)
        .map(|e| b_arr.remove(e));
    }
  }
  return a_arr.iter().collect();
}

fn find_anagrams(phrase: String, words: Vec<String>, candidate: Vec<String>) {
  let filtered: Vec<String> = words
    .into_iter()
    .filter(|w| diff(w.to_string(), phrase.to_string()).len() == 0)
    .collect();

  for w in &filtered {
    let mut new_candidate: Vec<String> = candidate.clone();
    new_candidate.push(w.to_string());
    let new_phrase = diff(phrase.to_string(), w.to_string());
    if new_phrase == "" {
      let digest = md5::compute(new_candidate.join(" ").as_bytes());
      if format!("{:x}", digest) == "e4820b45d2277f3844eac66c903e84be" {
        println!("Easy secret: {:?}", new_candidate);
      }
    } else {
      find_anagrams(new_phrase, filtered.clone(), new_candidate)
    }
  }
}

fn main() {
  let mut words: Vec<String> = Vec::new();
  let file = File::open("words").unwrap();
  let reader = BufReader::new(file);
  let phrase = "poultryoutwitsants";
  for line in reader.lines() {
    let word = line.unwrap();
    words.push(word);
  }
  words = words
    .into_iter()
    .filter(|w| diff(w.to_string(), phrase.to_string()).len() == 0)
    .collect();
  words = words
    .into_iter()
    .filter(|w| w.to_string().len() > 4)
    .collect();

  let candidate: Vec<String> = vec![];
  let now = Instant::now();
  find_anagrams(phrase.to_string(), words, candidate);
  println!("Elapsed Time: {}", now.elapsed().as_secs());
}

#[test]
fn test_diff() {
  let found = diff("aaabacc".to_string(), "aabd".to_string());
  assert!(found == "aacc");
}

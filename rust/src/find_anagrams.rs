use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

fn diff(a: &String, b: &String) -> String{
  let mut a_arr : Vec<char> =  a.chars().collect();
  let mut b_arr : Vec<char> =  b.chars().collect();
  for a_c in a.chars(){
    if b_arr.iter().any(|&x| x == a_c){
      a_arr.iter()
       .position(|&n| n == a_c)
       .map(|e| a_arr.remove(e));
      b_arr.iter()
       .position(|&n| n == a_c)
       .map(|e| b_arr.remove(e));
    }
  }
  return a_arr.iter().collect();
}

fn find_anagrams(phrase: String, words: Vec<String>, candidate: Vec<String>){
  let filtered: Vec<String> = words.into_iter()
    .filter(|w| diff(w, &phrase).len() == 0)
    .collect();
  
  for w in &filtered{
    let mut new_candidate: Vec<String> = candidate.clone();
    new_candidate.push(w.to_string());
    let new_phrase = diff(&phrase, w);
    if new_phrase == ""{
      //println!("{:?}", new_candidate);
    }
    else{
      let new_words = filtered.clone(); 
      find_anagrams(new_phrase.clone(), new_words, new_candidate)
    }
  }
}
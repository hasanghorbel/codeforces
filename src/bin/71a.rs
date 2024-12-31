#![allow(unused)]

use std::io::stdin;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap()
}

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec:Vec<char> = input.trim().chars().collect();
    return vec;
}
fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}

fn solve() {
    let word = take_string();
    let l = word.len();
    if l <= 10 {
        println!("{}", to_string(word));
    } else {
        println!("{}{}{}", word[0], l-2, word[l-1])
    }
    
}


pub fn main() {
    let t = take_int();
    for _ in 0..t { solve(); }
}
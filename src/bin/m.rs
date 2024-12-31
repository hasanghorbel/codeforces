#![allow(unused)]

use std::{collections::HashSet, io::stdin};

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap()
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec:Vec<char> = input.trim().chars().collect();
    return vec;
}
fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
fn operator(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
  let mut v = HashSet::new();
  for a in a {
    for b in &b {
      for i in [a+b, a*b, a-b, a/b, b/a] {
        if i > 0 {
          v.insert(i);
        }
      }
    }
  }
  println!("{:?}", v);
  return Vec::from_iter(v);
}
fn solve(d: i32) {
  for a in 1..=100 {
    for b in (a+1)..=99 {
      for c in (b+1)..=98 {
        println!("{} {} {}", a, b, c);
        if !operator(operator(vec![a as i32],vec![b as i32]),vec![c as i32]).contains(&d) {
          return;
        }
      }
    }
  }
}


pub fn main() {
    let t = take_int();
    solve(t as i32);
}
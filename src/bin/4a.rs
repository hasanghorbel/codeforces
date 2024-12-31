#![allow(unused)]

use std::io;

pub fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let w = input.trim().parse::<i32>().unwrap();
  println!("{}", if w >= 4 && w % 2 == 0 { "YES" } else { "NO" });
}
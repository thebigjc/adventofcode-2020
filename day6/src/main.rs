static INPUT: &str = include_str!("../day6.txt");
use itertools::Itertools;
use reduce::Reduce;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let groups: Vec<&str> = INPUT.split("\n\n").collect();

    let mut cnt_one = 0;
    let mut cnt_two = 0;

    for g in groups {
        cnt_one += g
            .chars()
            .filter(|x| x.is_ascii_alphabetic())
            .unique()
            .count();

        cnt_two += g
            .split_ascii_whitespace()
            .map(|x| HashSet::<char>::from_iter(x.chars()))
            .reduce(|a, b| &a & &b)
            .unwrap()
            .len();
    }

    println!("{}", cnt_one);
    println!("{}", cnt_two);
}

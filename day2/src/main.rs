static INPUT: &str = include_str!("../day2.txt");
#[macro_use] extern crate lazy_static;

use regex::Regex;

fn is_valid_one(min: usize, max: usize, c: char, pass: &String) -> bool {
    let cnt = pass.matches(c).count();

    if cnt >= min && cnt <= max {
        true
    } else {
        false
    }
}

fn is_valid_two(a: usize, b: usize, c: char, pass: &String) -> bool {
    let mut cnt = 0;

    if pass.chars().nth(a-1).unwrap() == c {
        cnt+=1;
    }

    if pass.chars().nth(b-1).unwrap() == c {
        cnt+=1;
    }

    cnt == 1
}

fn parse_line(line: &str) -> (usize, usize, char, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    }

    let cap = RE.captures(line).unwrap();
    let min : usize = cap[1].parse().unwrap();
    let max : usize = cap[2].parse().unwrap();
    let c : char = cap[3].parse().unwrap();
    let pass = &cap[4];

    (min, max, c, pass.to_string())
} 

fn main() {
    let mut cnt_one = 0;
    let mut cnt_two = 0;

    for l in INPUT.lines() {
        let (min, max, c, pass) = parse_line(l);

        if is_valid_one(min, max, c, &pass) {
            cnt_one += 1;
        }
        if is_valid_two(min, max, c, &pass) {
            cnt_two += 1;
        }  
    }

    println!("{}", cnt_one);
    println!("{}", cnt_two);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let (min, max, c, pass) = parse_line("1-3 a: abcde");
        assert_eq!(is_valid_one(min, max, c, &pass), true);
        let (min, max, c, pass) = parse_line("1-3 b: cdefg");
        assert_eq!(is_valid_one(min, max, c, &pass), false);
        let (min, max, c, pass) = parse_line("2-10 c: cccccccccc");
        assert_eq!(is_valid_one(min, max, c, &pass), true);
    }

    #[test]
    fn test_two() {
        let (min, max, c, pass) = parse_line("1-3 a: abcde");
        assert_eq!(is_valid_two(min, max, c, &pass), true);
        let (min, max, c, pass) = parse_line("1-3 b: cdefg");
        assert_eq!(is_valid_two(min, max, c, &pass), false);
        let (min, max, c, pass) = parse_line("2-10 c: cccccccccc");
        assert_eq!(is_valid_two(min, max, c, &pass), false);
    }
}
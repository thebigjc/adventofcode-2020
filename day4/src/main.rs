static INPUT: &str = include_str!("../day4.txt");
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;

fn is_valid_one(passport : &HashMap<String,String>) -> bool  {
    let valid_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for k in valid_keys.iter() {
        if !passport.contains_key(&k.to_string()) {
            return false;
        }
    }

    true
}

fn is_valid_year(year : Option<&String>, min: usize, max: usize) -> bool {
    match year {
        None => false,
        Some(x) => { 
            let a = x.parse::<usize>();
            match a {
                Result::Ok(v) => v >= min && v <= max,
                Result::Err(_) => false 
            }
        }
    }
}

fn is_valid_height(height : Option<&String>) -> bool {
    /* 
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
*/
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    }

    match height {
        None => false,
        Some(x) => {
            let m = RE.captures(x);
            match m {
                None => false,
                Some(h) => {
                    let unit = h.get(2).unwrap().as_str();
                    let a = h.get(1).unwrap().as_str().parse::<usize>();
                    match a {
                        Result::Err(_) => false,
                        Result::Ok(b) => {
                            match unit {
                                "cm" => b >= 150 && b <= 193,
                                "in" =>  b >= 59 && b <= 76,
                                _ => false,
                            }
                        }
                    }
                }
            }
        }
    }

}

fn is_valid_hair_color(color : Option<&String>) -> bool {
    match color {
        None => false,
        Some(x) => {
            let b : Vec<char> = x.chars().collect();
            return b.len() == 7 && b[0] == '#' && b[1..].iter().all(|&x| x.is_ascii_hexdigit())
        }
    }
}

fn is_valid_eye_color(color : Option<&String>) -> bool {
    match color {
        None => false,
        Some(x) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|c| *c == x)
    }
}

fn is_valid_pid(pid : Option<&String>) -> bool {
    match pid {
        None => false,
        Some(x) => x.len() == 9 && x.chars().all(|c| c.is_ascii_digit())
    }
}

fn is_valid_two(passport : &HashMap<String, String>) -> bool {
    if !is_valid_year(passport.get("byr"), 1920, 2002) {
        return false;
    }

    if !is_valid_year(passport.get("iyr"), 2010, 2020) {
        return false;
    }

    if !is_valid_year(passport.get("eyr"), 2020, 2030) {
        return false;
    }

    if !is_valid_height(passport.get("hgt")) {
        return false;
    }

    if !is_valid_hair_color(passport.get("hcl")) {
        return false;        
    }

    if !is_valid_eye_color(passport.get("ecl")) {
        return false;        
    }

    if !is_valid_pid(passport.get("pid")) {
        return false;        
    }

    true
}


fn main() {
    let mut passport = HashMap::new();
    let re: Regex = Regex::new(r"^(([^:]+):(\S+)\s*)+$").unwrap();
    let mut cnt_one = 0;
    let mut cnt_two = 0;
    let passports : Vec<&str> = INPUT.split("\n\n").collect();


    for l in passports {
        for p in l.split_whitespace() {
            match re.captures(p) {
                Some(x) => passport.insert(x.get(2).unwrap().as_str().to_string(), x.get(3).unwrap().as_str().to_string()),
                None => None
            };
        }
    
        if is_valid_one(&passport) {
            cnt_one += 1;
        }
        if is_valid_two(&passport) {
            cnt_two += 1;
        }
        passport.clear();
    }

    println!("{}", cnt_one);
    println!("{}", cnt_two);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_year() {
        assert_eq!(is_valid_year(Some(&String::from("2002")), 1920, 2002), true);
        assert_eq!(is_valid_year(Some(&String::from("2003")), 1920, 2002), false);
    }

    #[test]
    fn test_height() {
        assert_eq!(is_valid_height(Some(&String::from("60in"))), true);
        assert_eq!(is_valid_height(Some(&String::from("190cm"))), true);
        assert_eq!(is_valid_height(Some(&String::from("190in"))), false);
        assert_eq!(is_valid_height(Some(&String::from("190"))), false);
    }

    #[test]
    fn test_hair() {
        assert_eq!(is_valid_hair_color(Some(&String::from("#123abc"))), true);
        assert_eq!(is_valid_hair_color(Some(&String::from("#123abz"))), false);
        assert_eq!(is_valid_hair_color(Some(&String::from("123abc"))), false);
    }

    #[test]
    fn test_eyes() {
        assert_eq!(is_valid_eye_color(Some(&String::from("brn"))), true);
        assert_eq!(is_valid_eye_color(Some(&String::from("wat"))), false);
    }

    #[test]
    fn test_pid() {
        assert_eq!(is_valid_pid(Some(&String::from("000000001"))), true);
        assert_eq!(is_valid_pid(Some(&String::from("0123456789"))), false);
    }
}
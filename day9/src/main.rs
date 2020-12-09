static INPUT: &str = include_str!("../day9.txt");
static WINDOW: usize = 25;
use std::iter::FromIterator;
use std::collections::VecDeque;
use std::collections::HashSet;

fn is_valid(x : usize, window: &VecDeque<usize>) -> bool {
    let hs : HashSet::<&usize> = HashSet::from_iter(window.iter());
    for l in hs.iter() {
        if **l > x {
            continue;
        }

        let delta = x-*l;

        if delta != **l && hs.contains(&delta) {
            return true;
        }
    }

    return false;
}

fn main() {
    let mut window = VecDeque::new();
    let mut all_numbers = Vec::new();
    let mut part1 = 0;

    for l in INPUT.lines().map(|x|x.parse::<usize>().unwrap()) {
        if part1 == 0 {
            if window.len() > WINDOW {
                window.pop_front(); 
                if !is_valid(l, &window) {
                    part1 = l;
                }
            }
            window.push_back(l);
        }
        all_numbers.push(l);
    }

    println!("{}", part1);

    for i in 0..all_numbers.len() {
        let mut sum = all_numbers[i];
        let mut min = sum;
        let mut max = sum; 

        for j in i+1..all_numbers.len() {
            let n = all_numbers[j]; 
            sum += n;

            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            if sum == part1 {
                println!("{}+{} = {}", min, max, min+max);
            }
            if sum > part1 {
                break;
            }
        }
    }

}

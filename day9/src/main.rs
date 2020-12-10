static INPUT: &str = include_str!("../day9.txt");
static WINDOW: usize = 25;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn is_valid(x: isize, window: &VecDeque<isize>) -> bool {
    let hs: HashSet<&isize> = HashSet::from_iter(window.iter());
    for l in hs.iter() {
        if **l > x {
            continue;
        }

        let delta = x - *l;

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

    for l in INPUT.lines().map(|x| x.parse::<isize>().unwrap()) {
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

    let mut sum = 0;
    let mut lo = 0;
    let mut hi = 0;

    loop {
        if sum < part1 {
            sum += all_numbers[hi];
            hi += 1;
        } else if sum > part1 {
            sum -= all_numbers[lo];
            lo += 1;
        } else {
            let subseq = &all_numbers[lo..hi];
            let min = subseq.iter().min().unwrap();
            let max = subseq.iter().max().unwrap();
            println!("{} + {} = {}", min, max, min + max);
            break;
        }
    }
}

static INPUT: &str = include_str!("../day10.txt");
static INPUT_1: &str = include_str!("../test1.txt");
static INPUT_2: &str = include_str!("../test2.txt");

use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let chain1 = find_chain(INPUT_1);
    assert_eq!(7 * 5, chain1);
    let chain2 = find_chain(INPUT_2);
    assert_eq!(22 * 10, chain2);

    let chain3 = find_chain(INPUT);
    println!("{}", chain3);

    let arrangements1 = find_arrangements(INPUT_1);
    assert_eq!(arrangements1, 8);

    let arrangements2 = find_arrangements(INPUT_2);
    assert_eq!(arrangements2, 19208);

    let arrangements = find_arrangements(INPUT);
    println!("{}", arrangements);
}

fn find_sub_arrangements(
    current: usize,
    memo: &mut HashMap<Vec<usize>, usize>,
    joltages: &[usize],
) -> usize {
    let mut cnt = 0;

    if joltages.len() == 0 {
        1
    } else if memo.contains_key(joltages) {
        *memo.get(joltages).unwrap()
    } else {
        for i in 0..3 {
            if i >= joltages.len() {
                continue;
            }
            if joltages[i] - current <= 3 {
                cnt += find_sub_arrangements(joltages[i], memo, &joltages[i + 1..]);
            }
        }

        memo.insert(joltages.to_vec(), cnt);
        cnt
    }
}

fn find_arrangements(input: &str) -> usize {
    let mut joltages: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).sorted().collect();
    joltages.sort();
    let mut memo = HashMap::<Vec<usize>, usize>::new();

    let cnt = find_sub_arrangements(0, &mut memo, &joltages[..]);
    println!("{}", memo.len());
    cnt
}

fn find_chain(input: &str) -> usize {
    let mut joltages: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).sorted().collect();
    joltages.sort();

    let mut current = 0;
    let mut cnt1 = 0;
    let mut cnt3 = 1;

    for i in joltages {
        let delta = i - current;
        match delta {
            3 => cnt3 += 1,
            1 => cnt1 += 1,
            2 => (),
            _ => unreachable!(),
        }
        current = i;
    }

    cnt1 * cnt3
}

static INPUT: &str = include_str!("../day10.txt");
static INPUT_1: &str = include_str!("../test1.txt");
static INPUT_2: &str = include_str!("../test2.txt");


use itertools::Itertools;

fn main() {
    let chain1 = find_chain(INPUT_1);
    assert_eq!(7*5, chain1);
    let chain2 = find_chain(INPUT_2);
    assert_eq!(22*10, chain2);

    let chain3 = find_chain(INPUT);
    println!("{}", chain3);
}

fn find_chain(input: &str) -> usize {
    let mut joltages : Vec<usize> = input.lines().map(|x|x.parse().unwrap()).sorted().collect();
    joltages.sort();

    let mut current = 0;
    let mut cnt1 = 0;
    let mut cnt3 = 1;

    for i in joltages {
        let delta = i-current;
        match delta {
            3 => cnt3 += 1,
            1 => cnt1 += 1,
            2 => (),
            0 => unreachable!(),
            _ => unreachable!(),
        }
        current = i;
    }

    cnt1 * cnt3
}

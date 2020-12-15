static INPUT: &str = include_str!("../day14.txt");

use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Debug)]
enum Command {
    #[display("mask = {0}")]
    Mask(String),
    #[display("mem[{0}] = {1}")]
    Mem(usize, usize),
}

fn build_mask(x: &str, c: char) -> usize {
    x.chars()
        .enumerate()
        .filter(|(_i, x)| *x == c)
        .map(|(i, _x)| 1 << 35 - i)
        .fold(0, |a, b| a | b)
}

fn apply_mask(zero_mask: usize, one_mask: usize, input: usize) -> usize {
    (input | one_mask) & !zero_mask
}

fn insert_part_two(loc: usize, val: usize, mem : &mut HashMap::<usize, usize>, one_mask: usize, x_mask: &[usize]) {
    let loc = loc | one_mask;

    float_bit(mem,
        loc,
        val,
        x_mask
    );
}

fn float_bit( mem : &mut HashMap::<usize, usize>, loc: usize, val: usize, x_mask: &[usize]) {
    if x_mask.is_empty() {
        mem.insert(loc, val);
    } else {
        let mask = x_mask[0];
        let x_mask = &x_mask[1..];

        float_bit(mem, loc | mask, val, x_mask);
        float_bit(mem, loc & !mask, val, x_mask);
    }
}

fn build_x_mask (x : &str) -> Vec<usize> {
    x.chars()
    .enumerate()
    .filter(|(_i, x)| *x == 'X')
    .map(|(i, _x)| 1 << 35 - i)
    .collect()
}

fn main() {
    let mut mem_one = HashMap::<usize, usize>::new();
    let mut mem_two = HashMap::<usize, usize>::new();

    let mut one_mask = 0;
    let mut zero_mask = 0;
    let mut x_mask = Vec::<usize>::new();

    for l in INPUT.lines() {
        let c: Command = l.parse().unwrap();

        match c {
            Command::Mask(x) => {
                one_mask = build_mask(&x, '1');
                zero_mask = build_mask(&x, '0');
                x_mask = build_x_mask(&x);
            }
            Command::Mem(loc, val) => {
                mem_one.insert(loc, apply_mask(zero_mask, one_mask, val));
                insert_part_two(loc, val, &mut mem_two, one_mask, &x_mask);
            }        
        }
    }

    println!("{}", mem_one.values().sum::<usize>()); 
    println!("{}", mem_two.values().sum::<usize>()); 
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(2, build_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", '0'));
        assert_eq!(64, build_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", '1'));
    }

    #[test]
    fn test_apply_masks() {
        let one_mask = build_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", '1');
        let zero_mask = build_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", '0');
        let input = 11;
        let output = 73;

        assert_eq!(output, apply_mask(zero_mask, one_mask, input));
    }
}

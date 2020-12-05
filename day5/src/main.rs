static INPUT: &str = include_str!("../day5.txt");

fn partition_rows(rows : &[char], start : usize, end : usize) -> usize {
    if rows.len() == 1 {
        match rows[0] {
            'F' => start,
            'B' => end,
            _ => unreachable!()
        }
    } else {
        let mut new_start = start;
        let mut new_end = end;
        let delta = (end-start+1)/2;

        match rows[0] {
            'F' => new_end =  start + delta - 1,
            'B' => new_start = start + delta,
            _ => unreachable!()
        }

        partition_rows(&rows[1..], new_start, new_end)
    }
}

fn partition_cols(cols : &[char], start : usize, end : usize) -> usize {
    let flip : Vec<char> = cols.iter().map(|x| match x {
        'L' => 'F',
        'R' => 'B',
        _ => unreachable!()
    }).collect();

    partition_rows(&flip[..], start, end)
}


fn seat_id(boarding : &str) -> usize {
    let cs : Vec<char> = boarding.chars().collect();
    let rows = &cs[0..=6];
    let row = partition_rows(rows, 0, 127);
    let cols = &cs[7..=9];
    let col = partition_cols(cols, 0, 7);

    row * 8 + col
}

fn main() {
    let mut max = 0;
    let mut seats = Vec::new();

    for boarding in INPUT.lines() {
        let id = seat_id(boarding);
        if id > max {
            max = id;
        }

        seats.push(id);
    }
    println!("{}", max);

    seats.sort();

    let mut last = seats[0]-1;
    for s in seats {
        if s - last > 1 {
            println!("{}", s-1);
            break;
        }
        last = s;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
    }
}
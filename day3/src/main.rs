static INPUT: &str = include_str!("../day3.txt");

fn count_trees (lines : &Vec<&str>, right : usize, down : usize) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        let mut r : Vec<char> = lines[y].chars().collect();
        if r[x] == '#' {
            r[x] = 'X';
            trees += 1;
        } else {
            r[x] = 'O';
        }

        x += right;
        x = x % r.len();
        y += down;

        if y >= lines.len() {
            break;
        }
    }

    trees
}

fn main() {
    let lines : Vec<&str> = INPUT.lines().collect();
    let trees = count_trees(&lines, 3, 1);
    
    println!("Part1: {}", trees);

    let paths = [[1,1], [3,1], [5,1], [7,1], [1,2]];
    let mut product = 1; 
    for p in paths.iter() {
        let tree_count = count_trees(&lines, p[0], p[1]);
        product *= tree_count;    
    }

    println!("Part2: {}", product);
}

static INPUT: &str = include_str!("../day13.txt");
use modinverse::modinverse;

fn part2(input: &str) -> i64 {
    let (a, n): (Vec<i64>, Vec<i64>) = input
        .split(",")
        .enumerate()
        .filter(|(_a, b)| *b != "x")
        .map(|(a, b)| (a as i64, b.parse::<i64>().unwrap()))
        .map(|(a, b)| (if a == 0 { a } else { b - a }, b))
        .unzip();

    // Can we calculate the product of n?
    let big_n = n.iter().fold(1, |a, b| a * b);

    let mut sum = 0;
    for i in 1..a.len() {
        let big_n_i = big_n / n[i];
        let big_m_i = modinverse(big_n_i, n[i]).unwrap();
        sum += a[i] * big_m_i * big_n_i;
    }

    return sum % big_n;
}

fn main() {
    let mut lines = INPUT.lines();
    let start = lines.next().unwrap().parse::<usize>().unwrap();
    let line2 = lines.next().unwrap();
    let buses: Vec<usize> = line2
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    let waits: Vec<usize> = buses.iter().map(|b| b - (start % b)).collect();
    let output = waits.iter().zip(buses).min().unwrap();

    println!("{}", output.0 * output.1);

    assert_eq!(3417, part2("17,x,13,19"));
    assert_eq!(754018, part2("67,7,59,61"));
    assert_eq!(779210, part2("67,x,7,59,61"));
    assert_eq!(1261476, part2("67,7,x,59,61"));
    assert_eq!(1202161486, part2("1789,37,47,1889"));
    assert_eq!(1068781, part2("7,13,x,x,59,x,31,19"));

    println!("{}", part2(line2));
}

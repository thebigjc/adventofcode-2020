static INPUT: &str = include_str!("../day12.txt");

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum Dir {
    #[display("N{0}")]
    North(isize),
    #[display("S{0}")]
    South(isize),
    #[display("E{0}")]
    East(isize),
    #[display("W{0}")]
    West(isize),
    #[display("L{0}")]
    Left(isize),
    #[display("R{0}")]
    Right(isize),
    #[display("F{0}")]
    Forward(isize),
}

fn main() {
    let mut x = 0 as isize;
    let mut y = 0 as isize;
    let mut wx = 10 as isize;
    let mut wy = -1 as isize;
    let mut sx = 0 as isize;
    let mut sy = 0 as isize;

    let mut dir = 90;

    for l in INPUT.lines() {
        let d: Dir = l.parse().unwrap();
        match d {
            Dir::North(u) => {
                y -= u;
                wy -= u;
            }
            Dir::South(u) => {
                y += u;
                wy += u;
            }
            Dir::East(u) => {
                x += u;
                wx += u;
            }
            Dir::West(u) => {
                x -= u;
                wx -= u;
            }
            Dir::Left(u) => {
                dir = (dir - u + 360) % 360;
                match u {
                    90 => {
                        let tmpx = wx;
                        wx = wy;
                        wy = -tmpx;
                    }
                    270 => {
                        let tmpx = wx;
                        wx = -wy;
                        wy = tmpx;
                    }
                    180 => {
                        wx = -wx;
                        wy = -wy;
                    }
                    _ => unreachable!(),
                }
            }
            Dir::Right(u) => {
                dir = (dir + u + 360) % 360;
                match u {
                    270 => {
                        let tmpx = wx;
                        wx = wy;
                        wy = -tmpx;
                    }
                    90 => {
                        let tmpx = wx;
                        wx = -wy;
                        wy = tmpx;
                    }
                    180 => {
                        wx = -wx;
                        wy = -wy;
                    }
                    _ => unreachable!(),
                }
            }
            Dir::Forward(u) => {
                match dir {
                    0 => y -= u,
                    180 => y += u,
                    90 => x += u,
                    270 => x -= u,
                    _ => {
                        println!("{}", dir);
                        unreachable!();
                    }
                };
                sx += wx * u;
                sy += wy * u;
            }
        }
    }

    println!("Part One: {} ", x.abs() + y.abs());
    println!("Part Two: {} ", sx.abs() + sy.abs());
}

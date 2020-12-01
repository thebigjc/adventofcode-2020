use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let mut vec = read(File::open("day1.txt")?)?;
    vec.sort();

    for i in 0..vec.len() {
       for j in i+1..vec.len() {
           if vec[i] + vec[j] > 2020 {
               break;
           }
           if vec[i] + vec[j] == 2020 {
               println!("{:?}", vec[i] * vec[j]);
           }
       }
    }

    for i in 0..vec.len() {
       for j in i+1..vec.len() {
           for k in j+1..vec.len() {
               if vec[i] + vec[j] + vec[k] > 2020 {
                   break;
               }
               if vec[i] + vec[j] + vec[k] == 2020 {
                   println!("{:?}", vec[i] * vec[j] * vec[k]);
               }
           }
       }
    }

    // use `vec` for whatever
    Ok(())
}

use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

const SQRT_MAX: u128 = 18446744073709551616;
const FILE: &str = "primes.dat";

fn main() {
    match env::args().nth(1).unwrap().parse::<u128>() {
        Ok(limit) => {
            let mut start = 3;
            if Path::new(FILE).exists() {
                start = BufReader::new(File::open(FILE).unwrap())
                    .lines()
                    .last()
                    .unwrap()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap()
                    + 2;
                if start >= limit {
                    println!("The file already contains all the prime numbers up to the limit you entered.");
                    return;
                }
            } else {
                BufWriter::new(File::create(FILE).unwrap())
                    .write_all(b"2\n")
                    .unwrap();
            }
            for dividend in (start..=limit).step_by(2) {
                let reader = BufReader::new(File::open(FILE).unwrap()).lines();
                for divisor in reader {
                    let divisor = divisor.unwrap().parse::<u128>().unwrap();
                    if divisor >= SQRT_MAX || divisor * divisor > dividend {
                        BufWriter::new(OpenOptions::new().append(true).open(FILE).unwrap())
                            .write_all(format!("{}\n", dividend).as_bytes())
                            .unwrap();
                        break;
                    } else if dividend % divisor == 0 {
                        break;
                    }
                }
            }
        }
        Err(_) => println!("The input is not an integer."),
    }
    return;
}

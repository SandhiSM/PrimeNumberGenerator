use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    match env::args().nth(1).unwrap().parse::<u128>() {
        Ok(n) => {
            {
                let file = File::create("primes.dat").unwrap();
                let mut writer = BufWriter::new(file);
                writer.write_all(b"2\n").unwrap();
            }
            calc(3, 2, 0, n);
            return;
        }
        Err(_) => {
            println!(
                "Read the README.md file to understand how to use this application and try again."
            );
            return;
        }
    }
}

fn calc(dividend: u128, divisor: u128, index: usize, limit: u128) {
    if divisor * divisor > dividend {
        write(dividend);
    } else if dividend % divisor != 0 {
        return calc(dividend, read(index + 1), index + 1, limit);
    }
    if dividend != u128::MAX && dividend + 2 <= limit {
        return calc(dividend + 2, 2, 0, limit);
    } else {
        return;
    }
}

fn read(index: usize) -> u128 {
    BufReader::new(File::open("primes.dat").unwrap())
        .lines()
        .into_iter()
        .nth(index)
        .unwrap()
        .unwrap()
        .parse()
        .unwrap()
}

fn write(prime: u128) {
    BufWriter::new(OpenOptions::new().append(true).open("primes.dat").unwrap())
        .write_all(format!("{}\n", prime).as_bytes())
        .unwrap();
}

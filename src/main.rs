use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
fn main() {
    if env::args().len() != 2 {
        println!("Read the Readme.md file for more information on how to use the program.");
        return;
    }
    let limit: usize = env::args().nth(1).unwrap().parse().unwrap();
    let mut primes = Vec::new();
    for i in (11..=limit).step_by(2) {
        if i % 3 != 0 && i % 5 != 0 && i % 7 != 0 {
            primes.push(i);
        }
    }
    let mut writer = BufWriter::new(File::create(format!("primes~{}.csv", limit)).unwrap());
    writer.write(b"2,\n3,\n5,\n7,\n").unwrap();
    for index_dividend in 0..primes.len() {
        for index_divisor in 0..index_dividend {
            if primes[index_divisor] == 0 {
                continue;
            } else if primes[index_dividend] < primes[index_divisor] * primes[index_divisor] {
                writer
                    .write(format!("{},\n", primes[index_dividend]).as_bytes())
                    .unwrap();
                break;
            } else if primes[index_dividend] % primes[index_divisor] == 0 {
                primes[index_dividend] = 0;
                break;
            } else {
                continue;
            }
        }
    }
    return;
}

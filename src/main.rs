use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    match env::args().nth(1).unwrap().parse::<u128>() {
        Ok(limit) => {
            {
                BufWriter::new(File::create("primes.dat").unwrap())
                    .write_all(b"2\n")
                    .unwrap();
            }
            for dividend in (3..=limit).step_by(2) {
                let reader = BufReader::new(File::open("primes.dat").unwrap());
                for divisor in reader.lines() {
                    let divisor = divisor.unwrap().parse::<u128>().unwrap();
                    if divisor * divisor > dividend {
                        BufWriter::new(OpenOptions::new().append(true).open("primes.dat").unwrap())
                            .write_all(format!("{}\n", dividend).as_bytes())
                            .unwrap();
                        break;
                    } else if dividend % divisor == 0 {
                        break;
                    }
                }
            }
        }
        Err(_) => println!(
            "Read the README.md file to understand how to use this application and try again."
        ),
    }
    return;
}

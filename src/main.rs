use std::fs::File;
use std::io::{BufWriter, Write};
fn main() {
    let mut number = String::new();
    println!("Enter a integer (max: {}): ", u32::MAX);
    std::io::stdin().read_line(&mut number).unwrap();
    let number: u32 = number.trim().parse().unwrap();
    let mut numbers = Vec::new();
    for i in 11..number {
        if i % 2 != 0 && i % 3 != 0 && i % 5 != 0 && i % 7 != 0 {
            numbers.push(i);
        }
    }
    let mut removed: usize = 0;
    'top: for mut index_dividend in 0..numbers.len() {
        index_dividend -= removed;
        for index_divisor in 0..numbers.len() {
            if index_dividend == index_divisor {
                continue;
            } else if numbers[index_dividend] < numbers[index_divisor].pow(2) {
                continue 'top;
            } else if numbers[index_dividend] % numbers[index_divisor] == 0 {
                numbers.remove(index_dividend);
                removed += 1;
                continue 'top;
            }
        }
    }
    println!("Finished calculating primes up to {}", number);
    {
        let length = numbers.len() + 4;
        let file_path = File::create("primes.csv").unwrap();
        let mut writer = BufWriter::new(file_path);
        writeln!(writer, "# Primes up to {}\n2\n3\n5\n7", number).unwrap();
        for number in numbers {
            writeln!(writer, "{}", number).unwrap();
        }
        writeln!(writer, "# Total: {}", length).unwrap();
    }
}

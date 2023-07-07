use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    println!("Finding all primes that fit inside a u32.");

    let mut known_primes: Vec<u128> = Vec::with_capacity(8192);
    let mut reader = BufReader::new(File::open("primes_u16.txt").unwrap());
    let mut buffer: String = String::with_capacity(64);
    while let Ok(result) = reader.read_line(&mut buffer) {
        if result > 0 {
            match u128::from_str_radix(&buffer.trim(), 10) {
                Ok(y) => {
                    known_primes.push(y);
                    buffer.clear();
                }
                Err(err) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                }
            }
        } else {
            break;
        }
    }

    for x in 4..=u32::MAX {
        if is_prime(x as u128, &known_primes) {
            //println!("{} is prime.", x);
            known_primes.push(x as u128);
        }
    }

    let mut writer = BufWriter::new(File::create("primes_u32.txt").unwrap());
    for x in known_primes {
        let _ = writeln!(writer, "{}", x);
    }
    return writer.flush();
}

fn is_prime(x: u128, known_primes: &Vec<u128>) -> bool {
    for y in known_primes {
        if x % y == 0 {
            return false;
        }
    }
    return true;
}

use bincode::Options;
use clap::Parser;
use prime_finder::compute_fast_divisor;
use prime_finder::PrimeAndDivisor;
use prime_finder::SERIALIZER;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

/// Simple program to make a lookup table of primes and fast divisors.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file of primes.
    #[arg(index = 1)]
    input_file: String,
    /// Output file for the table.
    #[arg(index = 2)]
    output_file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input_path: PathBuf = Path::new(&args.input_file).canonicalize()?;
    let output_path: &Path = Path::new(&args.output_file);
    let input_file: File = File::open(input_path)?;
    let output_file: File = File::create(output_path)?;
    let mut reader: BufReader<File> = BufReader::new(input_file);
    let mut writer: BufWriter<File> = BufWriter::new(output_file);

    let mut buffer: String = String::with_capacity(64);
    while let Ok(result) = reader.read_line(&mut buffer) {
        if result > 0 {
            match u128::from_str_radix(&buffer.trim(), 10) {
                Ok(y) => {
                    buffer.clear();
                    let x: u128 = compute_fast_divisor(y);
                    match SERIALIZER.serialize(&PrimeAndDivisor {
                        prime: y,
                        divisor: x,
                    }) {
                        Ok(buf) => {
                            writer.write_all(&buf)?;
                        }
                        Err(err) => {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, *err));
                        }
                    };
                }
                Err(err) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                }
            }
        } else {
            break;
        }
    }
    return Ok(());
}

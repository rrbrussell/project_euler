use bincode::Options;
use clap::Parser;
use prime_finder::open_file;
use prime_finder::PrimeAndDivisor;
use prime_finder::SERIALIZER;
use std::collections::HashMap;
use std::fs::File;
use std::fs::Metadata;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;

/// Simple program to make a lookup table of primes and fast divisors.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file of primes.
    #[arg(long = "list")]
    list_file: String,
    /// Output file for the primes.
    #[arg(long = "output")]
    output_file: String,
    /// Test candidates up to bits in length.
    #[arg(long = "length")]
    candidate_length: u32,
}

fn main() -> Result<(), std::io::Error> {
    let args: Args = Args::parse();

    // Use threads or not.
    let number_of_threads = thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(1_usize).unwrap())
        .get();
    if number_of_threads < 2 {
        println!("You shouldn't run this program on a single thread processor.");
        return Ok(());
    } else {
        let mut known_primes: HashMap<u128, u128> = HashMap::with_capacity(8192);
        let list_reader: BufReader<File>;
        let file_size: u64;

        match open_file(&args.list_file) {
            Ok(file) => {
                let metadata: Metadata = file.metadata().expect(
                    format!("Somebody is screwing around with {}", &args.list_file).as_str(),
                );
                file_size = metadata.len();
                if file_size == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        format!("{} must not be empty.", &args.list_file),
                    ));
                }
            }
            Err(err) => {
                eprintln!("Unable to open {}.", &args.list_file);
                return Err(err);
            }
        }

        let mut buffer: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let mut keep_reading: bool = true;
        while keep_reading {
            match list_reader.read_exact(&mut buffer[..]) {
                Err(err) => {
                    match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            // We are done reading the file.
                            keep_reading = false;
                            break;
                        }
                        _ => {
                            eprintln!(
                                "Unexpected {err} occurred while reading from {}.",
                                args.list_file
                            );
                            keep_reading = false;
                            break;
                        }
                    }
                }
                Ok(_) => match list_reader.stream_position() {
                    Err(err) => {
                        eprintln!("I received {err} message while reading input primes table.");
                        keep_reading = false;
                    }
                    Ok(position) => {
                        match SERIALIZER.deserialize::<PrimeAndDivisor>(&buffer) {
                            Ok(prime) => {
                                known_primes.insert(prime.prime, prime.divisor);
                                buffer.fill(0);
                            }
                            Err(err) => {
                                eprintln!("Unable to parse a prime because of {}", err);
                            }
                        }
                        if position >= file_size {
                            keep_reading = false;
                        }
                    }
                },
            }
        }

        let mut threads = HashMap::<String, JoinHandle<()>>::with_capacity(number_of_threads);
        let mut to_threads = HashMap::<String, Sender<Message>>::with_capacity(number_of_threads);
        let (to_main, from_threads) = channel::<Message>();
        for ind in 0..number_of_threads {
            let (to_thread, from_main) = channel::<Message>();
            to_threads.insert(format!("Worker {ind}"), to_thread);
            let mut known_primes = known_primes.clone();
            let to_main = to_main.clone();
            let thread = Builder::new()
                .name(format!("Worker {ind}"))
                .spawn(move || other_thread(&mut known_primes, from_main, to_main));
            if thread.is_err() {
                return Err(thread.err().unwrap());
            } else {
                threads.insert(format!("Worker {ind}"), thread.unwrap());
            }
        }

        // known_primes has at least two items in it.
        let mut x: u128 = *known_primes.last().unwrap();

        //prime the worker threads
        for (_, value) in to_threads.iter() {
            x += 2;
            // This should explode if a thread went missing.
            value.send(Message::TestThis(x)).unwrap();
        }
        let mut exhausted_search_space: bool = false;
        while let Ok(received) = from_threads.recv() {
            let mut refill_thread_name: String = "".to_string();
            match received {
                Message::IsPrime((prime, thread_name)) => {
                    known_primes.push(prime);
                    refill_thread_name = thread_name.to_owned();
                    for (_, sender) in to_threads.iter() {
                        // If a thread has died unexpectedly then fail.
                        sender
                            .send(Message::IsPrime((prime, thread_name.to_owned())))
                            .unwrap();
                    }
                }
                Message::IsNotPrime((_, thread_name)) => {
                    refill_thread_name = thread_name.to_owned();
                }
                Message::TestThis(_) => {
                    unreachable!("Nothing should be sending main primes to test.");
                }
                Message::Stop(thread_name) => {
                    if !to_threads.is_empty() {
                        panic!("Why haven't all the channels been dropped in main.");
                    } else {
                        match threads.remove(&thread_name) {
                            None => {
                                panic!("I lost the handle to {thread_name}.");
                            }
                            Some(handle) => {
                                handle
                                    .join()
                                    .expect("Something exploded while joining {thread_name}");
                            }
                        }
                        if threads.is_empty() {
                            break;
                        }
                    }
                }
            }
            if !exhausted_search_space {
                x += 2;
                if x > u32::MAX as u128 {
                    exhausted_search_space = true;
                    for (key, value) in to_threads.drain() {
                        let _ = value.send(Message::Stop("main".to_string()));
                        drop(value);
                        drop(key);
                    }
                } else {
                    match to_threads[&refill_thread_name].send(Message::TestThis(x)) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("{refill_thread_name} disappeared early.");
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                        }
                    }
                }
            }
        }
        let mut writer = BufWriter::new(File::create("primes_u32.txt").unwrap());
        known_primes.sort();
        known_primes.dedup();
        for x in known_primes {
            let _ = writeln!(writer, "{}", x);
        }
        return writer.flush();
    }
}

fn is_prime(x: u128, known_primes: &Vec<u128>) -> bool {
    for y in known_primes {
        if x % y == 0 {
            return false;
        }
    }
    return true;
}

enum Message {
    IsPrime((u128, String)),
    IsNotPrime((u128, String)),
    TestThis(u128),
    Stop(String),
}

/// This is the other_threads
fn other_thread(
    known_primes: &mut Vec<u128>,
    from_main: Receiver<Message>,
    to_main: Sender<Message>,
) {
    let my_name: String = thread::current().name().unwrap().to_string();
    loop {
        match from_main.recv() {
            Err(_) => {}
            Ok(input) => match input {
                Message::IsPrime((x, thread_name)) => {
                    if my_name != thread_name {
                        known_primes.push(x);
                    }
                }
                Message::IsNotPrime((x, thread)) => {
                    unreachable!("Main forwarded {x} is not prime from {thread}. Please fix.");
                }
                Message::TestThis(x) => {
                    if is_prime(x, known_primes) {
                        to_main
                            .send(Message::IsPrime((x, my_name.to_owned())))
                            .expect("Main thread should not die first.");
                        known_primes.push(x);
                    } else {
                        to_main
                            .send(Message::IsNotPrime((x, my_name.to_owned())))
                            .expect("Main thread should not die first.");
                    }
                }
                Message::Stop(_) => {
                    let _ = to_main.send(Message::Stop(my_name.to_owned()));
                    drop(to_main);
                    drop(from_main);
                    drop(known_primes);
                    drop(my_name);
                    break; // I can stop now.
                }
            },
        }
    }
}

/*
match x.checked_add(2_u128) {
    Some(new_x) => {
        to_threads[usize::from_str(&thread_name).unwrap()]
            .send(Message::TestThis(new_x))
            .unwrap();
        x = new_x;
    }
    None => {
        for index in &to_threads {
            index.send(Message::Shutdown).unwrap();
        }
    }
}*/

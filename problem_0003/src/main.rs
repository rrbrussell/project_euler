use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::TrySendError;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;

fn main() -> Result<(), std::io::Error> {
    println!("Finding all primes that fit inside a u32.");

    // Use threads or not.
    let number_of_threads = thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(1_usize).unwrap())
        .get();
    if number_of_threads < 2 {
        println!("You shouldn't run this program on a single thread processor.");
        return Ok(());
    } else {
        let mut known_primes: Vec<u128> = Vec::with_capacity(8192);
        let previous_primes_list = [
            /*"primes_u64.txt",
            "primes_u32.txt",
            "primes_u16.txt",*/
            "primes_u8.txt",
        ];
        for list_entry in previous_primes_list {
            let possible_previous_primes = Path::new(list_entry).try_exists()?;
            if possible_previous_primes {
                let mut reader = BufReader::new(File::open(list_entry).unwrap());
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
                break;
            }
        }
        if known_primes.len() < 3 {
            known_primes.push(2);
            known_primes.push(3);
        }

        let mut threads = Vec::<JoinHandle<()>>::with_capacity(number_of_threads);
        let mut to_threads = Vec::<SyncSender<Message>>::with_capacity(number_of_threads);
        let (to_main, from_threads) = sync_channel::<Message>(10);
        for ind in 0..number_of_threads {
            let (to_thread, from_main) = sync_channel::<Message>(10);
            to_threads.push(to_thread);
            let mut known_primes = known_primes.clone();
            let to_main = to_main.clone();
            let thread = Builder::new()
                .name(ind.to_string())
                .spawn(move || other_thread(&mut known_primes, from_main, to_main));
            if thread.is_err() {
                eprintln!("Cannot spawn a client thread.");
                return Err(thread.err().unwrap());
            } else {
                threads.push(thread.unwrap());
            }
        }

        // known_primes has at least two items in it.
        let mut x: u128 = *known_primes.last().unwrap();
        let mut keep_looping: bool = true;
        let mut overflow: bool = false;
        while keep_looping {
            if !overflow {
                for index in 0..to_threads.len() {
                    eprintln!("Sending {x} to thread {index}");
                    let mut keep_sending = true;
                    while keep_sending {
                        x += 2;
                        if x < u16::MAX as u128 {
                            match to_threads[index].try_send(Message::TestThis(x)) {
                                Ok(()) => {}
                                Err(err) => match err {
                                    TrySendError::Full::<Message>(_) => {
                                        x -= 2;
                                        keep_sending = false;
                                    }
                                    TrySendError::Disconnected::<Message>(_) => {
                                        x -= 2;
                                        keep_sending = false;
                                    }
                                },
                            }
                        } else {
                            keep_sending = false;
                            overflow = true;
                        }
                    }
                }
            } else if to_threads.len() > 0 {
                let mut index: usize = to_threads.len();
                while index > 0 {
                    match to_threads[index - 1].try_send(Message::Shutdown) {
                        Ok(()) => {
                            let _ = to_threads.remove(index);
                            index -= 1;
                        }
                        Err(err) => match err {
                            // Leave this thread alone. We will try again to close
                            // it later.
                            TrySendError::Full::<Message>(_) => {
                                index -= 1;
                            }
                            TrySendError::Disconnected::<Message>(_) => {
                                // This thread is already shutdown remove it.
                                let _ = to_threads.remove(index);
                                index -= 1;
                            }
                        },
                    }
                }
            }

            let received = from_threads.recv();
            match received {
                Err(_) => {
                    keep_looping = false;
                }
                Ok(message) => match message {
                    Message::FoundPrime((prime, thread_name)) => {
                        known_primes.push(prime);
                        for thread in &to_threads {
                            eprintln!("Found prime: {prime} from {thread_name}");
                            // If a thread has died unexpectedly then fail.
                            thread
                                .send(Message::FoundPrime((prime, thread_name.to_owned())))
                                .unwrap();
                        }
                    }
                    Message::TestThis(_) => {
                        eprintln!("Nothing should be sending main primes to test.")
                    }
                    Message::Shutdown => {
                        eprintln!("Main tells everything else to shutdown.")
                    }
                },
            }
        }
        let mut writer = BufWriter::new(File::create("primes_u32.txt").unwrap());
        known_primes.sort();
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
    FoundPrime((u128, String)),
    TestThis(u128),
    Shutdown,
}

/// This is the other_threads
fn other_thread(
    known_primes: &mut Vec<u128>,
    from_main: Receiver<Message>,
    to_main: SyncSender<Message>,
) {
    let my_name: String = thread::current().name().unwrap().to_string();
    while let Ok(input) = from_main.recv() {
        match input {
            Message::Shutdown => {
                break;
            }
            Message::FoundPrime((x, thread_name)) => {
                if my_name != thread_name {
                    known_primes.push(x);
                }
            }
            Message::TestThis(x) => {
                if is_prime(x, known_primes) {
                    to_main
                        .send(Message::FoundPrime((x, my_name.to_owned())))
                        .expect("Main thread should not die first.");
                    known_primes.push(x);
                }
            }
        }
    }
    drop(from_main);
    drop(to_main);
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

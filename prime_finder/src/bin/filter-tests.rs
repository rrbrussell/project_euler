use std::collections::BTreeSet;

/// A program for testing different ways of filtering primes.
fn main() {
    let mut fives_multiple: u16 = 5_u16;
    let mut last_multiple: u16 = 5_u16;
    let mut bytes: BTreeSet<u8> = BTreeSet::<u8>::new();
    while fives_multiple >= last_multiple {
        if (fives_multiple & 5_u16) == 5 {
            last_multiple = fives_multiple;
            fives_multiple += 5;
        } else {
            if (fives_multiple & 10_u16) == 10 {
                last_multiple = fives_multiple;
                fives_multiple += 5;
            } else {
                bytes.insert(fives_multiple as u8);
                let byte = fives_multiple as u8;
                println!("{fives_multiple:0>8}:\t{byte:0>8b}\t{byte:0>2x}\t{byte:0>4}");
                last_multiple = fives_multiple;
                fives_multiple += 5;
            }
        }
    }

    // for byte in bytes {
    //     println!("{byte:0>8b}\t{byte:0>2x}\t{byte:0>4}");
    // }
    // println!(
    //     "{0:0>8}:\t{1:0>8b}\t{1:0>2x}\t{1:0>4}\t{2}",
    //     5,
    //     5 & 5,
    //     5 & 5 == 5
    // );
    // println!(
    //     "{0:0>8}:\t{1:0>8b}\t{1:0>2x}\t{1:0>4}\t{2}",
    //     10,
    //     10 & 5,
    //     10 & 5 == 5
    // );
    // println!(
    //     "{0:0>8}:\t{1:0>8b}\t{1:0>2x}\t{1:0>4}\t{2}",
    //     10,
    //     10 & 10,
    //     10 & 10 == 10
    // );
    // println!(
    //     "{0:0>8}:\t{1:0>8b}\t{1:0>2x}\t{1:0>4}\t{2}",
    //     15,
    //     15 & 5,
    //     15 & 5 == 5
    // );
    // println!(
    //     "{0:0>8}:\t{1:0>8b}\t{1:0>2x}\t{1:0>4}\t{2}",
    //     15,
    //     15 & 10,
    //     15 & 10 == 10
    // );
}

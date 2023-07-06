fn main() {
    println!("If we list all the natural numbers below 10 that are multiples");
    print!("of 3 or 5, we get 3, 5, 6, and 9. The sum of these multiples is");
    println!(" 23.\n");
    println!("Find the sum of all the multiples of 3 or 5 below 1000.");

    let mut sum: i64 = 0;
    for item in 1..=1000 {
        if item % 3 == 0 {
            sum = sum + item;
        }
        if item % 5 == 0 {
            if item % 15 != 0 {
                sum = sum + item;
            }
        }
    }
    println!("\nThe sum of the multiples of 3 or 5 below 1000 is {}", sum);
}

use std::vec::Vec;
fn main() {
    println!("Project Euler Problem 4 Solver");
    print!("The palindromes made from multiplying three-digit");
    println!(" numbers are:");

    let mut x: u32 = 999_u32;
    let mut y: u32 = 999_u32;
    let mut z: u32;
    let mut palindromes: Vec<u32> = Vec::<u32>::with_capacity(1024);
    while x > 99 {
        while y > 99 {
            z = x * y;
            y -= 1;
            if is_palindrome(z) {
                palindromes.push(z);
            }
        }
        y = 999;
        x -= 1;
    }
    palindromes.sort();
    palindromes.dedup();
    println!("Largest palindrome is {}", palindromes.last().unwrap());
}

fn is_palindrome(number: u32) -> bool {
    let forward: String = number.to_string();
    let mut backward: String = String::with_capacity(forward.len());
    let mut temp: String = forward.to_owned();
    loop {
        match temp.pop() {
            Some(ch) => {
                backward.push(ch);
            }
            None => {
                break;
            }
        }
    }
    let backward: String = backward;
    return forward == backward;
}

#[cfg(test)]
mod test {
    use crate::is_palindrome;
    #[test]
    fn test_is_palindrome() {
        let test_data = [
            (32_u32, false),
            (99_u32, true),
            (11_u32, true),
            (2_u32, true),
        ];
        for (test_case, expected_result) in test_data {
            assert_eq!(is_palindrome(test_case), expected_result);
        }
    }
}

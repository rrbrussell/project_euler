fn main() {
    println!("Hello, world!");
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

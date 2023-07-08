// The prime numbers less than 20 are 2, 3, 5, 7, 11, 13, 17, and 19.
// I will first try the multiple of all these numbers.

fn main() {
    let mut x: u64 = 9699690_u64;
    loop {
        x += 2_u64;
        if x < 9699690_u64 {
            break;
        }
        if x % 20 == 0 {
            if x % 19 == 0 {
                if x % 18 == 0 {
                    if x % 17 == 0 {
                        if x % 16 == 0 {
                            if x % 15 == 0 {
                                if x % 14 == 0 {
                                    if x % 13 == 0 {
                                        if x % 12 == 0 {
                                            if x % 11 == 0 {
                                                if x % 10 == 0 {
                                                    if x % 9 == 0 {
                                                        if x % 8 == 0 {
                                                            if x % 7 == 0 {
                                                                if x % 6 == 0 {
                                                                    if x % 5 == 0 {
                                                                        if x % 4 == 0 {
                                                                            if x % 3 == 0 {
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{x}");
}

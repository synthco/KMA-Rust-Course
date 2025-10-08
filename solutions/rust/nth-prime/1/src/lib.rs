pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")

    let mut count = 0;
    let mut num = 2;

    loop{
        if is_prime(num) {
            if count == n {
                return num;
            }
            count += 1;
        }
        num += 1;
    }
}

fn is_prime(x: u32) -> bool {
    fn check_divisor(x: u32, d: u32) -> bool {
        if d * d > x {
            true
        } 
        else if x % d == 0 {
            false
        } 
        else {
            check_divisor(x, d + 1)
        }
    }
    check_divisor(x, 2)
}

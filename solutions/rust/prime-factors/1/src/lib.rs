pub fn factors(mut n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")
    let mut result = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            result.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }
    result
}

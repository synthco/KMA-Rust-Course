pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");

    if s== 1 {
        1
    }
    else {
        2 * square(s-1)
    }
}

pub fn total() -> u64 {
    // todo!();
    fn helper(n: u32) -> u64 {
        if n == 0 {
            0
        } else {
            square(n) + helper(n - 1)
        }
    }
    helper(64)
}


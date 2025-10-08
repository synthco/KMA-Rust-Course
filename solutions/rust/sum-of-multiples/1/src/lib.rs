use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut multiples = HashSet::new();

    for &factors in factors {
        if factors == 0 {
            continue;
        } 
         let mut n = factors;
        while n < limit {
            multiples.insert(n);
            n += factors;
        }
    }
    multiples.iter().sum()
}

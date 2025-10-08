pub fn is_armstrong_number(num: u32) -> bool {
   let digits_str = num.to_string();
    let n = digits_str.len() as u32;
    let mut sum = 0;
    for c in digits_str.chars() {
        let d = c.to_digit(10).unwrap();
        sum += d.pow(n);
    }
    sum == num
}

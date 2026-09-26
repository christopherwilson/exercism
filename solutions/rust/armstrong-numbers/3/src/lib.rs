pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut n = num;
    
    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);

    let power = digits.len();

    let sum: u32 = digits.iter().map(|n| n.pow(power as u32)).sum();
    sum == num
}

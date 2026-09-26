pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut n = num;
    
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);

    let power = digits.len();

    digits.iter().map(|n| n.pow(power as u32)).sum::<u32>() == num
}

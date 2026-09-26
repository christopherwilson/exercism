/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        return false
    }

    let mut digits: Vec<u32> = Vec::new();

    for digit in code.chars().rev() {
        if digit.is_ascii_digit() {
            let digit: u32 = digit.to_digit(10).unwrap();
            if digits.len() % 2 == 1 {
                let mut doubled = digit * 2;
                if doubled > 9 {
                    doubled -= 9;
                }
                digits.push(doubled);
            } else {
                digits.push(digit);
            }
        } else if digit != ' ' {
            return false
        }
    }

    // if we have an invalid code padded with spaces
    if digits.len() <= 1 {
        return false
    }

    digits.into_iter().sum::<u32>().is_multiple_of(10)
}

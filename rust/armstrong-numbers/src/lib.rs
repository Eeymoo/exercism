pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let mut num = num;
    let len = (num as f64).log10().ceil() as u32;
    let original_num = num as u64;
    let mut sum: u64 = 0;
    while num > 0 {
        let digit = num % 10;
        sum += digit.pow(len) as u64;
        num /= 10;
    };

    sum == original_num
}
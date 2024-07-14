pub fn square(s: u32) -> u64 {

    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    let sum = u64::pow(2, s-1) as u64;
    println!("{}:{}", s, sum);
    
    sum
}

pub fn total() -> u64 {
    let mut sum = 0;
    for s in 1..65 {
        sum += square(s);
    };
    sum
}

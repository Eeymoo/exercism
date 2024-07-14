pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum:Vec<u32> = Vec::new();

    for s in 1..limit {
        for f in factors {
            if &0 == f {
                break;
            }
            if s % f == 0 && !sum.contains(&s) {
                sum.push(s);
            }
        }
    }

    let mut sum_u32 = 0;

    for s in sum.iter() {
        sum_u32 += s;        
    }

    sum_u32
}

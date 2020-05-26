pub fn nth(n: u32) -> u32 {
    let size: usize = (n + 1) as usize;
    let mut primes: Vec<u32> = Vec::new();
    let mut number: u32 = 3;
    primes.push(2);
    while primes.len() != size {
        if is_prime(number) {
            primes.push(number);
        }
        number += 2;
    }
    *primes.last().unwrap()
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let limit: u32 = (n as f32).sqrt() as u32;
        (3..=limit).step_by(2)
            .all(|i| n % i != 0)
    }
}

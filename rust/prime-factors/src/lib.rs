pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut num = n;
    factor_out(&mut num, &mut primes, 2);
    let mut factor: u64 = 3;
    while num != 1 {
        factor_out(&mut num, &mut primes, factor);
        factor += 2;
    }
    primes
}

fn factor_out(n: &mut u64, primes: &mut Vec<u64>, f: u64) {
    while *n % f == 0 {
        primes.push(f);
        *n /= f;
    }
}

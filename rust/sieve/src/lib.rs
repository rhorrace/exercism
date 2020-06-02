pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::new();
    while !numbers.is_empty() {
        let p = numbers[0];
        primes.push(p);
        numbers = numbers.iter()
            .filter(|&&n| n % p != 0)
            .map(|n| *n)
            .collect();
    }
    primes
}

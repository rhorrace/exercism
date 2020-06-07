pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some(conjecture(n))
    }
}

fn conjecture(mut n: u64) -> u64 {
    let mut count:u64 = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    count
}

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products: BTreeMap<u64, Palindrome> = BTreeMap::new();
    (min..=max).for_each(|i| {
        (i..=max).for_each(|j| {
            let n = i * j;
            if is_palindrome(n) {
                products.entry(n)
                    .and_modify(|p| p.insert(i, j))
                    .or_insert(Palindrome::new(i, j));
            }
        })
    });
    let mut prod_iter = products.iter();
    match (prod_iter.next(), prod_iter.last()) {
        (Some(a), Some(b)) => Some((a.1.clone(), b.1.clone())),
        (Some(a), None) => Some((a.1.clone(), a.1.clone())),
        _ => None
    }
}

fn is_palindrome(n: u64) -> bool {
    n.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap() == n
}
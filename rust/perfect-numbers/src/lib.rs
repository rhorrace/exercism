#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => Some(classification(num))
    }
}

fn classification(num: u64) -> Classification {
    let sum = factors_sum(num);
    if sum == num {
        Classification::Perfect
    } else if sum < num {
        Classification::Deficient
    } else {
        Classification::Abundant
    }
}

fn factors_sum(num: u64) -> u64 {
    let stop = (num as f64).sqrt() as u64;
    (2..=stop).filter(|&f| num % f == 0)
        .map(|f|
            if f != num / f {
                f + num / f
            } else {
                f
            })
        .sum::<u64>() + 1
}
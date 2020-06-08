use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum / 3).map(|a| {
            let bc = sum - a;
            let b = (bc.pow(2) - a.pow(2)) / (2 * bc);
            [a, b, bc - b]
        })
        .filter(|[a, b, c]| a < b && a.pow(2) + b.pow(2) == c.pow(2))
        .collect()
}

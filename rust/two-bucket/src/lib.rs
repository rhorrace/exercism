use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    let mut stops: HashSet<(u8, u8)> = HashSet::new();
    let capacities = (capacity_1, capacity_2);
    let one_full = (capacity_1, 0);
    let two_full = (0, capacity_2);
    match start_bucket {
        Bucket::One => {
            stops.insert(two_full);
            find(capacities, one_full, goal, 1, &mut stops)
        }
        Bucket::Two => {
            stops.insert(one_full);
            find(capacities, two_full, goal, 1, &mut stops)
        }
    }
}

fn find(capacity: (u8, u8), amount: (u8, u8), goal: u8, moves: u8, stops: &mut HashSet<(u8, u8)>) -> Option<BucketStats> {
    if stops.contains(&amount) {
        None
    } else if amount.0 == goal {
        Some(BucketStats {
            moves,
            goal_bucket: Bucket::One,
            other_bucket: amount.1,
        })
    } else if amount.1 == goal {
        Some(BucketStats {
            moves,
            goal_bucket: Bucket::Two,
            other_bucket: amount.0,
        })
    } else {
        stops.insert(amount);
        let amount_one = amount.0 as i16;
        let amount_two = amount.1 as i16;
        let capacity_one = capacity.0 as i16;
        let capacity_two = capacity.1 as i16;

        [(capacity_one, amount_two),
            ((amount_one, capacity_two)),
            ((0, amount_two)),
            ((amount_one, 0)),
            ((amount_one + amount_two - capacity_two, capacity_two)),
            ((0, amount_one + amount_two)),
            ((capacity_one, amount_one + amount_two - capacity_one)),
            ((amount_one + amount_two, 0))].iter()
            .filter(|b| b.0 >= 0 &&
                b.0 <= capacity_one && b.1 >= 0 &&
                b.1 <= capacity_two)
            .filter_map(|b| {
                find(
                    capacity,
                    (b.0 as u8, b.1 as u8),
                    goal,
                    moves + 1,
                    stops,
                )
            })
            .next()
    }
}

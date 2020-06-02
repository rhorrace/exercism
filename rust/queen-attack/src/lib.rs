#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let f = |&n| (0..8).any(|x| x == n);
        match f(&rank) && f(&file) {
            true => Some(Self(rank, file)),
            false => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            position: position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let this_pos = &self.position;
        let other_pos = &other.position;
        this_pos.0 == other_pos.0 ||
            this_pos.1 == other_pos.1 ||
            (this_pos.0 - other_pos.0).abs() == (this_pos.1 - other_pos.1).abs()
    }
}

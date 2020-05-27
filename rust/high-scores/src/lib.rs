#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec()
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.iter().last() {
            Some(&n) => Some(n),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(&n) => Some(n),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp_scores = self.scores.clone();
        temp_scores.sort();
        temp_scores.iter()
            .rev()
            .map(|&x| x)
            .take(3)
            .collect::<Vec<u32>>()
    }
}

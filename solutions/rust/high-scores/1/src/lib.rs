#[derive(Debug)]
pub struct HighScores<'a> {
    scores_vec: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores_vec: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores_vec
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores_vec.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_vec.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_3: Vec<u32> = Vec::from(self.scores_vec);
        top_3.sort();
        top_3.reverse();
        top_3.truncate(3);
        top_3
    }
}

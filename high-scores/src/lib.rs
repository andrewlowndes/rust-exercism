#[derive(Debug)]
pub struct HighScores<'a> {
    counts: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { counts: scores }
    }

    pub fn scores(&'a self) -> &'a [u32] {
        self.counts
    }

    pub fn latest(&self) -> Option<u32> {
        self.counts.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.counts.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_counts = self.counts.to_owned();
        sorted_counts.sort_by(|a, b| b.cmp(a));
        sorted_counts.into_iter().take(3).collect()
    }
}

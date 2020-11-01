#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }

        Some(ChessPosition { file, rank })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    fn is_same_vertical(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
    }

    fn is_same_horizontal(&self, other: &Queen) -> bool {
        self.position.file == other.position.file
    }

    fn is_same_diagonal(&self, other: &Queen) -> bool {
        let dy = (other.position.rank - self.position.rank).abs();
        let dx = (other.position.file - self.position.file).abs();
        dx == dy
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.is_same_vertical(other)
            || self.is_same_horizontal(other)
            || self.is_same_diagonal(other)
    }
}

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
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // Check if queens are on the same rank (row) or file (column)
        if self.position.rank == other.position.rank || self.position.file == other.position.file    {
            return true;
        }
      
        // Check if queens are on the same diagonal
        let rank_diff = (self.position.rank - other.position.rank).abs();
        let file_diff = (self.position.file - other.position.file).abs();
        rank_diff == file_diff
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CastlingRights {
    pub king_side: bool,
    pub queen_side: bool
}

impl CastlingRights {
    pub fn new() -> Self {
        return CastlingRights {
            king_side: false,
            queen_side: false
        }
    }
}
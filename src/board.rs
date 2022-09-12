#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Space {
    piece: Piece,
    side: Side,
}

#[derive(Clone, Debug)]
pub struct Board([Option<Space>; 128]);

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Board([
            // Rank 1
            Some(Space { piece: Piece::Rook, side: Side::White }), Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
            Some(Space { piece: Piece::Queen, side: Side::White }), Some(Space { piece: Piece::King, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
            Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Rook, side: Side::White }), None, None, None, None, None, None, None, None,
            // Rank 2
            Some(Space { piece: Piece::Pawn, side: Side::White }), Some(Space { piece: Piece::Pawn, side: Side::White }), Some(Space { piece: Piece::Pawn, side: Side::White }),
            Some(Space { piece: Piece::Pawn, side: Side::White }), Some(Space { piece: Piece::Pawn, side: Side::White }), Some(Space { piece: Piece::Pawn, side: Side::White }),
            Some(Space { piece: Piece::Pawn, side: Side::White }), Some(Space { piece: Piece::Pawn, side: Side::White }), None, None, None, None, None, None, None, None,
            // Rank 3 - 6
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            // Rank 7
            Some(Space { piece: Piece::Pawn, side: Side::Black }), Some(Space { piece: Piece::Pawn, side: Side::Black }), Some(Space { piece: Piece::Pawn, side: Side::Black }),
            Some(Space { piece: Piece::Pawn, side: Side::Black }), Some(Space { piece: Piece::Pawn, side: Side::Black }), Some(Space { piece: Piece::Pawn, side: Side::Black }),
            Some(Space { piece: Piece::Pawn, side: Side::Black }), Some(Space { piece: Piece::Pawn, side: Side::Black }), None, None, None, None, None, None, None, None,
            // Rank 8 
            Some(Space { piece: Piece::Rook, side: Side::Black }), Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
            Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
            Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Rook, side: Side::Black }), None, None, None, None, None, None, None, None,    
        ])
    }

    pub fn generate_moves(&self, player: Side) {
        let new_board = self.clone();

        for (i, space) in self.0.iter().enumerate() {
            if let Some(space) = space {
                if space.side == player {
                    match space.piece {
                        Piece::King => todo!(),
                        Piece::Queen => todo!(),
                        Piece::Rook => todo!(),
                        Piece::Knight => todo!(),
                        Piece::Bishop => {
                            let index = i + 17;
                            if index & 0x88 != 0 {
                                match self.0[index] {
                                    Some(Space { side: Side::White, .. }) => {},
                                    Some(Space { side: Side::Black, .. }) => {},
                                    // Valid position to move to
                                    None => {
                                        let mut new_board = self.clone();
                                        new_board.0[i + 17] = new_board.0[i];
                                        new_board.0[i] = None;
                                    },
                                }
                            }
                        },
                        Piece::Pawn => todo!(),
                    }
                }                
            }
        }

        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

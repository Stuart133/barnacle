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

    pub fn generate_moves(&self, player: Side) -> Vec<Board> {
        let mut moves = vec![];

        for (i, space) in self.0.iter().enumerate() {
            if let Some(space) = space {
                if space.side == player {
                    match space.piece {
                        Piece::King => todo!(),
                        Piece::Queen => todo!(),
                        Piece::Rook => todo!(),
                        Piece::Knight => todo!(),
                        Piece::Bishop => {
                            let mut index = i;
                            loop {
                                index += 17;
                                if index & 0x88 != 0 {
                                    match self.0[index] {
                                        Some(target) => {
                                            if target.side != player {
                                                moves.push(self.make_move(i, index));
                                            }
                                            break;
                                        },
                                        None => {
                                            moves.push(self.make_move(i, index));
                                        },
                                    }
                                }
                            }
                        },
                        Piece::Pawn => todo!(),
                    }
                }                
            }
        }

        moves
    }

    #[inline(always)]
    fn make_move(&self, src: usize, dest: usize) -> Board {
        let mut new_board = self.clone();
        new_board.0[dest] = new_board.0[src];
        new_board.0[src] = None;

        new_board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

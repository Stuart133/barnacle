// Directional movement offsets using 0x88 board representation
// Missing directions are inverts of these (So we subtract)
const UP_LEFT: usize = 15;
const UP: usize = 16;
const UP_RIGHT: usize = 17;
const RIGHT: usize = 1;

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
                        Piece::Bishop => self.generate_bishop_moves(&mut moves, i),
                        Piece::Pawn => todo!(),
                    }
                }
            }
        }

        moves
    }

    // Individual peice move functions to ease testing
    #[inline(always)]
    fn generate_bishop_moves(&self, moves: &mut Vec<Board>, i: usize) {
        self.make_sliding_moves(moves, i, |i| i + UP_RIGHT);
        self.make_sliding_moves(moves, i, |i| i + UP_LEFT);
        self.make_sliding_moves(moves, i, |i| {
            // Invert UP_RIGHT becomes DOWN_LEFT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP_RIGHT) {
                Some(i) => i,
                None => 0x88,
            }
        });
        self.make_sliding_moves(moves, i, |i| {
            // Invert UP_LEFT becomes DOWN_RIGHT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP_LEFT) {
                Some(i) => i,
                None => 0x88,
            }
        });
    }

    fn make_sliding_moves(&self, moves: &mut Vec<Board>, i: usize, index_exp: fn(usize) -> usize) {
        let mut index = i;
        loop {
            index = index_exp(index);
            if index & 0x88 == 0 {
                match self.0[index] {
                    Some(target) => {
                        if target.side
                            != self.0[i].expect("sliding move called on empty space").side
                        {
                            moves.push(self.make_move(i, index));
                        }
                        break;
                    }
                    None => {
                        moves.push(self.make_move(i, index));
                    }
                }
            } else {
                break;
            }
        }
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

    #[test]
    pub fn bishop_moves_from_start() {
        let board = Board::new();
        let mut moves = vec![];

        // C1
        board.generate_bishop_moves(&mut moves, 2);
        assert_eq!(0, moves.len());
        // F1
        board.generate_bishop_moves(&mut moves, 5);
        assert_eq!(0, moves.len());
        // C8
        board.generate_bishop_moves(&mut moves, 114);
        assert_eq!(0, moves.len());
        // F8
        board.generate_bishop_moves(&mut moves, 117);
        assert_eq!(0, moves.len());
        assert_eq!(Piece::Bishop, board.0[117].unwrap().piece);
    }

    #[test]
    pub fn bishop_moves_from_middle() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place bishop on D5
        board.0[67] = Some(Space {
            piece: Piece::Bishop,
            side: Side::White,
        });

        board.generate_bishop_moves(&mut moves, 67);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_side() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place bishop on B5
        board.0[65] = Some(Space {
            piece: Piece::Bishop,
            side: Side::White,
        });

        board.generate_bishop_moves(&mut moves, 65);
        assert_eq!(6, moves.len());
    }
}

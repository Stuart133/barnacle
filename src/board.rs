const BOARD_SIZE: usize = 8;

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

impl Space {
    fn generate_move_vector(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self.piece {
            Piece::King => todo!(),
            Piece::Queen => todo!(),
            Piece::Rook => {
                vec![
                    (x, 0),
                    (x, 1),
                    (x, 2),
                    (x, 3),
                    (x, 4),
                    (x, 5),
                    (x, 6),
                    (x, 7),
                    (0, y),
                    (1, y),
                    (2, y),
                    (3, y),
                    (4, y),
                    (5, y),
                    (6, y),
                    (7, y),
                ]
            }
            Piece::Knight => todo!(),
            Piece::Bishop => todo!(),
            Piece::Pawn => {
                if self.side == Side::Black {
                    if y < BOARD_SIZE - 1 {
                        vec![(x, y + 1)]
                    } else {
                        vec![]
                    }
                } else {
                    if y > 0 {
                        vec![(x, y - 1)]
                    } else {
                        vec![]
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct X88Board([Option<Space>; 128]);

#[derive(Clone, Debug)]
pub struct Board([[Option<Space>; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
      Board([
        [Some(Space { piece: Piece::Rook, side: Side::Black }), Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
        Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
        Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Rook, side: Side::Black })],
        [Some(Space { piece: Piece::Pawn, side: Side::Black }); BOARD_SIZE],
        [None; BOARD_SIZE],
        [None; BOARD_SIZE],
        [None; BOARD_SIZE],
        [None; BOARD_SIZE],
        [Some(Space { piece: Piece::Pawn, side: Side::White }); BOARD_SIZE],
        [Some(Space { piece: Piece::Rook, side: Side::White }), Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
        Some(Space { piece: Piece::King, side: Side::White }), Some(Space { piece: Piece::Queen, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
        Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Rook, side: Side::White })],
      ])
    }

    pub fn generate_moves(&self, player: Side) {
        let new_board = self.clone();

        for (j, row) in self.0.iter().enumerate() {
            for (i, space) in row.iter().enumerate() {
                if let Some(space) = space {
                    if space.side == player {
                        match space.piece {
                            Piece::King => todo!(),
                            Piece::Queen => todo!(),
                            Piece::Rook => todo!(),
                            Piece::Knight => todo!(),
                            Piece::Bishop => todo!(),
                            Piece::Pawn => todo!(),
                        }
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

    #[test]
    pub fn generate_white_pawn_vector() {
        let space = Space {
            piece: Piece::Pawn,
            side: Side::White,
        };

        let vector = space.generate_move_vector(3, 2);
        assert_eq!(vector.len(), 1);
        assert_eq!(vector[0], (3, 1));

        let vector = space.generate_move_vector(4, 0);
        assert_eq!(vector.len(), 0);
    }

    #[test]
    pub fn generate_black_pawn_vector() {
        let space = Space {
            piece: Piece::Pawn,
            side: Side::Black,
        };

        let vector = space.generate_move_vector(5, 2);
        assert_eq!(vector.len(), 1);
        assert_eq!(vector[0], (5, 3));

        let vector = space.generate_move_vector(6, 7);
        assert_eq!(vector.len(), 0);
    }
}

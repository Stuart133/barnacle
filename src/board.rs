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
pub struct Board([[Option<Space>; 8]; 8]);

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
      Board([
        [Some(Space { piece: Piece::Rook, side: Side::White }), Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
          Some(Space { piece: Piece::King, side: Side::White }), Some(Space { piece: Piece::Queen, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
          Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Rook, side: Side::White })],
        [Some(Space { piece: Piece::Pawn, side: Side::White }); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Space { piece: Piece::Pawn, side: Side::Black }); 8],
        [Some(Space { piece: Piece::Rook, side: Side::Black }), Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
          Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
          Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Rook, side: Side::Black })],
      ])
    }

    pub fn generate_moves(&self, player: Side) {
        let new_board = self.clone();

        for row in self.0.iter() {
            for space in row.iter() {
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

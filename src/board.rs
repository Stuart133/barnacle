#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Space {
    piece: Piece,
    side: Side,
}

pub struct Board([[Option<Space>; 8]; 8]);

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
      Board([
        [Some(Space { piece: Piece::Rook, side: Side::White }), Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
          Some(Space { piece: Piece::Queen, side: Side::White }), Some(Space { piece: Piece::King, side: Side::White }), Some(Space { piece: Piece::Bishop, side: Side::White }),
          Some(Space { piece: Piece::Knight, side: Side::White }), Some(Space { piece: Piece::Rook, side: Side::White })],
        [Some(Space { piece: Piece::Pawn, side: Side::White }); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Space { piece: Piece::Pawn, side: Side::Black }); 8],
        [Some(Space { piece: Piece::Rook, side: Side::Black }), Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
          Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
          Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Rook, side: Side::Black })],
      ])
    }
}

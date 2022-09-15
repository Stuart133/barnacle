// Directional movement offsets using 0x88 board representation
// Missing directions are inverts of these (So we subtract)
const UP_LEFT: usize = 15;
const UP: usize = 16;
const UP_RIGHT: usize = 17;
const RIGHT: usize = 1;
const KNIGHT_MOVES: [usize; 4] = [14, 18, 31, 33];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn(bool), // Store the en passent status of the pawn
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
            Some(Space { piece: Piece::Pawn(false), side: Side::White }), Some(Space { piece: Piece::Pawn(false), side: Side::White }), Some(Space { piece: Piece::Pawn(false), side: Side::White }),
            Some(Space { piece: Piece::Pawn(false), side: Side::White }), Some(Space { piece: Piece::Pawn(false), side: Side::White }), Some(Space { piece: Piece::Pawn(false), side: Side::White }),
            Some(Space { piece: Piece::Pawn(false), side: Side::White }), Some(Space { piece: Piece::Pawn(false), side: Side::White }), None, None, None, None, None, None, None, None,
            // Rank 3 - 6
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            // Rank 7
            Some(Space { piece: Piece::Pawn(false), side: Side::Black }), Some(Space { piece: Piece::Pawn(false), side: Side::Black }), Some(Space { piece: Piece::Pawn(false), side: Side::Black }),
            Some(Space { piece: Piece::Pawn(false), side: Side::Black }), Some(Space { piece: Piece::Pawn(false), side: Side::Black }), Some(Space { piece: Piece::Pawn(false), side: Side::Black }),
            Some(Space { piece: Piece::Pawn(false), side: Side::Black }), Some(Space { piece: Piece::Pawn(false), side: Side::Black }), None, None, None, None, None, None, None, None,
            // Rank 8 
            Some(Space { piece: Piece::Rook, side: Side::Black }), Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
            Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Bishop, side: Side::Black }),
            Some(Space { piece: Piece::Knight, side: Side::Black }), Some(Space { piece: Piece::Rook, side: Side::Black }), None, None, None, None, None, None, None, None,
        ])
    }

    pub fn generate_ply(&self, player: Side) -> Vec<Board> {
        let mut moves = vec![];

        for (i, space) in self.0.iter().enumerate() {
            if let Some(space) = space {
                if space.side == player {
                    match space.piece {
                        Piece::King => self.generate_king_moves(&mut moves, i),
                        Piece::Queen => self.generate_queen_moves(&mut moves, i),
                        Piece::Rook => self.generate_rook_moves(&mut moves, i),
                        Piece::Knight => self.generate_knight_moves(&mut moves, i),
                        Piece::Bishop => self.generate_bishop_moves(&mut moves, i),
                        Piece::Pawn(_) => self.generate_pawn_moves(&mut moves, i),
                    }
                }
            }
        }

        moves
    }

    // Is the king is check in this position?
    fn king_check(&self, side: Side, position: usize) -> bool {
        // Check knight attacks
        KNIGHT_MOVES.iter().fold(false, |val, offset| {
            self.king_check_inner(side, position, Piece::Knight, val, offset)
        }) ||
        // // Check rook attacks
        [UP, RIGHT].iter().fold(false, |val, offset| {
            self.king_check_inner(side, position, Piece::Rook, val, offset)
        }) ||
        // Check bishop attacks
        [UP_LEFT, UP_RIGHT].iter().fold(false, |val, offset| {
            self.king_check_inner(side, position, Piece::Bishop, val, offset)
        }) ||
        // Check queen attacks
        [UP, RIGHT, UP_LEFT, UP_RIGHT]
            .iter()
            .fold(false, |val, offset| {
                self.king_check_inner(side, position, Piece::Queen, val, offset)
            }) ||
        // Check pawn attacks
        [UP_LEFT, UP_RIGHT].iter().fold(false, |val, offset| {
            // Detect black attacking pawns - which from above
            if side == Side::White && (position + offset) & 0x88 == 0 {
                if let Some(Space {
                    piece: Piece::Pawn(_),
                    side: Side::Black,
                }) = self.0[position + offset]
                {
                    return true;
                };

            false && val
            // Detect white attacking pawns - which attack from below
            } else {
                if let Some(attack) = position.checked_sub(*offset) {
                    if let Some(Space {
                        piece: Piece::Pawn(_),
                        side: Side::White,
                    }) = self.0[attack]
                    {
                        return true;
                    }
                }

                false && val
            }
        })
    }

    fn king_check_inner(
        &self,
        side: Side,
        position: usize,
        attack_piece: Piece,
        val: bool,
        offset: &usize,
    ) -> bool {
        let mut attack = position;
        loop {
            attack += offset;
            if attack & 0x88 == 0 {
                if let Some(space) = self.0[attack] {
                    if space.side != side && space.piece == attack_piece {
                        return true;
                    } else {
                        return false || val;
                    }
                }
            } else {
                break;
            }
        }

        attack = position;
        loop {
            attack = match attack.checked_sub(*offset) {
                Some(new) => new,
                None => 0x88,
            };
            if attack & 0x88 == 0 {
                if let Some(space) = self.0[attack] {
                    if space.side != side && space.piece == attack_piece {
                        return true;
                    } else {
                        return false || val;
                    }
                }
            } else {
                break;
            }
        }

        false || val
    }

    // Individual peice move functions to ease testing
    #[inline(always)]
    fn generate_king_moves(&self, moves: &mut Vec<Board>, src: usize) {
        [UP_RIGHT, UP, UP_LEFT, RIGHT].iter().for_each(|offset| {
            self.make_checked_jump(moves, src, src + offset, Board::king_check);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_checked_jump(moves, src, dest, Board::king_check),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, moves: &mut Vec<Board>, src: usize) {
        // TODO: En passent
        match self.0[src]
            .expect("generate pawn moves called on empty space")
            .side
        {
            Side::White => {
                let dest = src + UP;
                if dest & 0x88 == 0 {
                    if let None = self.0[dest] {
                        moves.push(self.make_move(src, dest));
                    }
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test as this can't be off board
                if src <= 0x17 && src >= 0x10 {
                    let dest = src + UP + UP;
                    if let None = self.0[dest] {
                        moves.push(self.make_move(src, dest));
                    }
                }
                let dest = src + UP_RIGHT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.0[dest]
                    {
                        moves.push(self.make_move(src, dest));
                    }
                }
                let dest = src + UP_LEFT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.0[dest]
                    {
                        moves.push(self.make_move(src, dest));
                    }
                }
            }
            Side::Black => {
                match src.checked_sub(UP) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let None = self.0[dest] {
                                moves.push(self.make_move(src, dest));
                            }
                        }
                    }
                    None => {}
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test & checked sub as this can't be off board
                if src <= 0x67 && src >= 0x60 {
                    let dest = src - UP - UP;
                    if let None = self.0[dest] {
                        moves.push(self.make_move(src, dest));
                    }
                }
                match src.checked_sub(UP_RIGHT) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let Some(Space {
                                side: Side::White, ..
                            }) = self.0[dest]
                            {
                                moves.push(self.make_move(src, dest));
                            }
                        }
                    }
                    None => {}
                }
                match src.checked_sub(UP_LEFT) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let Some(Space {
                                side: Side::White, ..
                            }) = self.0[dest]
                            {
                                moves.push(self.make_move(src, dest));
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }

    #[inline(always)]
    fn generate_knight_moves(&self, moves: &mut Vec<Board>, src: usize) {
        KNIGHT_MOVES.iter().for_each(|offset| {
            self.make_jump_move(moves, src, src + offset);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_jump_move(moves, src, dest),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_queen_moves(&self, moves: &mut Vec<Board>, src: usize) {
        // Queen moves as the union of rook and bishop
        self.generate_rook_moves(moves, src);
        self.generate_bishop_moves(moves, src);
    }

    #[inline(always)]
    fn generate_bishop_moves(&self, moves: &mut Vec<Board>, src: usize) {
        self.make_sliding_moves(moves, src, |i| i + UP_RIGHT);
        self.make_sliding_moves(moves, src, |i| i + UP_LEFT);
        self.make_sliding_moves(moves, src, |i| {
            // Invert UP_RIGHT becomes DOWN_LEFT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP_RIGHT) {
                Some(i) => i,
                None => 0x88,
            }
        });
        self.make_sliding_moves(moves, src, |i| {
            // Invert UP_LEFT becomes DOWN_RIGHT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP_LEFT) {
                Some(i) => i,
                None => 0x88,
            }
        });
    }

    #[inline(always)]
    fn generate_rook_moves(&self, moves: &mut Vec<Board>, src: usize) {
        self.make_sliding_moves(moves, src, |i| i + RIGHT);
        self.make_sliding_moves(moves, src, |i| i + UP);
        self.make_sliding_moves(moves, src, |i| {
            // Invert UP becomes DOWN
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP) {
                Some(i) => i,
                None => 0x88,
            }
        });
        self.make_sliding_moves(moves, src, |i| {
            // Invert RIGHT becomes LEFT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(RIGHT) {
                Some(i) => i,
                None => 0x88,
            }
        });
    }

    fn make_sliding_moves(
        &self,
        moves: &mut Vec<Board>,
        src: usize,
        index_exp: fn(usize) -> usize,
    ) {
        let mut dest = src;
        loop {
            dest = index_exp(dest);
            if dest & 0x88 == 0 {
                match self.0[dest] {
                    Some(target) => {
                        if target.side
                            != self.0[src]
                                .expect("sliding move called on empty space")
                                .side
                        {
                            moves.push(self.make_move(src, dest));
                        }
                        break;
                    }
                    None => {
                        moves.push(self.make_move(src, dest));
                    }
                }
            } else {
                break;
            }
        }
    }

    fn make_checked_jump(
        &self,
        moves: &mut Vec<Board>,
        src: usize,
        dest: usize,
        check: fn(&Board, Side, usize) -> bool,
    ) {
        if dest & 0x88 == 0 {
            if check(
                self,
                self.0[src].expect("jump move called on empty space").side,
                dest,
            ) {
                return;
            }
            match self.0[dest] {
                Some(target) => {
                    if target.side != self.0[src].unwrap().side {
                        moves.push(self.make_move(src, dest));
                    }
                }
                None => moves.push(self.make_move(src, dest)),
            }
        }
    }

    fn make_jump_move(&self, moves: &mut Vec<Board>, src: usize, dest: usize) {
        self.make_checked_jump(moves, src, dest, |_, _, _| false)
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
    // This is the master correctness test, if it's wrong then the move generator is not working correctly
    // See https://www.chessprogramming.org/Perft for more details
    pub fn perft() {
        let correct_values = [20, 400, 8902, 197281];

        let board = Board::new();
        let mut side = Side::White;
        let mut moves = vec![board];

        for value in correct_values {
            let mut new_moves = vec![];
            for m in moves.iter() {
                new_moves.append(&mut m.generate_ply(side));
            }

            assert_eq!(value, new_moves.len());
            moves = new_moves;
            if side == Side::White {
                side = Side::Black;
            } else {
                side = Side::White;
            }
        }
    }

    #[test]
    pub fn king_moves_from_start() {
        let board = Board::new();
        let mut moves = vec![];

        // E1
        board.generate_king_moves(&mut moves, 4);
        assert_eq!(0, moves.len());
        // E8
        board.generate_king_moves(&mut moves, 0x74);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn king_moves_from_middle() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place king on D5
        board.0[0x43] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Place pawn on E5
        board.0[0x44] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        board.generate_king_moves(&mut moves, 0x4367);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn king_moves_with_potential_check() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place king on D5
        board.0[0x43] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Place pawn on C5
        board.0[0x42] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });
        // Place bishop on C7
        board.0[0x62] = Some(Space {
            piece: Piece::Bishop,
            side: Side::Black,
        });

        board.generate_king_moves(&mut moves, 0x43);
        assert_eq!(5, moves.len());
    }

    #[test]
    pub fn queen_moves_from_start() {
        let board = Board::new();
        let mut moves = vec![];

        // D1
        board.generate_queen_moves(&mut moves, 3);
        assert_eq!(0, moves.len());
        // D8
        board.generate_queen_moves(&mut moves, 0x73);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn queen_moves_from_middle() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place queen on D5
        board.0[67] = Some(Space {
            piece: Piece::Queen,
            side: Side::White,
        });

        board.generate_queen_moves(&mut moves, 67);
        assert_eq!(19, moves.len());
    }

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

    #[test]
    pub fn rook_moves_from_start() {
        let board = Board::new();
        let mut moves = vec![];

        // A1
        board.generate_rook_moves(&mut moves, 0);
        assert_eq!(0, moves.len());
        // H1
        board.generate_rook_moves(&mut moves, 7);
        assert_eq!(0, moves.len());
        // A8
        board.generate_rook_moves(&mut moves, 112);
        assert_eq!(0, moves.len());
        // H8
        board.generate_rook_moves(&mut moves, 119);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn rook_moves_from_middle() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place rook on D5
        board.0[67] = Some(Space {
            piece: Piece::Rook,
            side: Side::White,
        });

        board.generate_rook_moves(&mut moves, 67);
        assert_eq!(11, moves.len());
    }

    #[test]
    pub fn knight_moves_from_start() {
        let board = Board::new();
        let mut moves = vec![];

        // B1
        board.generate_knight_moves(&mut moves, 1);
        assert_eq!(2, moves.len());
        // G1
        board.generate_knight_moves(&mut moves, 6);
        assert_eq!(4, moves.len());
        // B8
        board.generate_knight_moves(&mut moves, 113);
        assert_eq!(6, moves.len());
        // G8
        board.generate_knight_moves(&mut moves, 118);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_middle() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place rook on D5
        board.0[67] = Some(Space {
            piece: Piece::Knight,
            side: Side::White,
        });

        board.generate_knight_moves(&mut moves, 67);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_side() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place rook on D5
        board.0[64] = Some(Space {
            piece: Piece::Knight,
            side: Side::White,
        });

        board.generate_knight_moves(&mut moves, 64);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_from_start() {
        let board = Board::new();
        let mut moves = vec![];

        // A2
        board.generate_pawn_moves(&mut moves, 17);
        assert_eq!(2, moves.len());
        // E2
        board.generate_pawn_moves(&mut moves, 21);
        assert_eq!(4, moves.len());
        // B7
        board.generate_pawn_moves(&mut moves, 97);
        assert_eq!(6, moves.len());
        // G7
        board.generate_pawn_moves(&mut moves, 102);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn pawn_move_from_center() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place pawn on D4
        board.0[0x33] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });
        // Place pawn on C5
        board.0[0x42] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        board.generate_pawn_moves(&mut moves, 0x33);
        assert_eq!(2, moves.len());

        board.generate_pawn_moves(&mut moves, 0x42);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_friendly_front() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place pawn on D5
        board.0[67] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });
        // Place pawn on D6
        board.0[83] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });

        board.generate_pawn_moves(&mut moves, 67);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_enemy_front() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place pawn on D5
        board.0[67] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });
        // Place pawn on D6
        board.0[83] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        board.generate_pawn_moves(&mut moves, 67);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_capture_front() {
        let mut board = Board::new();
        let mut moves = vec![];

        // Place pawn on D6
        board.0[0x53] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });

        board.generate_pawn_moves(&mut moves, 0x53);
        assert_eq!(2, moves.len());
    }

    #[test]
    pub fn king_not_in_check_from_start() {
        let board = Board::new();

        // E1
        assert!(!board.king_check(Side::White, 0x04));

        // E8
        assert!(!board.king_check(Side::Black, 0x74));
    }

    #[test]
    pub fn king_in_check_from_bishop() {
        let mut board = Board::new();

        // King at D4
        board.0[0x33] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Bishop on B6
        board.0[0x51] = Some(Space {
            piece: Piece::Bishop,
            side: Side::Black,
        });

        // E8
        assert!(board.king_check(Side::White, 0x33));
    }

    #[test]
    pub fn king_in_check_from_pawn() {
        let mut board = Board::new();

        // King at D4
        board.0[0x33] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Pawn on C5
        board.0[0x42] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        // E8
        assert!(board.king_check(Side::White, 0x33));
    }
}

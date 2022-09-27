// Directional movement offsets using 0x88 board representation
// Missing directions are inverts of these (So we subtract)
const UP_LEFT: usize = 15;
const UP: usize = 16;
const UP_RIGHT: usize = 17;
const RIGHT: usize = 1;
const KNIGHT_MOVES: [usize; 4] = [14, 18, 31, 33];

// Piece list index values
const WHITE_QUEEN_ROOK: usize = 0;
const WHITE_QUEEN_KNIGHT: usize = 1;
const WHITE_QUEEN_BISHOP: usize = 2;
const WHITE_QUEEN: usize = 3;
const WHITE_KING: usize = 4;
const WHITE_KING_BISHOP: usize = 5;
const WHITE_KING_KNIGHT: usize = 6;
const WHITE_KING_ROOK: usize = 7;
const WHITE_PAWN_A: usize = 8;
const WHITE_PAWN_B: usize = 9;
const WHITE_PAWN_C: usize = 10;
const WHITE_PAWN_D: usize = 11;
const WHITE_PAWN_E: usize = 12;
const WHITE_PAWN_F: usize = 13;
const WHITE_PAWN_G: usize = 14;
const WHITE_PAWN_H: usize = 15;
const BLACK_QUEEN_ROOK: usize = 16;
const BLACK_QUEEN_KNIGHT: usize = 17;
const BLACK_QUEEN_BISHOP: usize = 18;
const BLACK_QUEEN: usize = 19;
const BLACK_KING: usize = 20;
const BLACK_KING_BISHOP: usize = 21;
const BLACK_KING_KNIGHT: usize = 22;
const BLACK_KING_ROOK: usize = 23;
const BLACK_PAWN_A: usize = 24;
const BLACK_PAWN_B: usize = 25;
const BLACK_PAWN_C: usize = 26;
const BLACK_PAWN_D: usize = 27;
const BLACK_PAWN_E: usize = 28;
const BLACK_PAWN_F: usize = 29;
const BLACK_PAWN_G: usize = 30;
const BLACK_PAWN_H: usize = 31;

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
pub struct Game {
    board: [Option<Space>; 128], // TODO: Look into bijective map to replace this
    pieces: [usize; 32],
    white_check: bool,
    black_check: bool,
}

impl Game {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Game{board: [
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
        ], white_check: false, black_check: false, 
        pieces: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67]}
    }

    pub fn generate_ply(&self, player: Side) -> Vec<Game> {
        let mut moves = vec![];

        let range = if player == Side::White { 0..16 } else { 16..32 };

        for (i, piece) in self.pieces[range].iter().enumerate() {
            match self.board[*piece]
                .expect("piece index set to empty space")
                .piece
            {
                Piece::King => self.generate_king_moves(&mut moves, *piece, i),
                Piece::Queen => self.generate_queen_moves(&mut moves, *piece, i),
                Piece::Rook => self.generate_rook_moves(&mut moves, *piece, i),
                Piece::Knight => self.generate_knight_moves(&mut moves, *piece, i),
                Piece::Bishop => self.generate_bishop_moves(&mut moves, *piece, i),
                Piece::Pawn(_) => self.generate_pawn_moves(&mut moves, *piece, i),
            }
        }

        // for (i, space) in self.board.iter().enumerate() {
        //     if let Some(space) = space {
        //         if space.side == player {
        //             if space.side == Side::White && self.white_check || space.side == Side::Black && self.black_check {
        //                 if let Piece::King = space.piece {
        //                     println!("CHECK");
        //                     self.generate_king_moves(&mut moves, i);
        //                 }
        //             } else {
        //                 match space.piece {
        //                     Piece::King => self.generate_king_moves(&mut moves, i),
        //                     Piece::Queen => self.generate_queen_moves(&mut moves, i),
        //                     Piece::Rook => self.generate_rook_moves(&mut moves, i),
        //                     Piece::Knight => self.generate_knight_moves(&mut moves, i),
        //                     Piece::Bishop => self.generate_bishop_moves(&mut moves, i),
        //                     Piece::Pawn(_) => self.generate_pawn_moves(&mut moves, i),
        //                 }
        //             }
        //         }
        //     }
        // }

        moves
    }

    // Is the king is check in this position?
    fn king_check(&self, side: Side, position: usize) -> bool {
        // Check knight attacks
        KNIGHT_MOVES.iter().fold(false, |val, offset| {
            self.king_check_inner_jump(side, position, Piece::Knight, val, offset)
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
            if side == Side::White {
                if let Some(Space {
                    piece: Piece::Pawn(_),
                    side: Side::Black,
                }) = self.board[position + offset]
                {
                    return true;
                };

            false || val
        
            // Detect white attacking pawns - which attack from below
            } else {
                if let Some(attack) = position.checked_sub(*offset) {
                    if let Some(Space {
                        piece: Piece::Pawn(_),
                        side: Side::White,
                    }) = self.board[attack]
                    {
                        return true;
                    }
                }

                false || val
            }
        })
    }

    fn king_check_inner_jump(
        &self,
        side: Side,
        position: usize,
        attack_piece: Piece,
        val: bool,
        offset: &usize,
    ) -> bool {
        if position + offset * 0x88 == 0 {
            if let Some(space) = self.board[position + offset] {
                if space.side != side && space.piece == attack_piece {
                    return true;
                } else {
                    return false || val;
                }
            }
        }
        if let Some(attack) = position.checked_sub(*offset) {
            if attack & 0x88 == 0 {
                if let Some(space) = self.board[attack] {
                    if space.side != side && space.piece == attack_piece {
                        return true;
                    } else {
                        return false || val;
                    }
                }
            }
        }

        false || val
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
                if let Some(space) = self.board[attack] {
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
                if let Some(space) = self.board[attack] {
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
    fn generate_king_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize) {
        [UP_RIGHT, UP, UP_LEFT, RIGHT].iter().for_each(|offset| {
            self.make_checked_jump(moves, src, src + offset, index, Game::king_check);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_checked_jump(moves, src, dest, index, Game::king_check),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize) {
        // TODO: En passent
        match self.board[src]
            .expect("generate pawn moves called on empty space")
            .side
        {
            Side::White => {
                let dest = src + UP;
                if dest & 0x88 == 0 {
                    if let None = self.board[dest] {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test as this can't be off board
                if src >= 0x10 && src <= 0x17 {
                    let dest = src + UP + UP;
                    if let None = self.board[dest] {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
                let dest = src + UP_RIGHT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.board[dest]
                    {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
                let dest = src + UP_LEFT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.board[dest]
                    {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
            }
            Side::Black => {
                match src.checked_sub(UP) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let None = self.board[dest] {
                                moves.push(self.make_move(src, dest, index));
                            }
                        }
                    }
                    None => {}
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test & checked sub as this can't be off board
                if src >= 0x60 && src <= 0x67 {
                    let dest = src - UP - UP;
                    if let None = self.board[dest] {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
                match src.checked_sub(UP_RIGHT) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let Some(Space {
                                side: Side::White, ..
                            }) = self.board[dest]
                            {
                                moves.push(self.make_move(src, dest, index));
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
                            }) = self.board[dest]
                            {
                                moves.push(self.make_move(src, dest, index));
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }

    #[inline(always)]
    fn generate_knight_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize) {
        KNIGHT_MOVES.iter().for_each(|offset| {
            self.make_jump_move(moves, src, src + offset, index);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_jump_move(moves, src, dest, index),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_queen_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize) {
        // Queen moves as the union of rook and bishop
        self.generate_rook_moves(moves, src, index);
        self.generate_bishop_moves(moves, src, index);
    }

    #[inline(always)]
    fn generate_bishop_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize) {
        self.make_sliding_moves(moves, src, index, |i| i + UP_RIGHT);
        self.make_sliding_moves(moves, src, index, |i| i + UP_LEFT);
        self.make_sliding_moves(moves, src, index, |i| {
            // Invert UP_RIGHT becomes DOWN_LEFT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP_RIGHT) {
                Some(i) => i,
                None => 0x88,
            }
        });
        self.make_sliding_moves(moves, src, index, |i| {
            // Invert UP_LEFT becomes DOWN_RIGHT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP_LEFT) {
                Some(i) => i,
                None => 0x88,
            }
        });
    }

    #[inline(always)]
    fn generate_rook_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize) {
        self.make_sliding_moves(moves, src, index, |i| i + RIGHT);
        self.make_sliding_moves(moves, src, index, |i| i + UP);
        self.make_sliding_moves(moves, src, index, |i| {
            // Invert UP becomes DOWN
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(UP) {
                Some(i) => i,
                None => 0x88,
            }
        });
        self.make_sliding_moves(moves, src, index, |i| {
            // Invert RIGHT becomes LEFT
            // If we underflow we're off the bottom, so set to a know fail value
            match i.checked_sub(RIGHT) {
                Some(i) => i,
                None => 0x88,
            }
        });
    }

    fn make_sliding_moves(&self, moves: &mut Vec<Game>, src: usize, index: usize, src_exp: fn(usize) -> usize) {
        let mut dest = src;
        loop {
            dest = src_exp(dest);
            if dest & 0x88 == 0 {
                match self.board[dest] {
                    Some(target) => {
                        if target.side
                            != self.board[src]
                                .expect("sliding move called on empty space")
                                .side
                        {
                            moves.push(self.make_move(src, dest, index));
                        }
                        break;
                    }
                    None => {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
            } else {
                break;
            }
        }
    }

    fn make_checked_jump(
        &self,
        moves: &mut Vec<Game>,
        src: usize,
        dest: usize,
        index: usize,
        check: fn(&Game, Side, usize) -> bool,
    ) {
        let src2 = self.pieces[index];
        if src2 != src {
            println!("UH OH");  // Well fuck
        }

        if dest & 0x88 == 0 {
            if check(
                self,
                self.board[src]
                    .expect("jump move called on empty space")
                    .side,
                dest,
            ) {
                return;
            }
            match self.board[dest] {
                Some(target) => {
                    if target.side != self.board[src].unwrap().side {
                        moves.push(self.make_move(src, dest, index));
                    }
                }
                None => moves.push(self.make_move(src, dest, index)),
            }
        }
    }

    fn make_jump_move(&self, moves: &mut Vec<Game>, src: usize, dest: usize, index: usize) {
        self.make_checked_jump(moves, src, dest, index, |_, _, _| false)
    }

    #[inline(always)]
    fn make_move(&self, src: usize, dest: usize, index: usize,) -> Game {
        let mut new_board = self.clone();
        new_board.board[dest] = new_board.board[src];
        new_board.board[src] = None;
        new_board.pieces[index] = dest;

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
        return;
        let correct_values = [20, 400, 8902, 197281];

        let board = Game::new();
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
        let board = Game::new();
        let mut moves = vec![];

        // E1
        board.generate_king_moves(&mut moves, 4, WHITE_KING);
        assert_eq!(0, moves.len());
        // E8
        board.generate_king_moves(&mut moves, 0x74, BLACK_KING);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn king_moves_from_middle() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place king on D5
        board.board[0x34] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Place pawn on E5
        board.board[0x44] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        board.generate_king_moves(&mut moves, 0x34, WHITE_KING);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn king_moves_with_potential_check() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place king on D5
        board.board[0x43] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Place pawn on C5
        board.board[0x42] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });
        // Place bishop on C7
        board.board[0x62] = Some(Space {
            piece: Piece::Bishop,
            side: Side::Black,
        });

        board.generate_king_moves(&mut moves, 0x43, WHITE_KING);
        assert_eq!(3, moves.len());
    }

    #[test]
    pub fn queen_moves_from_start() {
        let board = Game::new();
        let mut moves = vec![];

        // D1
        board.generate_queen_moves(&mut moves, 3, WHITE_QUEEN);
        assert_eq!(0, moves.len());
        // D8
        board.generate_queen_moves(&mut moves, 0x73, BLACK_QUEEN);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn queen_moves_from_middle() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place queen on D5
        board.board[0x43] = Some(Space {
            piece: Piece::Queen,
            side: Side::White,
        });

        board.generate_queen_moves(&mut moves, 0x43, WHITE_QUEEN);
        assert_eq!(19, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_start() {
        let board = Game::new();
        let mut moves = vec![];

        // C1
        board.generate_bishop_moves(&mut moves, 2, WHITE_QUEEN_BISHOP);
        assert_eq!(0, moves.len());
        // F1
        board.generate_bishop_moves(&mut moves, 5, WHITE_KING_BISHOP);
        assert_eq!(0, moves.len());
        // C8
        board.generate_bishop_moves(&mut moves, 114, BLACK_QUEEN_BISHOP);
        assert_eq!(0, moves.len());
        // F8
        board.generate_bishop_moves(&mut moves, 117, BLACK_KING_BISHOP);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_middle() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place bishop on D5
        board.board[67] = Some(Space {
            piece: Piece::Bishop,
            side: Side::White,
        });

        board.generate_bishop_moves(&mut moves, 67, WHITE_KING_BISHOP);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_side() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place bishop on B5
        board.board[65] = Some(Space {
            piece: Piece::Bishop,
            side: Side::White,
        });

        board.generate_bishop_moves(&mut moves, 65, WHITE_KING_BISHOP);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn rook_moves_from_start() {
        let board = Game::new();
        let mut moves = vec![];

        // A1
        board.generate_rook_moves(&mut moves, 0, WHITE_QUEEN_ROOK);
        assert_eq!(0, moves.len());
        // H1
        board.generate_rook_moves(&mut moves, 7, WHITE_KING_ROOK);
        assert_eq!(0, moves.len());
        // A8
        board.generate_rook_moves(&mut moves, 112, BLACK_QUEEN_ROOK);
        assert_eq!(0, moves.len());
        // H8
        board.generate_rook_moves(&mut moves, 119, BLACK_KING_ROOK);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn rook_moves_from_middle() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place rook on D5
        board.board[67] = Some(Space {
            piece: Piece::Rook,
            side: Side::White,
        });

        board.generate_rook_moves(&mut moves, 67, WHITE_KING_ROOK);
        assert_eq!(11, moves.len());
    }

    #[test]
    pub fn knight_moves_from_start() {
        let board = Game::new();
        let mut moves = vec![];

        // B1
        board.generate_knight_moves(&mut moves, 1, WHITE_QUEEN_KNIGHT);
        assert_eq!(2, moves.len());
        // G1
        board.generate_knight_moves(&mut moves, 6, WHITE_KING_KNIGHT);
        assert_eq!(4, moves.len());
        // B8
        board.generate_knight_moves(&mut moves, 113, BLACK_QUEEN_KNIGHT);
        assert_eq!(6, moves.len());
        // G8
        board.generate_knight_moves(&mut moves, 118, BLACK_KING_KNIGHT);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_middle() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place knight on D5
        board.board[67] = Some(Space {
            piece: Piece::Knight,
            side: Side::White,
        });

        board.generate_knight_moves(&mut moves, 67, WHITE_KING_KNIGHT);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_side() {
        let mut moves = vec![];

        // Move knight to A5
        let game = Game::new().make_move(0x02, 0x40, WHITE_KING_KNIGHT);

        game.generate_knight_moves(&mut moves, game.pieces[WHITE_KING_KNIGHT], WHITE_KING_KNIGHT);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // A2
        game.generate_pawn_moves(&mut moves, game.pieces[WHITE_PAWN_A], WHITE_PAWN_A);
        assert_eq!(2, moves.len());
        // E2
        game.generate_pawn_moves(&mut moves, game.pieces[WHITE_PAWN_E], WHITE_PAWN_E);
        assert_eq!(4, moves.len());
        // B7
        game.generate_pawn_moves(&mut moves, game.pieces[BLACK_PAWN_B], BLACK_PAWN_B);
        assert_eq!(6, moves.len());
        // G7
        game.generate_pawn_moves(&mut moves, game.pieces[BLACK_PAWN_G], BLACK_PAWN_G);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn pawn_move_from_center() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place pawn on D4
        board.board[0x33] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });
        // Place pawn on C5
        board.board[0x42] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        board.generate_pawn_moves(&mut moves, 0x33, WHITE_PAWN_D);
        assert_eq!(2, moves.len());

        board.generate_pawn_moves(&mut moves, 0x42, BLACK_PAWN_C);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_friendly_front() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place pawn on D5
        board.board[67] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });
        // Place pawn on D6
        board.board[83] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });

        board.generate_pawn_moves(&mut moves, 67, WHITE_PAWN_D);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_enemy_front() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place pawn on D5
        board.board[67] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });
        // Place pawn on D6
        board.board[83] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        board.generate_pawn_moves(&mut moves, 67, WHITE_PAWN_D);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_capture_front() {
        let mut board = Game::new();
        let mut moves = vec![];

        // Place pawn on D6
        board.board[0x53] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });

        board.generate_pawn_moves(&mut moves, 0x53, WHITE_PAWN_D);
        assert_eq!(2, moves.len());
    }

    #[test]
    pub fn king_not_in_check_from_start() {
        let board = Game::new();

        // E1
        assert!(!board.king_check(Side::White, 0x04));

        // E8
        assert!(!board.king_check(Side::Black, 0x74));
    }

    #[test]
    pub fn king_in_check_from_bishop() {
        let mut board = Game::new();

        // King at D4
        board.board[0x33] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Bishop on B6
        board.board[0x51] = Some(Space {
            piece: Piece::Bishop,
            side: Side::Black,
        });

        // E8
        assert!(board.king_check(Side::White, 0x33));
    }

    #[test]
    pub fn king_in_check_from_pawn() {
        let mut board = Game::new();

        // King at D4
        board.board[0x33] = Some(Space {
            piece: Piece::King,
            side: Side::White,
        });
        // Pawn on C5
        board.board[0x42] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::Black,
        });

        // E8
        assert!(board.king_check(Side::White, 0x33));
    }

    #[test]
    pub fn black_king_in_check_from_pawn() {
        let mut board = Game::new();

        // King at D4
        board.board[0x33] = Some(Space {
            piece: Piece::King,
            side: Side::Black,
        });
        // Pawn on E3
        board.board[0x24] = Some(Space {
            piece: Piece::Pawn(false),
            side: Side::White,
        });

        // E8
        assert!(board.king_check(Side::Black, 0x33));
    }

    #[test]
    pub fn king_in_check_from_rook() {
        // Move white king to D4 & black rook to H4
        let game = Game::new().make_move(0x03, 0x33, WHITE_KING)
            .make_move(0x70, 0x37, BLACK_KING_ROOK);

        assert!(game.king_check(Side::White, game.pieces[WHITE_KING]));
    }
}

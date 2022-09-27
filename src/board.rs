use std::{collections::HashMap, mem::{Discriminant, discriminant}};

// Directional movement offsets using 0x88 board representation
// Missing directions are inverts of these (So we subtract)
const UP_LEFT: usize = 15;
const UP: usize = 16;
const UP_RIGHT: usize = 17;
const RIGHT: usize = 1;
const KNIGHT_MOVES: [usize; 4] = [14, 18, 31, 33];

// Captured position value
const CAPTURED: usize = 0xFF;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook(bool), // True if queen rook
    Knight(bool),// True if queen knight
    Bishop(bool),// True if queen bishop
    Pawn(u8, bool), // Store the pawn file & en passent status
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Side {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Space {
    piece: Piece,
    side: Side,
}

#[derive(Clone, Debug)]
pub struct Game {
    board: [Option<Space>; 128], // TODO: Look into bijective map to replace this
    pieces: [usize; 32],
    white: HashMap<Space, usize>,
    black: HashMap<Space, usize>,
    white_check: bool,
    black_check: bool,
}

impl Game {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Game{board: [
            // Rank 1
            Some(Space { piece: Piece::Rook(false), side: Side::White }), Some(Space { piece: Piece::Knight(false), side: Side::White }), Some(Space { piece: Piece::Bishop(false), side: Side::White }),
            Some(Space { piece: Piece::Queen, side: Side::White }), Some(Space { piece: Piece::King, side: Side::White }), Some(Space { piece: Piece::Bishop(true), side: Side::White }),
            Some(Space { piece: Piece::Knight(true), side: Side::White }), Some(Space { piece: Piece::Rook(true), side: Side::White }), None, None, None, None, None, None, None, None,
            // Rank 2
            Some(Space { piece: Piece::Pawn(0, false), side: Side::White }), Some(Space { piece: Piece::Pawn(1, false), side: Side::White }), Some(Space { piece: Piece::Pawn(2, false), side: Side::White }),
            Some(Space { piece: Piece::Pawn(3, false), side: Side::White }), Some(Space { piece: Piece::Pawn(4, false), side: Side::White }), Some(Space { piece: Piece::Pawn(5, false), side: Side::White }),
            Some(Space { piece: Piece::Pawn(6, false), side: Side::White }), Some(Space { piece: Piece::Pawn(7, false), side: Side::White }), None, None, None, None, None, None, None, None,
            // Rank 3 - 6
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            // Rank 7
            Some(Space { piece: Piece::Pawn(0, false), side: Side::Black }), Some(Space { piece: Piece::Pawn(1, false), side: Side::Black }), Some(Space { piece: Piece::Pawn(2, false), side: Side::Black }),
            Some(Space { piece: Piece::Pawn(3, false), side: Side::Black }), Some(Space { piece: Piece::Pawn(4, false), side: Side::Black }), Some(Space { piece: Piece::Pawn(5, false), side: Side::Black }),
            Some(Space { piece: Piece::Pawn(6, false), side: Side::Black }), Some(Space { piece: Piece::Pawn(7, false), side: Side::Black }), None, None, None, None, None, None, None, None,
            // Rank 8 
            Some(Space { piece: Piece::Rook(false), side: Side::Black }), Some(Space { piece: Piece::Knight(false), side: Side::Black }), Some(Space { piece: Piece::Bishop(false), side: Side::Black }),
            Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Bishop(true), side: Side::Black }),
            Some(Space { piece: Piece::Knight(true), side: Side::Black }), Some(Space { piece: Piece::Rook(true), side: Side::Black }), None, None, None, None, None, None, None, None,
        ], white_check: false, black_check: false, 
        pieces: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67],
        white: HashMap::from([(Space { piece: Piece::Rook(false), side: Side::White }, 0x00), (Space { piece: Piece::Knight(false), side: Side::White }, 0x01), (Space { piece: Piece::Bishop(false), side: Side::White }, 0x02),
                              (Space { piece: Piece::Queen, side: Side::White }, 0x03), (Space { piece: Piece::King, side: Side::White }, 0x04), (Space { piece: Piece::Bishop(true), side: Side::White }, 0x05),
                              (Space { piece: Piece::Knight(true), side: Side::White }, 0x06), (Space { piece: Piece::Rook(true), side: Side::White }, 0x07), (Space { piece: Piece::Pawn(0, false), side: Side::White }, 0x10),
                              (Space { piece: Piece::Pawn(1, false), side: Side::White }, 0x11), (Space { piece: Piece::Pawn(2, false), side: Side::White }, 0x12), (Space { piece: Piece::Pawn(3, false), side: Side::White }, 0x13),
                              (Space { piece: Piece::Pawn(4, false), side: Side::White }, 0x14), (Space { piece: Piece::Pawn(5, false), side: Side::White }, 0x15), (Space { piece: Piece::Pawn(6, false), side: Side::White }, 0x16),
                              (Space { piece: Piece::Pawn(7, false), side: Side::White }, 0x17)]),
        black: HashMap::from([(Space { piece: Piece::Rook(false), side: Side::Black }, 0x70), (Space { piece: Piece::Knight(false), side: Side::Black }, 0x71), (Space { piece: Piece::Bishop(false), side: Side::Black }, 0x72),
                              (Space { piece: Piece::Queen, side: Side::Black }, 0x73), (Space { piece: Piece::King, side: Side::Black }, 0x74), (Space { piece: Piece::Bishop(true), side: Side::Black }, 0x75),
                              (Space { piece: Piece::Knight(true), side: Side::Black }, 0x76), (Space { piece: Piece::Rook(true), side: Side::Black }, 0x77), (Space { piece: Piece::Pawn(0, false), side: Side::Black }, 0x60),
                              (Space { piece: Piece::Pawn(1, false), side: Side::Black }, 0x61), (Space { piece: Piece::Pawn(2, false), side: Side::Black }, 0x62), (Space { piece: Piece::Pawn(3, false), side: Side::Black }, 0x63),
                              (Space { piece: Piece::Pawn(4, false), side: Side::Black }, 0x64), (Space { piece: Piece::Pawn(5, false), side: Side::Black }, 0x65), (Space { piece: Piece::Pawn(6, false), side: Side::Black }, 0x66),
                              (Space { piece: Piece::Pawn(7, false), side: Side::Black }, 0x67)]),
    }
    }

    pub fn generate_ply(&self, player: Side) -> Vec<Game> {
        let mut moves = vec![];

        let range = if player == Side::White { 0..16 } else { 16..32 };
        let side = if player == Side::White {&self.white} else {&self.black};

        for (space, position) in side {
            match space.piece {
                Piece::King => self.generate_king_moves(&mut moves, *position, 0),
                Piece::Queen => self.generate_queen_moves(&mut moves, *position, 0),
                Piece::Rook(_) => self.generate_rook_moves(&mut moves, *position, 0),
                Piece::Knight(_) => self.generate_knight_moves(&mut moves, *position, 0),
                Piece::Bishop(_) => self.generate_bishop_moves(&mut moves, *position, 0),
                Piece::Pawn(_, _) => self.generate_pawn_moves(&mut moves, *position, 0),
            }
        }

        // for (i, position) in self.pieces[range].iter().enumerate() {
        //     match self.board[*position]
        //         .expect("piece index set to empty space")
        //         .piece
        //     {
        //         Piece::King => self.generate_king_moves(&mut moves, *position, i),
        //         Piece::Queen => self.generate_queen_moves(&mut moves, *position, i),
        //         Piece::Rook(_) => self.generate_rook_moves(&mut moves, *position, i),
        //         Piece::Knight(_) => self.generate_knight_moves(&mut moves, *position, i),
        //         Piece::Bishop(_) => self.generate_bishop_moves(&mut moves, *position, i),
        //         Piece::Pawn(_, _) => self.generate_pawn_moves(&mut moves, *position, i),
        //     }
        // }

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
            self.king_check_inner_jump(side, position, discriminant(&Piece::Knight(false)), val, offset)
        }) ||
        // // Check rook attacks
        [UP, RIGHT].iter().fold(false, |val, offset| {
            self.king_check_inner(side, position, discriminant(&Piece::Rook(false)), val, offset)
        }) ||
        // Check bishop attacks
        [UP_LEFT, UP_RIGHT].iter().fold(false, |val, offset| {
            self.king_check_inner(side, position, discriminant(&Piece::Bishop(false)), val, offset)
        }) ||
        // Check queen attacks
        [UP, RIGHT, UP_LEFT, UP_RIGHT]
            .iter()
            .fold(false, |val, offset| {
                self.king_check_inner(side, position, discriminant(&Piece::Queen), val, offset)
            }) ||
        // Check pawn attacks
        [UP_LEFT, UP_RIGHT].iter().fold(false, |val, offset| {
            // Detect black attacking pawns - which from above
            if side == Side::White {
                if let Some(Space {
                    piece: Piece::Pawn(_, _),
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
                        piece: Piece::Pawn(_, _),
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
        attack_piece: Discriminant<Piece>,
        val: bool,
        offset: &usize,
    ) -> bool {
        if position + offset * 0x88 == 0 {
            if let Some(space) = self.board[position + offset] {
                if space.side != side && discriminant(&space.piece) == attack_piece {
                    return true;
                } else {
                    return false || val;
                }
            }
        }
        if let Some(attack) = position.checked_sub(*offset) {
            if attack & 0x88 == 0 {
                if let Some(space) = self.board[attack] {
                    if space.side != side && discriminant(&space.piece) == attack_piece {
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
        attack_piece: Discriminant<Piece>,
        val: bool,
        offset: &usize,
    ) -> bool {
        let mut attack = position;
        loop {
            attack += offset;
            if attack & 0x88 == 0 {
                if let Some(space) = self.board[attack] {
                    if space.side != side && discriminant(&space.piece) == attack_piece {
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
                    if space.side != side && discriminant(&space.piece) == attack_piece {
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

    fn make_sliding_moves(
        &self,
        moves: &mut Vec<Game>,
        src: usize,
        index: usize,
        src_exp: fn(usize) -> usize,
    ) {
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
            println!("UH OH"); // Well fuck
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
    fn make_move(&self, src: usize, dest: usize, index: usize) -> Game {
        let mut new_board = self.clone();
        new_board.pieces[index] = dest;
        // if let Some(space) = new_board.pieces[dest] {
        //     new_board.
        // }
        
        new_board.board[dest] = new_board.board[src];
        new_board.board[src] = None;

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
    pub fn make_move_sets_board_and_pieces() {
        let game = Game::new();
        let queen = game.board[0x73];

        // Move black queen to G6
        let game = game.make_move(0x73, 0x55, BLACK_QUEEN);

        assert_eq!(0x55, game.pieces[BLACK_QUEEN]);
        assert_eq!(None, game.board[0x73]);
        assert_eq!(queen, game.board[0x55]);
    }

    #[test]
    pub fn make_move_with_capture_clears_captured_piece() {
        let game = Game::new();
        let queen = game.board[0x73];

        // Move black queen to D2
        let game = game.make_move(0x73, 0x13, BLACK_QUEEN);

        assert_eq!(0x13, game.pieces[BLACK_QUEEN]);
        assert_eq!(None, game.board[0x73]);
        assert_eq!(queen, game.board[0x13]);
        assert_eq!(CAPTURED, game.pieces[WHITE_PAWN_D]);
    }

    #[test]
    pub fn king_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // E1
        game.generate_king_moves(&mut moves, game.pieces[WHITE_KING], WHITE_KING);
        assert_eq!(0, moves.len());
        // E8
        game.generate_king_moves(&mut moves, game.pieces[BLACK_KING], BLACK_KING);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn king_moves_from_middle() {
        let mut moves = vec![];

        // Move white king to E4 & black pawn to E5
        let game =
            Game::new()
                .make_move(0x04, 0x34, WHITE_KING)
                .make_move(0x64, 0x44, BLACK_PAWN_E);

        game.generate_king_moves(&mut moves, game.pieces[WHITE_KING], WHITE_KING);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn king_moves_with_potential_check() {
        let mut moves = vec![];

        // Move white king to D5, black pawn to C5 & black bishop to C7
        let game = Game::new()
            .make_move(0x04, 0x43, WHITE_KING)
            .make_move(0x62, 0x42, BLACK_PAWN_C)
            .make_move(0x75, 0x62, BLACK_KING_BISHOP);

        game.generate_king_moves(&mut moves, game.pieces[WHITE_KING], WHITE_KING);
        assert_eq!(3, moves.len());
    }

    #[test]
    pub fn queen_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // D1
        game.generate_queen_moves(&mut moves, game.pieces[WHITE_QUEEN], WHITE_QUEEN);
        assert_eq!(0, moves.len());
        // D8
        game.generate_queen_moves(&mut moves, game.pieces[BLACK_QUEEN], BLACK_QUEEN);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn queen_moves_from_middle() {
        let mut moves = vec![];

        // Move white queen to D5
        let game = Game::new().make_move(0x03, 0x43, WHITE_QUEEN);

        game.generate_queen_moves(&mut moves, game.pieces[WHITE_QUEEN], WHITE_QUEEN);
        assert_eq!(19, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // C1
        game.generate_bishop_moves(
            &mut moves,
            game.pieces[WHITE_QUEEN_BISHOP],
            WHITE_QUEEN_BISHOP,
        );
        assert_eq!(0, moves.len());
        // F1
        game.generate_bishop_moves(
            &mut moves,
            game.pieces[WHITE_KING_BISHOP],
            WHITE_KING_BISHOP,
        );
        assert_eq!(0, moves.len());
        // C8
        game.generate_bishop_moves(
            &mut moves,
            game.pieces[BLACK_QUEEN_BISHOP],
            BLACK_QUEEN_BISHOP,
        );
        assert_eq!(0, moves.len());
        // F8
        game.generate_bishop_moves(
            &mut moves,
            game.pieces[BLACK_KING_BISHOP],
            BLACK_KING_BISHOP,
        );
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_middle() {
        let mut moves = vec![];

        // Move white bishop to D5
        let game = Game::new().make_move(0x05, 0x43, WHITE_KING_BISHOP);

        game.generate_bishop_moves(
            &mut moves,
            game.pieces[WHITE_KING_BISHOP],
            WHITE_KING_BISHOP,
        );
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_side() {
        let mut moves = vec![];

        // Move white bishop to B5
        let game = Game::new().make_move(0x05, 0x41, WHITE_KING_BISHOP);

        game.generate_bishop_moves(
            &mut moves,
            game.pieces[WHITE_KING_BISHOP],
            WHITE_KING_BISHOP,
        );
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn rook_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // A1
        game.generate_rook_moves(&mut moves, game.pieces[WHITE_QUEEN_ROOK], WHITE_QUEEN_ROOK);
        assert_eq!(0, moves.len());
        // H1
        game.generate_rook_moves(&mut moves, game.pieces[WHITE_KING_ROOK], WHITE_KING_ROOK);
        assert_eq!(0, moves.len());
        // A8
        game.generate_rook_moves(&mut moves, game.pieces[BLACK_QUEEN_ROOK], BLACK_QUEEN_ROOK);
        assert_eq!(0, moves.len());
        // H8
        game.generate_rook_moves(&mut moves, game.pieces[BLACK_KING_ROOK], BLACK_KING_ROOK);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn rook_moves_from_middle() {
        let mut moves = vec![];

        // Move white rook to D5
        let game = Game::new().make_move(0x00, 0x43, WHITE_QUEEN_ROOK);

        game.generate_rook_moves(&mut moves, game.pieces[WHITE_QUEEN_ROOK], WHITE_QUEEN_ROOK);
        assert_eq!(11, moves.len());
    }

    #[test]
    pub fn knight_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // B1
        game.generate_knight_moves(
            &mut moves,
            game.pieces[WHITE_QUEEN_KNIGHT],
            WHITE_QUEEN_KNIGHT,
        );
        assert_eq!(2, moves.len());
        // G1
        game.generate_knight_moves(
            &mut moves,
            game.pieces[WHITE_KING_KNIGHT],
            WHITE_KING_KNIGHT,
        );
        assert_eq!(4, moves.len());
        // B8
        game.generate_knight_moves(
            &mut moves,
            game.pieces[BLACK_QUEEN_KNIGHT],
            BLACK_QUEEN_KNIGHT,
        );
        assert_eq!(6, moves.len());
        // G8
        game.generate_knight_moves(
            &mut moves,
            game.pieces[BLACK_KING_KNIGHT],
            BLACK_KING_KNIGHT,
        );
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_middle() {
        let mut moves = vec![];

        // Move white knight to D5
        let game = Game::new().make_move(0x02, 0x43, WHITE_QUEEN_KNIGHT);

        game.generate_knight_moves(
            &mut moves,
            game.pieces[WHITE_QUEEN_KNIGHT],
            WHITE_QUEEN_KNIGHT,
        );
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_side() {
        let mut moves = vec![];

        // Move white knight to A5
        let game = Game::new().make_move(0x02, 0x40, WHITE_QUEEN_KNIGHT);

        game.generate_knight_moves(
            &mut moves,
            game.pieces[WHITE_QUEEN_KNIGHT],
            WHITE_QUEEN_KNIGHT,
        );
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
        let mut moves = vec![];

        // Move white pawn to D4 & black pawn to C5
        let game =
            Game::new()
                .make_move(0x13, 0x33, WHITE_PAWN_D)
                .make_move(0x62, 0x42, BLACK_PAWN_C);

        game.generate_pawn_moves(&mut moves, game.pieces[WHITE_PAWN_D], WHITE_PAWN_D);
        assert_eq!(2, moves.len());

        game.generate_pawn_moves(&mut moves, game.pieces[BLACK_PAWN_C], BLACK_PAWN_C);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_friendly_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & white pawn to D6
        let game =
            Game::new()
                .make_move(0x13, 0x43, WHITE_PAWN_D)
                .make_move(0x14, 0x53, WHITE_PAWN_E);

        game.generate_pawn_moves(&mut moves, game.pieces[WHITE_PAWN_D], WHITE_PAWN_D);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_enemy_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & black pawn to D6
        let game =
            Game::new()
                .make_move(0x13, 0x43, WHITE_PAWN_D)
                .make_move(0x63, 0x53, BLACK_PAWN_D);

        game.generate_pawn_moves(&mut moves, game.pieces[WHITE_PAWN_D], WHITE_PAWN_D);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_capture_front() {
        let mut moves = vec![];

        // Move white pawn to D6
        let game = Game::new().make_move(0x13, 0x53, WHITE_PAWN_D);

        game.generate_pawn_moves(&mut moves, game.pieces[WHITE_PAWN_D], WHITE_PAWN_D);
        assert_eq!(2, moves.len());
    }

    #[test]
    pub fn king_not_in_check_from_start() {
        let game = Game::new();

        // E1
        assert!(!game.king_check(Side::White, game.pieces[WHITE_KING]));

        // E8
        assert!(!game.king_check(Side::Black, game.pieces[BLACK_KING]));
    }

    #[test]
    pub fn king_in_check_from_bishop() {
        // Move white king to D4 & black bishop to B6
        let game =
            Game::new()
                .make_move(0x04, 0x33, WHITE_KING)
                .make_move(0x75, 0x51, BLACK_KING_BISHOP);

        assert!(game.king_check(Side::White, game.pieces[WHITE_KING]));
    }

    #[test]
    pub fn king_in_check_from_pawn() {
        // Move white king to D4 & black pawn to C5
        let game =
            Game::new()
                .make_move(0x04, 0x33, WHITE_KING)
                .make_move(0x67, 0x42, BLACK_PAWN_H);

        assert!(game.king_check(Side::White, game.pieces[WHITE_KING]));
    }

    #[test]
    pub fn black_king_in_check_from_pawn() {
        // Move black king to D4 & white pawn to E3
        let game =
            Game::new()
                .make_move(0x74, 0x33, BLACK_KING)
                .make_move(0x16, 0x24, WHITE_PAWN_G);

        assert!(game.king_check(Side::Black, game.pieces[BLACK_KING]));
    }

    #[test]
    pub fn king_in_check_from_rook() {
        // Move white king to D4 & black rook to H4
        let game =
            Game::new()
                .make_move(0x04, 0x33, WHITE_KING)
                .make_move(0x70, 0x37, BLACK_QUEEN_ROOK);

        assert!(game.king_check(Side::White, game.pieces[WHITE_KING]));
    }
}

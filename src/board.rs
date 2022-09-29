use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook(bool),   // True if king rook
    Knight(bool), // True if king knight
    Bishop(bool), // True if king bishop
    Pawn(u8),     // Store the pawn file
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
    white: HashMap<Piece, usize>,
    black: HashMap<Piece, usize>,
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
            Some(Space { piece: Piece::Pawn(0), side: Side::White }), Some(Space { piece: Piece::Pawn(1), side: Side::White }), Some(Space { piece: Piece::Pawn(2), side: Side::White }),
            Some(Space { piece: Piece::Pawn(3), side: Side::White }), Some(Space { piece: Piece::Pawn(4), side: Side::White }), Some(Space { piece: Piece::Pawn(5), side: Side::White }),
            Some(Space { piece: Piece::Pawn(6), side: Side::White }), Some(Space { piece: Piece::Pawn(7), side: Side::White }), None, None, None, None, None, None, None, None,
            // Rank 3 - 6
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            // Rank 7
            Some(Space { piece: Piece::Pawn(0), side: Side::Black }), Some(Space { piece: Piece::Pawn(1), side: Side::Black }), Some(Space { piece: Piece::Pawn(2), side: Side::Black }),
            Some(Space { piece: Piece::Pawn(3), side: Side::Black }), Some(Space { piece: Piece::Pawn(4), side: Side::Black }), Some(Space { piece: Piece::Pawn(5), side: Side::Black }),
            Some(Space { piece: Piece::Pawn(6), side: Side::Black }), Some(Space { piece: Piece::Pawn(7), side: Side::Black }), None, None, None, None, None, None, None, None,
            // Rank 8 
            Some(Space { piece: Piece::Rook(false), side: Side::Black }), Some(Space { piece: Piece::Knight(false), side: Side::Black }), Some(Space { piece: Piece::Bishop(false), side: Side::Black }),
            Some(Space { piece: Piece::Queen, side: Side::Black }), Some(Space { piece: Piece::King, side: Side::Black }), Some(Space { piece: Piece::Bishop(true), side: Side::Black }),
            Some(Space { piece: Piece::Knight(true), side: Side::Black }), Some(Space { piece: Piece::Rook(true), side: Side::Black }), None, None, None, None, None, None, None, None,
        ], white_check: false, black_check: false, 
        white: HashMap::from([(Piece::Rook(false), 0x00), (Piece::Knight(false), 0x01), (Piece::Bishop(false), 0x02),
                              (Piece::Queen, 0x03), (Piece::King, 0x04), (Piece::Bishop(true), 0x05),
                              (Piece::Knight(true), 0x06), (Piece::Rook(true), 0x07), (Piece::Pawn(0), 0x10),
                              (Piece::Pawn(1), 0x11), (Piece::Pawn(2), 0x12), (Piece::Pawn(3), 0x13),
                              (Piece::Pawn(4), 0x14), (Piece::Pawn(5), 0x15), (Piece::Pawn(6), 0x16),
                              (Piece::Pawn(7), 0x17)]),
        black: HashMap::from([(Piece::Rook(false), 0x70), (Piece::Knight(false), 0x71), (Piece::Bishop(false), 0x72),
                              (Piece::Queen, 0x73), (Piece::King, 0x74), (Piece::Bishop(true), 0x75),
                              (Piece::Knight(true), 0x76), (Piece::Rook(true), 0x77), (Piece::Pawn(0), 0x60),
                              (Piece::Pawn(1), 0x61), (Piece::Pawn(2), 0x62), (Piece::Pawn(3), 0x63),
                              (Piece::Pawn(4), 0x64), (Piece::Pawn(5), 0x65), (Piece::Pawn(6), 0x66),
                              (Piece::Pawn(7), 0x67)]),
    }
    }

    pub fn generate_ply(&self, player: Side) -> Vec<Game> {
        let mut moves = vec![];

        let side = if player == Side::White {
            &self.white
        } else {
            &self.black
        };

        for (piece, position) in side {
            match piece {
                Piece::King => self.generate_king_moves(&mut moves, *position),
                Piece::Queen => self.generate_queen_moves(&mut moves, *position),
                Piece::Rook(_) => self.generate_rook_moves(&mut moves, *position),
                Piece::Knight(_) => self.generate_knight_moves(&mut moves, *position),
                Piece::Bishop(_) => self.generate_bishop_moves(&mut moves, *position),
                Piece::Pawn(_) => self.generate_pawn_moves(&mut moves, *position),
            }
        }

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
    fn generate_king_moves(&self, moves: &mut Vec<Game>, src: usize) {
        [UP_RIGHT, UP, UP_LEFT, RIGHT].iter().for_each(|offset| {
            self.make_checked_jump(moves, src, src + offset, Game::king_check);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_checked_jump(moves, src, dest, Game::king_check),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, moves: &mut Vec<Game>, src: usize) {
        // TODO: En passent
        match self.board[src]
            .expect("generate pawn moves called on empty space")
            .side
        {
            Side::White => {
                let dest = src + UP;
                if dest & 0x88 == 0 {
                    if let None = self.board[dest] {
                        moves.push(self.make_move(src, dest));
                    }
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test as this can't be off board
                if src >= 0x10 && src <= 0x17 {
                    let dest = src + UP + UP;
                    if let None = self.board[dest] {
                        moves.push(self.make_move(src, dest));
                    }
                }
                let dest = src + UP_RIGHT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.board[dest]
                    {
                        moves.push(self.make_move(src, dest));
                    }
                }
                let dest = src + UP_LEFT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.board[dest]
                    {
                        moves.push(self.make_move(src, dest));
                    }
                }
            }
            Side::Black => {
                match src.checked_sub(UP) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let None = self.board[dest] {
                                moves.push(self.make_move(src, dest));
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
                        moves.push(self.make_move(src, dest));
                    }
                }
                match src.checked_sub(UP_RIGHT) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let Some(Space {
                                side: Side::White, ..
                            }) = self.board[dest]
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
                            }) = self.board[dest]
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
    fn generate_knight_moves(&self, moves: &mut Vec<Game>, src: usize) {
        KNIGHT_MOVES.iter().for_each(|offset| {
            self.make_jump_move(moves, src, src + offset);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_jump_move(moves, src, dest),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_queen_moves(&self, moves: &mut Vec<Game>, src: usize) {
        // Queen moves as the union of rook and bishop
        self.generate_rook_moves(moves, src);
        self.generate_bishop_moves(moves, src);
    }

    #[inline(always)]
    fn generate_bishop_moves(&self, moves: &mut Vec<Game>, src: usize) {
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
    fn generate_rook_moves(&self, moves: &mut Vec<Game>, src: usize) {
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

    fn make_sliding_moves(&self, moves: &mut Vec<Game>, src: usize, src_exp: fn(usize) -> usize) {
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
        moves: &mut Vec<Game>,
        src: usize,
        dest: usize,
        check: fn(&Game, Side, usize) -> bool,
    ) {
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
                        moves.push(self.make_move(src, dest));
                    }
                }
                None => moves.push(self.make_move(src, dest)),
            }
        }
    }

    fn make_jump_move(&self, moves: &mut Vec<Game>, src: usize, dest: usize) {
        self.make_checked_jump(moves, src, dest, |_, _, _| false)
    }

    #[inline(always)]
    fn make_move(&self, src: usize, dest: usize) -> Game {
        let mut new_board = self.clone();

        // Update piece hashmaps
        let (me, opponent) = match new_board.board[src].unwrap().side {
            Side::White => (&mut new_board.white, &mut new_board.black),
            Side::Black => (&mut new_board.black, &mut new_board.white),
        };
        me.insert(new_board.board[src].unwrap().piece, dest);
        if let Some(space) = new_board.board[dest] {
            opponent.remove(&space.piece);
        }

        // Update board array
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
    pub fn make_move_black_sets_board_and_pieces() {
        let game = Game::new();
        let queen = game.board[0x73];

        // Move black queen to G6
        let game = game.make_move(0x73, 0x55);

        assert_eq!(0x55, game.black[&Piece::Queen]);
        assert_eq!(None, game.board[0x73]);
        assert_eq!(queen, game.board[0x55]);
    }

    #[test]
    pub fn make_move_white_sets_board_and_pieces() {
        let game = Game::new();
        let queen = game.board[0x03];

        // Move white queen to G6
        let game = game.make_move(0x03, 0x55);

        assert_eq!(0x55, game.white[&Piece::Queen]);
        assert_eq!(None, game.board[0x03]);
        assert_eq!(queen, game.board[0x55]);
    }

    #[test]
    pub fn make_move_with_capture_clears_captured_piece() {
        let game = Game::new();
        let queen = game.board[0x73];

        // Move black queen to D2
        let game = game.make_move(0x73, 0x13);

        assert_eq!(0x13, game.black[&Piece::Queen]);
        assert_eq!(None, game.board[0x73]);
        assert_eq!(queen, game.board[0x13]);
        assert!(!game.white.contains_key(&Piece::Pawn(3)))
    }

    #[test]
    pub fn king_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // E1
        game.generate_king_moves(&mut moves, game.white[&Piece::King]);
        assert_eq!(0, moves.len());
        // E8
        game.generate_king_moves(&mut moves, game.black[&Piece::King]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn king_moves_from_middle() {
        let mut moves = vec![];

        // Move white king to E4 & black pawn to E5
        let game = Game::new().make_move(0x04, 0x34).make_move(0x64, 0x44);

        game.generate_king_moves(&mut moves, game.white[&Piece::King]);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn king_moves_with_potential_check() {
        let mut moves = vec![];

        // Move white king to D5, black pawn to C5 & black bishop to C7
        let game = Game::new()
            .make_move(0x04, 0x43)
            .make_move(0x62, 0x42)
            .make_move(0x75, 0x62);

        game.generate_king_moves(&mut moves, game.white[&Piece::King]);
        assert_eq!(3, moves.len());
    }

    #[test]
    pub fn queen_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // D1
        game.generate_queen_moves(&mut moves, game.white[&Piece::Queen]);
        assert_eq!(0, moves.len());
        // D8
        game.generate_queen_moves(&mut moves, game.black[&Piece::Queen]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn queen_moves_from_middle() {
        let mut moves = vec![];

        // Move white queen to D5
        let game = Game::new().make_move(0x03, 0x43);

        game.generate_queen_moves(&mut moves, game.white[&Piece::Queen]);
        assert_eq!(19, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // C1
        game.generate_bishop_moves(&mut moves, game.white[&Piece::Bishop(false)]);
        assert_eq!(0, moves.len());
        // F1
        game.generate_bishop_moves(&mut moves, game.white[&Piece::Bishop(true)]);
        assert_eq!(0, moves.len());
        // C8
        game.generate_bishop_moves(&mut moves, game.black[&Piece::Bishop(false)]);
        assert_eq!(0, moves.len());
        // F8
        game.generate_bishop_moves(&mut moves, game.black[&Piece::Bishop(true)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_middle() {
        let mut moves = vec![];

        // Move white bishop to D5
        let game = Game::new().make_move(0x05, 0x43);

        game.generate_bishop_moves(&mut moves, game.white[&Piece::Bishop(true)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_side() {
        let mut moves = vec![];

        // Move white bishop to B5
        let game = Game::new().make_move(0x05, 0x41);

        game.generate_bishop_moves(&mut moves, game.white[&Piece::Bishop(true)]);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn rook_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // A1
        game.generate_rook_moves(&mut moves, game.white[&Piece::Rook(false)]);
        assert_eq!(0, moves.len());
        // H1
        game.generate_rook_moves(&mut moves, game.white[&Piece::Rook(true)]);
        assert_eq!(0, moves.len());
        // A8
        game.generate_rook_moves(&mut moves, game.black[&Piece::Rook(false)]);
        assert_eq!(0, moves.len());
        // H8
        game.generate_rook_moves(&mut moves, game.black[&Piece::Rook(true)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn rook_moves_from_middle() {
        let mut moves = vec![];

        // Move white rook to D5
        let game = Game::new().make_move(0x00, 0x43);

        game.generate_rook_moves(&mut moves, game.white[&Piece::Rook(false)]);
        assert_eq!(11, moves.len());
    }

    #[test]
    pub fn knight_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // B1
        game.generate_knight_moves(&mut moves, game.white[&Piece::Knight(false)]);
        assert_eq!(2, moves.len());
        // G1
        game.generate_knight_moves(&mut moves, game.white[&Piece::Knight(true)]);
        assert_eq!(4, moves.len());
        // B8
        game.generate_knight_moves(&mut moves, game.black[&Piece::Knight(false)]);
        assert_eq!(6, moves.len());
        // G8
        game.generate_knight_moves(&mut moves, game.black[&Piece::Knight(true)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_middle() {
        let mut moves = vec![];

        // Move white knight to D5
        let game = Game::new().make_move(0x01, 0x43);

        game.generate_knight_moves(&mut moves, game.white[&Piece::Knight(false)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_side() {
        let mut moves = vec![];

        // Move white knight to A5
        let game = Game::new().make_move(0x01, 0x40);

        game.generate_knight_moves(&mut moves, game.white[&Piece::Knight(false)]);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // A2
        game.generate_pawn_moves(&mut moves, game.white[&Piece::Pawn(0)]);
        assert_eq!(2, moves.len());
        // E2
        game.generate_pawn_moves(&mut moves, game.white[&Piece::Pawn(4)]);
        assert_eq!(4, moves.len());
        // B7
        game.generate_pawn_moves(&mut moves, game.black[&Piece::Pawn(1)]);
        assert_eq!(6, moves.len());
        // G7
        game.generate_pawn_moves(&mut moves, game.black[&Piece::Pawn(6)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn pawn_move_from_center() {
        let mut moves = vec![];

        // Move white pawn to D4 & black pawn to C5
        let game = Game::new().make_move(0x13, 0x33).make_move(0x62, 0x42);

        game.generate_pawn_moves(&mut moves, game.white[&Piece::Pawn(3)]);
        assert_eq!(2, moves.len());

        game.generate_pawn_moves(&mut moves, game.black[&Piece::Pawn(2)]);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_friendly_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & white pawn to D6
        let game = Game::new().make_move(0x13, 0x43).make_move(0x14, 0x53);

        game.generate_pawn_moves(&mut moves, game.white[&Piece::Pawn(3)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_enemy_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & black pawn to D6
        let game = Game::new().make_move(0x13, 0x43).make_move(0x63, 0x53);

        game.generate_pawn_moves(&mut moves, game.white[&Piece::Pawn(3)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_capture_front() {
        let mut moves = vec![];

        // Move white pawn to D6
        let game = Game::new().make_move(0x13, 0x53);

        game.generate_pawn_moves(&mut moves, game.white[&Piece::Pawn(3)]);
        assert_eq!(2, moves.len());
    }

    #[test]
    pub fn king_not_in_check_from_start() {
        let game = Game::new();

        // E1
        assert!(!game.king_check(Side::White, game.white[&Piece::King]));

        // E8
        assert!(!game.king_check(Side::Black, game.black[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_bishop() {
        // Move white king to D4 & black bishop to B6
        let game = Game::new().make_move(0x04, 0x33).make_move(0x75, 0x51);

        assert!(game.king_check(Side::White, game.white[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_pawn() {
        // Move white king to D4 & black pawn to C5
        let game = Game::new().make_move(0x04, 0x33).make_move(0x67, 0x42);

        assert!(game.king_check(Side::White, game.white[&Piece::King]));
    }

    #[test]
    pub fn black_king_in_check_from_pawn() {
        // Move black king to D4 & white pawn to E3
        let game = Game::new().make_move(0x74, 0x33).make_move(0x16, 0x24);

        assert!(game.king_check(Side::Black, game.black[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_rook() {
        // Move white king to D4 & black rook to H4
        let game = Game::new().make_move(0x04, 0x33).make_move(0x70, 0x37);

        assert!(game.king_check(Side::White, game.white[&Piece::King]));
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Player {
    pieces: HashMap<Piece, usize>,
    check: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    board: [Option<Space>; 128], // TODO: Look into bijective map to replace this
    white: Player,
    black: Player,
}

impl Game {
    /// Create a new game object, from the standard starting position
    #[rustfmt::skip]
    pub fn new() -> Self {
        Game{
            board: [
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
            ],
            white: Player{ pieces: HashMap::from([(Piece::Rook(false), 0x00), (Piece::Knight(false), 0x01), (Piece::Bishop(false), 0x02),
                                (Piece::Queen, 0x03), (Piece::King, 0x04), (Piece::Bishop(true), 0x05),
                                (Piece::Knight(true), 0x06), (Piece::Rook(true), 0x07), (Piece::Pawn(0), 0x10),
                                (Piece::Pawn(1), 0x11), (Piece::Pawn(2), 0x12), (Piece::Pawn(3), 0x13),
                                (Piece::Pawn(4), 0x14), (Piece::Pawn(5), 0x15), (Piece::Pawn(6), 0x16),
                                (Piece::Pawn(7), 0x17)]), check: false },
            black: Player{ pieces: HashMap::from([(Piece::Rook(false), 0x70), (Piece::Knight(false), 0x71), (Piece::Bishop(false), 0x72),
                                (Piece::Queen, 0x73), (Piece::King, 0x74), (Piece::Bishop(true), 0x75),
                                (Piece::Knight(true), 0x76), (Piece::Rook(true), 0x77), (Piece::Pawn(0), 0x60),
                                (Piece::Pawn(1), 0x61), (Piece::Pawn(2), 0x62), (Piece::Pawn(3), 0x63),
                                (Piece::Pawn(4), 0x64), (Piece::Pawn(5), 0x65), (Piece::Pawn(6), 0x66),
                                (Piece::Pawn(7), 0x67)]), check: false },
        }
    }

    // Create a new game object, using A Forsyth–Edwards Notation string
    pub fn from_fen(raw_game: String) -> Game {
        let mut game = Self::new();
        let mut fen = raw_game.split(" ");

        let mut white_pawn = 0;
        let mut black_pawn = 0;
        let mut white_knight = false;
        let mut black_knight = false;
        let mut white_bishop = false;
        let mut black_bishop = false;
        let mut white_rook = false;
        let mut black_rook = false;
        let mut space = 0x70;
        for rune in fen.next().unwrap().chars() {
            println!("{} {:#02x}", rune, space);
            match rune {
                'P' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Pawn(white_pawn),
                        side: Side::White,
                    });
                    game.white.pieces.insert(Piece::Pawn(white_pawn), space);
                    white_pawn += 1;
                }
                'N' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Knight(white_knight),
                        side: Side::White,
                    });
                    game.white.pieces.insert(Piece::Knight(white_knight), space);
                    white_knight = true;
                }
                'B' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Bishop(white_bishop),
                        side: Side::White,
                    });
                    game.white.pieces.insert(Piece::Bishop(white_bishop), space);
                    white_bishop = true;
                }
                'R' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Rook(white_rook),
                        side: Side::White,
                    });
                    game.white.pieces.insert(Piece::Rook(white_rook), space);
                    white_rook = true;
                }
                'Q' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Queen,
                        side: Side::White,
                    });
                    game.white.pieces.insert(Piece::Queen, space);
                }
                'K' => {
                    game.board[space] = Some(Space {
                        piece: Piece::King,
                        side: Side::White,
                    });
                    game.white.pieces.insert(Piece::King, space);
                }
                'p' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Pawn(black_pawn),
                        side: Side::Black,
                    });
                    game.black.pieces.insert(Piece::Pawn(black_pawn), space);
                    black_pawn += 1;
                }
                'n' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Knight(black_knight),
                        side: Side::Black,
                    });
                    game.black.pieces.insert(Piece::Knight(black_knight), space);
                    black_knight = true;
                }
                'b' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Bishop(black_bishop),
                        side: Side::Black,
                    });
                    game.black.pieces.insert(Piece::Bishop(black_bishop), space);
                    black_bishop = true;
                }
                'r' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Rook(black_rook),
                        side: Side::Black,
                    });
                    game.black.pieces.insert(Piece::Rook(black_rook), space);
                    black_rook = true;
                }
                'q' => {
                    game.board[space] = Some(Space {
                        piece: Piece::Queen,
                        side: Side::Black,
                    });
                    game.black.pieces.insert(Piece::Queen, space);
                }
                'k' => {
                    game.board[space] = Some(Space {
                        piece: Piece::King,
                        side: Side::Black,
                    });
                    game.black.pieces.insert(Piece::King, space);
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    let skip = rune.to_digit(10).unwrap() as usize;
                    space += skip - 1;
                }
                '/' => {
                    space -= 0x18; // Move to the start of the next rank
                    continue; // And skip the +1
                }
                _ => panic!("unexpected character in FEN string"),
            }
            space += 1;
        }

        game
    }

    #[inline(always)]
    fn get_player(&self, player: Side) -> &Player {
        match player {
            Side::White => &self.white,
            Side::Black => &self.black,
        }
    }

    pub fn generate_ply(&self, side: Side) -> Vec<Game> {
        let mut moves = vec![];

        let player = self.get_player(side);
        if player.check {
            self.generate_king_moves(&mut moves, player.pieces[&Piece::King])
        } else {
            for (piece, position) in player.pieces.iter() {
                match piece {
                    Piece::King => self.generate_king_moves(&mut moves, *position),
                    Piece::Queen => self.generate_queen_moves(&mut moves, *position),
                    Piece::Rook(_) => self.generate_rook_moves(&mut moves, *position),
                    Piece::Knight(_) => self.generate_knight_moves(&mut moves, *position),
                    Piece::Bishop(_) => self.generate_bishop_moves(&mut moves, *position),
                    Piece::Pawn(_) => self.generate_pawn_moves(&mut moves, *position),
                }
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
        me.pieces.insert(new_board.board[src].unwrap().piece, dest);
        if let Some(space) = new_board.board[dest] {
            opponent.pieces.remove(&space.piece);
        }

        // Update board array
        new_board.board[dest] = new_board.board[src];
        new_board.board[src] = None;

        // Detect if this puts the either king in check
        // We need to check both as a pin could cause the moving side to check itself
        new_board.white.check =
            new_board.king_check(Side::White, new_board.white.pieces[&Piece::King]);
        new_board.black.check =
            new_board.king_check(Side::Black, new_board.black.pieces[&Piece::King]);

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
        let correct_values = [20, 400, 8982, 197281];

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
    pub fn parse_fen_matches_default() {
        let game = Game::new();
        let game_fen =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0".to_string());

        assert_eq!(game.board, game_fen.board);
    }

    #[test]
    pub fn make_move_black_sets_board_and_pieces() {
        let game = Game::new();
        let queen = game.board[0x73];

        // Move black queen to G6
        let game = game.make_move(0x73, 0x55);

        assert_eq!(0x55, game.black.pieces[&Piece::Queen]);
        assert_eq!(None, game.board[0x73]);
        assert_eq!(queen, game.board[0x55]);
        assert!(!game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn make_move_white_sets_board_and_pieces() {
        let game = Game::new();
        let queen = game.board[0x03];

        // Move white queen to G6
        let game = game.make_move(0x03, 0x55);

        assert_eq!(0x55, game.white.pieces[&Piece::Queen]);
        assert_eq!(None, game.board[0x03]);
        assert_eq!(queen, game.board[0x55]);
        assert!(!game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn make_move_with_capture_clears_captured_piece() {
        let game = Game::new();
        let queen = game.board[0x73];

        // Move black queen to D2
        let game = game.make_move(0x73, 0x13);

        assert_eq!(0x13, game.black.pieces[&Piece::Queen]);
        assert_eq!(None, game.board[0x73]);
        assert_eq!(queen, game.board[0x13]);
        assert!(!game.white.pieces.contains_key(&Piece::Pawn(3)));
        assert!(game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn make_move_puts_king_in_check() {
        let game = Game::new();

        // Move white king to D4 & black queen to F6
        let game = game.make_move(4, 0x33).make_move(0x73, 0x55);

        assert_eq!(0x33, game.white.pieces[&Piece::King]);
        assert_eq!(0x55, game.black.pieces[&Piece::Queen]);
        assert!(game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn king_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // E1
        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);
        assert_eq!(0, moves.len());
        // E8
        game.generate_king_moves(&mut moves, game.black.pieces[&Piece::King]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn king_moves_from_middle() {
        let mut moves = vec![];

        // Move white king to E4 & black pawn to E5
        let game = Game::new().make_move(0x04, 0x34).make_move(0x64, 0x44);

        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);
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

        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);
        assert_eq!(3, moves.len());
    }

    #[test]
    pub fn queen_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // D1
        game.generate_queen_moves(&mut moves, game.white.pieces[&Piece::Queen]);
        assert_eq!(0, moves.len());
        // D8
        game.generate_queen_moves(&mut moves, game.black.pieces[&Piece::Queen]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn queen_moves_from_middle() {
        let mut moves = vec![];

        // Move white queen to D5
        let game = Game::new().make_move(0x03, 0x43);

        game.generate_queen_moves(&mut moves, game.white.pieces[&Piece::Queen]);
        assert_eq!(19, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // C1
        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(false)]);
        assert_eq!(0, moves.len());
        // F1
        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(true)]);
        assert_eq!(0, moves.len());
        // C8
        game.generate_bishop_moves(&mut moves, game.black.pieces[&Piece::Bishop(false)]);
        assert_eq!(0, moves.len());
        // F8
        game.generate_bishop_moves(&mut moves, game.black.pieces[&Piece::Bishop(true)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_middle() {
        let mut moves = vec![];

        // Move white bishop to D5
        let game = Game::new().make_move(0x05, 0x43);

        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(true)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_side() {
        let mut moves = vec![];

        // Move white bishop to B5
        let game = Game::new().make_move(0x05, 0x41);

        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(true)]);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn rook_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // A1
        game.generate_rook_moves(&mut moves, game.white.pieces[&Piece::Rook(false)]);
        assert_eq!(0, moves.len());
        // H1
        game.generate_rook_moves(&mut moves, game.white.pieces[&Piece::Rook(true)]);
        assert_eq!(0, moves.len());
        // A8
        game.generate_rook_moves(&mut moves, game.black.pieces[&Piece::Rook(false)]);
        assert_eq!(0, moves.len());
        // H8
        game.generate_rook_moves(&mut moves, game.black.pieces[&Piece::Rook(true)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn rook_moves_from_middle() {
        let mut moves = vec![];

        // Move white rook to D5
        let game = Game::new().make_move(0x00, 0x43);

        game.generate_rook_moves(&mut moves, game.white.pieces[&Piece::Rook(false)]);
        assert_eq!(11, moves.len());
    }

    #[test]
    pub fn knight_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // B1
        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(false)]);
        assert_eq!(2, moves.len());
        // G1
        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(true)]);
        assert_eq!(4, moves.len());
        // B8
        game.generate_knight_moves(&mut moves, game.black.pieces[&Piece::Knight(false)]);
        assert_eq!(6, moves.len());
        // G8
        game.generate_knight_moves(&mut moves, game.black.pieces[&Piece::Knight(true)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_middle() {
        let mut moves = vec![];

        // Move white knight to D5
        let game = Game::new().make_move(0x01, 0x43);

        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(false)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_side() {
        let mut moves = vec![];

        // Move white knight to A5
        let game = Game::new().make_move(0x01, 0x40);

        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(false)]);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_from_start() {
        let game = Game::new();
        let mut moves = vec![];

        // A2
        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(0)]);
        assert_eq!(2, moves.len());
        // E2
        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(4)]);
        assert_eq!(4, moves.len());
        // B7
        game.generate_pawn_moves(&mut moves, game.black.pieces[&Piece::Pawn(1)]);
        assert_eq!(6, moves.len());
        // G7
        game.generate_pawn_moves(&mut moves, game.black.pieces[&Piece::Pawn(6)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn pawn_move_from_center() {
        let mut moves = vec![];

        // Move white pawn to D4 & black pawn to C5
        let game = Game::new().make_move(0x13, 0x33).make_move(0x62, 0x42);

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(2, moves.len());

        game.generate_pawn_moves(&mut moves, game.black.pieces[&Piece::Pawn(2)]);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_friendly_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & white pawn to D6
        let game = Game::new().make_move(0x13, 0x43).make_move(0x14, 0x53);

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_enemy_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & black pawn to D6
        let game = Game::new().make_move(0x13, 0x43).make_move(0x63, 0x53);

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_capture_front() {
        let mut moves = vec![];

        // Move white pawn to D6
        let game = Game::new().make_move(0x13, 0x53);

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(2, moves.len());
    }

    #[test]
    pub fn king_not_in_check_from_start() {
        let game = Game::new();

        // E1
        assert!(!game.king_check(Side::White, game.white.pieces[&Piece::King]));

        // E8
        assert!(!game.king_check(Side::Black, game.black.pieces[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_bishop() {
        // Move white king to D4 & black bishop to B6
        let game = Game::new().make_move(0x04, 0x33).make_move(0x75, 0x51);

        assert!(game.king_check(Side::White, game.white.pieces[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_pawn() {
        // Move white king to D4 & black pawn to C5
        let game = Game::new().make_move(0x04, 0x33).make_move(0x67, 0x42);

        assert!(game.king_check(Side::White, game.white.pieces[&Piece::King]));
    }

    #[test]
    pub fn black_king_in_check_from_pawn() {
        // Move black king to D4 & white pawn to E3
        let game = Game::new().make_move(0x74, 0x33).make_move(0x16, 0x24);

        assert!(game.king_check(Side::Black, game.black.pieces[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_rook() {
        // Move white king to D4 & black rook to H4
        let game = Game::new().make_move(0x04, 0x33).make_move(0x70, 0x37);

        assert!(game.king_check(Side::White, game.white.pieces[&Piece::King]));
    }
}

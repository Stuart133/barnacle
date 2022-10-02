use std::{
    collections::HashMap,
    fmt::Display,
    mem::{discriminant, Discriminant},
    ops,
};

// Directional movement offsets using 0x88 board representation
// Missing directions are inverts of these (So we subtract)
const UP_LEFT: usize = 15;
const UP: usize = 16;
const UP_RIGHT: usize = 17;
const RIGHT: usize = 1;
const KNIGHT_MOVES: [usize; 4] = [14, 18, 31, 33];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Piece {
    King,
    Queen,
    Rook(bool),   // True if king rook
    Knight(bool), // True if king knight
    Bishop(bool), // True if king bishop
    Pawn(u8),     // Store the pawn file
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Side {
    White,
    Black,
}

impl ops::Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == Side::White {
            Side::Black
        } else {
            Side::White
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Space {
    piece: Piece,
    side: Side,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.side {
            Side::White => match self.piece {
                Piece::King => write!(f, "K"),
                Piece::Queen => write!(f, "Q"),
                Piece::Rook(_) => write!(f, "R"),
                Piece::Knight(_) => write!(f, "N"),
                Piece::Bishop(_) => write!(f, "B"),
                Piece::Pawn(_) => write!(f, "P"),
            },
            Side::Black => match self.piece {
                Piece::King => write!(f, "k"),
                Piece::Queen => write!(f, "q"),
                Piece::Rook(_) => write!(f, "r"),
                Piece::Knight(_) => write!(f, "n"),
                Piece::Bishop(_) => write!(f, "b"),
                Piece::Pawn(_) => write!(f, "p"),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Player {
    pieces: HashMap<Piece, usize>,
    check: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    board: [Option<Space>; 128], // TODO: Look into bijective map to replace this
    current_player: Side,
    white: Player,
    black: Player,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = writeln!(f, "  A B C D E F G H");
        for i in (1..9).rev() {
            write!(f, "{} ", i);
            for j in 0..8 {
                match self.board[j + ((i - 1) * 2) * 8] {
                    Some(s) => write!(f, "{} ", s),
                    None => write!(f, "  "),
                };
            }
            write!(f, "\n");
        }

        res
    }
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
            current_player: Side::White,
        }
    }

    // Create a new game object, using A Forsyth–Edwards Notation string
    pub fn from_fen(raw_game: String) -> Game {
        let mut game = Game {
            board: [None; 128],
            white: Player {
                pieces: HashMap::new(),
                check: false,
            },
            black: Player {
                pieces: HashMap::new(),
                check: false,
            },
            current_player: Side::White,
        };
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

        game.white.check = game.king_check(Side::White, game.white.pieces[&Piece::King]);
        game.black.check = game.king_check(Side::Black, game.black.pieces[&Piece::King]);
        game
    }

    #[inline(always)]
    fn get_player(&self) -> &Player {
        match self.current_player {
            Side::White => &self.white,
            Side::Black => &self.black,
        }
    }

    pub fn generate_ply(&self) -> Vec<Game> {
        let mut moves = vec![];

        let player = self.get_player();

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
                // println!("{:#02x} {}", position, offset);
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
        if position + offset & 0x88 == 0 {
            if let Some(space) = self.board[position + offset] {
                if space.side != side && discriminant(&space.piece) == attack_piece {
                    return true;
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
                        break;
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
                        break;
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
            self.make_jump_move(moves, src, src + offset);
            match src.checked_sub(*offset) {
                Some(dest) => self.make_jump_move(moves, src, dest),
                None => {}
            }
        })
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, moves: &mut Vec<Game>, src: usize) {
        // TODO: En passant
        match self.current_player {
            Side::White => {
                let dest = src + UP;
                if dest & 0x88 == 0 {
                    if let None = self.board[dest] {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test as this can't be off board
                if src >= 0x10 && src <= 0x17 && self.board[src + 0x10] == None {
                    let dest = src + UP + UP;
                    if let None = self.board[dest] {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
                let dest = src + UP_RIGHT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.board[dest]
                    {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
                let dest = src + UP_LEFT;
                if dest & 0x88 == 0 {
                    if let Some(Space {
                        side: Side::Black, ..
                    }) = self.board[dest]
                    {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
            }
            Side::Black => {
                match src.checked_sub(UP) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let None = self.board[dest] {
                                if let Some(m) = self.make_move(src, dest) {
                                    moves.push(m)
                                }
                            }
                        }
                    }
                    None => {}
                }
                // If we're on the starting space, generate the two space move
                // We can omit the off board test & checked sub as this can't be off board
                if src >= 0x60 && src <= 0x67 && self.board[src - 0x10] == None {
                    let dest = src - UP - UP;
                    if let None = self.board[dest] {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
                match src.checked_sub(UP_RIGHT) {
                    Some(dest) => {
                        if dest & 0x88 == 0 {
                            if let Some(Space {
                                side: Side::White, ..
                            }) = self.board[dest]
                            {
                                if let Some(m) = self.make_move(src, dest) {
                                    moves.push(m)
                                }
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
                                if let Some(m) = self.make_move(src, dest) {
                                    moves.push(m)
                                }
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
                        if target.side != self.current_player {
                            if let Some(m) = self.make_move(src, dest) {
                                moves.push(m)
                            }
                        }
                        break;
                    }
                    None => {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
            } else {
                break;
            }
        }
    }

    fn make_jump_move(&self, moves: &mut Vec<Game>, src: usize, dest: usize) {
        if dest & 0x88 == 0 {
            match self.board[dest] {
                Some(target) => {
                    if target.side != self.current_player {
                        if let Some(m) = self.make_move(src, dest) {
                            moves.push(m)
                        }
                    }
                }
                None => {
                    if let Some(m) = self.make_move(src, dest) {
                        moves.push(m)
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn make_move(&self, src: usize, dest: usize) -> Option<Game> {
        let mut new_board = self.clone();

        match self.current_player {
            Side::White => {
                // Update piece hashmaps
                new_board
                    .white
                    .pieces
                    .insert(new_board.board[src].unwrap().piece, dest);
                if let Some(space) = new_board.board[dest] {
                    new_board.black.pieces.remove(&space.piece);
                }

                // Update board array
                new_board.board[dest] = new_board.board[src];
                new_board.board[src] = None;

                // Did we check ourselves
                if new_board.king_check(Side::White, new_board.white.pieces[&Piece::King]) {
                    None
                } else {
                    // Did we check the opponent
                    new_board.black.check =
                        new_board.king_check(Side::Black, new_board.black.pieces[&Piece::King]);

                    new_board.current_player = !new_board.current_player;
                    Some(new_board)
                }
            }
            Side::Black => {
                // Update piece hashmaps
                new_board
                    .black
                    .pieces
                    .insert(new_board.board[src].unwrap().piece, dest);
                if let Some(space) = new_board.board[dest] {
                    new_board.white.pieces.remove(&space.piece);
                }

                // Update board array
                new_board.board[dest] = new_board.board[src];
                new_board.board[src] = None;

                // Did we check ourselves
                if new_board.king_check(Side::Black, new_board.black.pieces[&Piece::King]) {
                    None
                } else {
                    // Did we check the opponent
                    new_board.white.check =
                        new_board.king_check(Side::White, new_board.white.pieces[&Piece::King]);

                    new_board.current_player = !new_board.current_player;
                    Some(new_board)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Level {
        moves: usize,
        captures: usize,
        checks: usize,
    }

    fn perft_ply<const N: usize>(
        levels: &mut [Level; N],
        game: &Game,
        depth: usize,
        all: &mut Vec<Game>,
    ) {
        if depth == 0 {
            return;
        }

        let moves = game.generate_ply();
        levels[levels.len() - depth].moves += moves.len();

        for new_game in moves {
            all.push(new_game.clone());
            if new_game.black.check || new_game.white.check {
                levels[levels.len() - depth].checks += 1;
            }
            if new_game.black.pieces.len() + new_game.white.pieces.len()
                != game.black.pieces.len() + game.white.pieces.len()
            {
                levels[levels.len() - depth].captures += 1;

                // Sanity check
                if (game.black.pieces.len() + game.white.pieces.len())
                    - (new_game.black.pieces.len() + new_game.white.pieces.len())
                    != 1
                {
                    panic!("invalid capture");
                }
            }

            perft_ply(levels, &new_game, depth - 1, all);
        }
    }

    #[test]
    pub fn perft_test() {
        let game = Game::new();

        let moves = game.generate_ply();
        let mut output = HashMap::<&str, usize>::new();

        const DEPTH: usize = 3;

        for new_move in moves {
            let mut levels = [Level {
                moves: 0,
                captures: 0,
                checks: 0,
            }; DEPTH];
            let mut all = vec![];
            perft_ply(&mut levels, &new_move, DEPTH - 1, &mut all);
            match diff_board(&game, &new_move) {
                (0x20, Piece::Pawn(0)) => output.insert("a2a3", levels[DEPTH - 1].moves),
                (0x21, _) => output.insert("b2b3", levels[DEPTH - 1].moves),
                (0x22, Piece::Pawn(2)) => output.insert("c2c3", levels[DEPTH - 1].moves),
                (0x23, _) => output.insert("d2d3", levels[DEPTH - 1].moves),
                (0x24, _) => output.insert("e2e3", levels[DEPTH - 1].moves),
                (0x25, Piece::Pawn(5)) => output.insert("f2f3", levels[DEPTH - 1].moves),
                (0x26, _) => output.insert("g2g3", levels[DEPTH - 1].moves),
                (0x27, Piece::Pawn(7)) => output.insert("h2h3", levels[DEPTH - 1].moves),
                (0x30, _) => output.insert("a2a4", levels[DEPTH - 1].moves),
                (0x31, _) => output.insert("b2b4", levels[DEPTH - 1].moves),
                (0x32, _) => output.insert("c2c4", levels[DEPTH - 1].moves),
                (0x33, _) => output.insert("d2d4", levels[DEPTH - 1].moves),
                (0x34, _) => output.insert("e2e4", levels[DEPTH - 1].moves),
                (0x35, _) => output.insert("f2f4", levels[DEPTH - 1].moves),
                (0x36, _) => output.insert("g2g4", levels[DEPTH - 1].moves),
                (0x37, _) => output.insert("h2h4", levels[DEPTH - 1].moves),
                (0x20, _) => output.insert("b1a3", levels[DEPTH - 1].moves),
                (0x22, _) => output.insert("b1c3", levels[DEPTH - 1].moves),
                (0x25, _) => output.insert("g1f3", levels[DEPTH - 1].moves),
                (0x27, _) => output.insert("g1h3", levels[DEPTH - 1].moves),
                _ => panic!("unexpected move"),
            };
        }

        for (k, v) in output {
            if v != match k {
                "a2a3" => 380,
                "b2b3" => 420,
                "c2c3" => 420,
                "d2d3" => 539,
                "e2e3" => 599,
                "f2f3" => 380,
                "g2g3" => 420,
                "h2h3" => 380,
                "a2a4" => 420,
                "b2b4" => 421,
                "c2c4" => 441,
                "d2d4" => 560,
                "e2e4" => 600,
                "f2f4" => 401,
                "g2g4" => 421,
                "h2h4" => 420,
                "b1a3" => 400,
                "b1c3" => 440,
                "g1f3" => 440,
                "g1h3" => 400,
                _ => panic!("wrong symbol"),
            } {
                println!("{}: {}", k, v);
                assert!(false);
            }
        }
    }

    fn diff_board(a: &Game, b: &Game) -> (usize, Piece) {
        for (piece, space) in a.white.pieces.iter() {
            if b.white.pieces[piece] != *space {
                return (b.white.pieces[piece], piece.clone());
            }
        }
        for (piece, space) in a.black.pieces.iter() {
            if b.black.pieces[piece] != *space {
                return (b.white.pieces[piece], piece.clone());
            }
        }

        panic!("no diff");
    }

    #[test]
    // This is the master correctness test, if it's wrong then the move generator is not working correctly
    // See https://www.chessprogramming.org/Perft for more details
    pub fn perft() {
        let correct_levels = [
            Level {
                moves: 20,
                captures: 0,
                checks: 0,
            },
            Level {
                moves: 400,
                captures: 0,
                checks: 0,
            },
            Level {
                moves: 8902,
                captures: 34,
                checks: 12,
            },
            Level {
                moves: 197281,
                captures: 1576,
                checks: 469,
            },
        ];

        let mut all = vec![];

        let game = Game::new();
        let mut levels = [Level {
            moves: 0,
            captures: 0,
            checks: 0,
        }; 4];
        perft_ply(&mut levels, &game, 4, &mut all);

        assert_eq!(correct_levels, levels);
    }

    // #[test]
    // pub fn perft_in() {
    //     let correct_levels = [
    //         Level {
    //             moves: 14,
    //             captures: 1,
    //             checks: 2,
    //         },
    //         Level {
    //             moves: 191,
    //             captures: 14,
    //             checks: 10,
    //         },
    //         Level {
    //             moves: 2812,
    //             captures: 209,
    //             checks: 267,
    //         },
    //         Level {
    //             moves: 43238,
    //             captures: 3348,
    //             checks: 1680,
    //         },
    //     ];

    //     let game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -".to_string());
    //     let mut levels = [Level {
    //         moves: 0,
    //         captures: 0,
    //         checks: 0,
    //     }; 4];
    //     perft_ply(&mut levels, game, 4);

    //     assert_eq!(correct_levels, levels);
    // }

    #[test]
    pub fn parse_fen_matches_default() {
        let game = Game::new();
        let game_fen =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0".to_string());

        assert_eq!(game.board, game_fen.board);
    }

    #[test]
    pub fn parse_fen_with_inspection() {
        let game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -".to_string());

        assert_eq!(game.black.pieces[&Piece::Pawn(0)], 0x62);
        assert_eq!(game.black.pieces[&Piece::Pawn(1)], 0x53);
        assert_eq!(game.white.pieces[&Piece::King], 0x40);
        assert_eq!(game.white.pieces[&Piece::Pawn(0)], 0x41);
        assert_eq!(game.black.pieces[&Piece::Rook(false)], 0x47);
        assert_eq!(game.white.pieces[&Piece::Rook(false)], 0x31);
        assert_eq!(game.black.pieces[&Piece::Pawn(2)], 0x35);
        assert_eq!(game.black.pieces[&Piece::King], 0x37);
        assert_eq!(game.white.pieces[&Piece::Pawn(1)], 0x14);
        assert_eq!(game.white.pieces[&Piece::Pawn(2)], 0x16);

        assert_eq!(5, game.white.pieces.len());
        assert_eq!(5, game.black.pieces.len());
    }

    #[test]
    pub fn parse_fen_with_check() {
        let game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/r7 w - -".to_string());

        assert!(game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn parse_fen_with_black_check() {
        let game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/7R w - -".to_string());

        assert!(!game.white.check);
        assert!(game.black.check);
    }

    #[test]
    pub fn make_move_black_sets_board_and_pieces() {
        let mut game = Game::new();
        game.current_player = Side::Black;
        let queen = game.board[0x73];

        // Move black queen to G6
        let game = game.make_move(0x73, 0x55).unwrap();

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
        let game = game.make_move(0x03, 0x55).unwrap();

        assert_eq!(0x55, game.white.pieces[&Piece::Queen]);
        assert_eq!(None, game.board[0x03]);
        assert_eq!(queen, game.board[0x55]);
        assert!(!game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn make_move_with_capture_clears_captured_piece() {
        let mut game = Game::new();
        game.current_player = Side::Black;
        let queen = game.board[0x73];

        // Move black queen to D2
        let game = game.make_move(0x73, 0x13).unwrap();

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
        let game = game
            .make_move(4, 0x33)
            .unwrap()
            .make_move(0x73, 0x55)
            .unwrap();

        assert_eq!(0x33, game.white.pieces[&Piece::King]);
        assert_eq!(0x55, game.black.pieces[&Piece::Queen]);
        assert!(game.white.check);
        assert!(!game.black.check);
    }

    #[test]
    pub fn king_moves_from_start() {
        let mut game = Game::new();
        let mut moves = vec![];

        // E1
        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);
        assert_eq!(0, moves.len());

        game.current_player = Side::Black;
        // E8
        game.generate_king_moves(&mut moves, game.black.pieces[&Piece::King]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn king_moves_from_middle() {
        let mut moves = vec![];

        // Move white king to E4 & black pawn to E5
        let game = Game::new()
            .make_move(0x04, 0x34)
            .unwrap()
            .make_move(0x64, 0x44)
            .unwrap();

        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn king_moves_with_potential_check() {
        let mut moves = vec![];

        // Move white king to D5, black pawn to C5 & black bishop to C7
        let mut game = Game::new()
            .make_move(0x04, 0x43)
            .unwrap()
            .make_move(0x62, 0x42)
            .unwrap();
        game.current_player = Side::Black;
        game = game.make_move(0x75, 0x62).unwrap();

        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);
        assert_eq!(3, moves.len());
    }

    #[test]
    pub fn king_moves_with_other_checks() {
        let mut moves = vec![];

        // Move white king to D4, white rook to B4, black rook to D6
        let mut game = Game::new().make_move(0x04, 0x33).unwrap();
        game.current_player = Side::White;
        game = game
            .make_move(0x00, 0x31)
            .unwrap()
            .make_move(0x70, 0x53)
            .unwrap();
        game.current_player = Side::White;

        game.generate_king_moves(&mut moves, game.white.pieces[&Piece::King]);

        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn queen_moves_from_start() {
        let mut game = Game::new();
        let mut moves = vec![];

        // D1
        game.generate_queen_moves(&mut moves, game.white.pieces[&Piece::Queen]);
        assert_eq!(0, moves.len());

        game.current_player = Side::Black;
        // D8
        game.generate_queen_moves(&mut moves, game.black.pieces[&Piece::Queen]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn queen_moves_from_middle() {
        let mut moves = vec![];

        // Move white queen to D5
        let game = Game::new().make_move(0x03, 0x43).unwrap();

        game.generate_queen_moves(&mut moves, game.white.pieces[&Piece::Queen]);
        assert_eq!(19, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_start() {
        let mut game = Game::new();
        let mut moves = vec![];

        // C1
        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(false)]);
        assert_eq!(0, moves.len());
        // F1
        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(true)]);
        assert_eq!(0, moves.len());

        game.current_player = Side::Black;
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
        let game = Game::new().make_move(0x05, 0x43).unwrap();

        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(true)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn bishop_moves_from_side() {
        let mut moves = vec![];

        // Move white bishop to B5
        let game = Game::new().make_move(0x05, 0x41).unwrap();

        game.generate_bishop_moves(&mut moves, game.white.pieces[&Piece::Bishop(true)]);
        assert_eq!(6, moves.len());
    }

    #[test]
    pub fn rook_moves_from_start() {
        let mut game = Game::new();
        let mut moves = vec![];

        // A1
        game.generate_rook_moves(&mut moves, game.white.pieces[&Piece::Rook(false)]);
        assert_eq!(0, moves.len());
        // H1
        game.generate_rook_moves(&mut moves, game.white.pieces[&Piece::Rook(true)]);
        assert_eq!(0, moves.len());

        game.current_player = Side::Black;
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
        let game = Game::new().make_move(0x00, 0x43).unwrap();

        game.generate_rook_moves(&mut moves, game.white.pieces[&Piece::Rook(false)]);
        assert_eq!(11, moves.len());
    }

    #[test]
    pub fn knight_moves_from_start() {
        let mut game = Game::new();
        let mut moves = vec![];

        // B1
        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(false)]);
        assert_eq!(2, moves.len());
        // G1
        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(true)]);
        assert_eq!(4, moves.len());

        game.current_player = Side::Black;
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
        let mut game = Game::new().make_move(0x01, 0x43).unwrap();
        game.current_player = Side::White;

        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(false)]);
        assert_eq!(8, moves.len());
    }

    #[test]
    pub fn knight_moves_from_side() {
        let mut moves = vec![];

        // Move white knight to A5
        let mut game = Game::new().make_move(0x01, 0x40).unwrap();
        game.current_player = Side::White;

        game.generate_knight_moves(&mut moves, game.white.pieces[&Piece::Knight(false)]);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_from_start() {
        let mut game = Game::new();
        let mut moves = vec![];

        // A2
        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(0)]);
        assert_eq!(2, moves.len());
        // E2
        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(4)]);
        assert_eq!(4, moves.len());

        game.current_player = Side::Black;
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
        let mut game = Game::new()
            .make_move(0x13, 0x33)
            .unwrap()
            .make_move(0x62, 0x42)
            .unwrap();
        game.current_player = Side::White;

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(2, moves.len());

        game.current_player = Side::Black;
        game.generate_pawn_moves(&mut moves, game.black.pieces[&Piece::Pawn(2)]);
        assert_eq!(4, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_friendly_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & white pawn to D6
        let game = Game::new()
            .make_move(0x13, 0x43)
            .unwrap()
            .make_move(0x14, 0x53)
            .unwrap();

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_blocked_enemy_front() {
        let mut moves = vec![];

        // Move white pawn to D5 & black pawn to D6
        let game = Game::new()
            .make_move(0x13, 0x43)
            .unwrap()
            .make_move(0x63, 0x53)
            .unwrap();

        game.generate_pawn_moves(&mut moves, game.white.pieces[&Piece::Pawn(3)]);
        assert_eq!(0, moves.len());
    }

    #[test]
    pub fn pawn_moves_capture_front() {
        let mut moves = vec![];

        // Move white pawn to D6
        let mut game = Game::new().make_move(0x13, 0x53).unwrap();
        game.current_player = Side::White;

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
        let game = Game::new()
            .make_move(0x04, 0x33)
            .unwrap()
            .make_move(0x75, 0x51)
            .unwrap();

        assert!(game.king_check(Side::White, game.white.pieces[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_pawn() {
        // Move white king to D4 & black pawn to C5
        let game = Game::new()
            .make_move(0x04, 0x33)
            .unwrap()
            .make_move(0x67, 0x42)
            .unwrap();

        assert!(game.king_check(Side::White, game.white.pieces[&Piece::King]));
    }

    #[test]
    pub fn black_king_in_check_from_pawn() {
        let mut game = Game::new();
        game.current_player = Side::Black;

        // Move black king to D4 & white pawn to E3
        game = game
            .make_move(0x74, 0x33)
            .unwrap()
            .make_move(0x16, 0x24)
            .unwrap();

        assert!(game.king_check(Side::Black, game.black.pieces[&Piece::King]));
    }

    #[test]
    pub fn king_in_check_from_rook() {
        // Move white king to D4 & black rook to H4
        let game = Game::new()
            .make_move(0x04, 0x33)
            .unwrap()
            .make_move(0x70, 0x37)
            .unwrap();

        assert!(game.king_check(Side::White, game.white.pieces[&Piece::King]));
    }
}

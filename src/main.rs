use board::{Board, Side};

mod board;

fn main() {
    let board = Board::new();

    board.generate_moves(Side::White);
}

#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use board::{Board, Side};

mod board;

fn main() {
    let board = Board::new();

    board.generate_ply(Side::White);
}

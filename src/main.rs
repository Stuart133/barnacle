#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use std::mem::size_of;

use board::{Board, Side};

mod board;

fn main() {
    let board = Board::new();

    board.generate_ply(Side::White);

    println!("{}", size_of::<Board>());
}

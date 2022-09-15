#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use std::mem::size_of;

use board::{Game, Side};

mod board;

fn main() {
    let board = Game::new();

    board.generate_ply(Side::White);

    println!("{}", size_of::<Game>());
}

// 256 bytes for a simple repr
// 300 bytes is the copy limit perhaps?

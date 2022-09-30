#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use std::mem::size_of;

use game::Game;

mod game;

fn main() {
    let board = Game::new();

    board.generate_ply();

    println!("{}", size_of::<Game>());
}

// 256 bytes for a simple repr
// 300 bytes is the copy limit perhaps?
// 616 bytes for array & map combo

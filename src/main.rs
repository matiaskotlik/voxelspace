#![feature(trait_alias, slice_as_chunks)]

use game::Game;
use game::Scene;
use ggez::{self};

pub mod game;
pub mod gamescreen;
pub mod timer;

#[macro_use]
extern crate derivative;

use std::env;

use ggez::event;
use ggez::GameResult;

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {
    let zipped_resources = include_bytes!(concat!(env!("OUT_DIR"), "/resources.zip"));
    let cb = ggez::ContextBuilder::new("voxelspace", "qmatias")
        .add_zipfile_bytes(&zipped_resources[..]) // need a slice here or it won't compile
        .window_setup(ggez::conf::WindowSetup::default().title("Voxel Space Demo"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0).resizable(true));

    let (mut ctx, event_loop) = cb.build()?;

    let scenes: Vec<Box<dyn Scene>> = vec![Box::new(gamescreen::MapState::new(&mut ctx, 3)?)];
    let game = Game::new(scenes);
    event::run(ctx, event_loop, game);
}

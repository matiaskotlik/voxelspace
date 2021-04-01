use ggez::Context;
use ggez::GameResult;

use super::Event;
use super::World;

pub trait Scene {
    fn update(&mut self, state: &mut World, ctx: &mut Context) -> GameResult;
    fn draw(&mut self, state: &mut World, ctx: &mut Context) -> GameResult;
    fn input(
        &mut self,
        state: &mut World,
        ctx: &mut Context,
        event: Event,
        started: bool,
    ) -> GameResult;
}

extern crate rand;

mod actors;
mod animation;
mod assets;
mod behaviour;
mod helpers;
mod state;
use std::path;

#[cfg(test)]
mod tests;

use ggez::conf;
use ggez::event;
use ggez::glam::Vec2;
use ggez::ContextBuilder;

pub const SCREEN_SIZE: Vec2 = Vec2::new(1200.0, 1000.0);
fn main() {
    let c = conf::Conf::new()
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y));
    let (mut ctx, event_loop) = ContextBuilder::new("spaceship", "Pesho153Python")
        .default_conf(c)
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .unwrap();
    let state = state::State::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

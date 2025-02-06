use ggez::glam::Vec2;
use rand::Rng;

use crate::actors::{Actor, TypeActor};
use crate::SCREEN_SIZE;

pub fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

pub fn position_to_direction(current_position: Vec2, desired_pos: Vec2) -> Vec2 {
    let dx = desired_pos.x - current_position.x;
    let dy = desired_pos.y - current_position.y;
    let dist_sq = dx * dx + dy * dy;
    let distance = dist_sq.sqrt();
    Vec2::new(dx / distance, dy / distance)
}
pub fn make_rand_pos() -> Vec2 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(5.0..SCREEN_SIZE.x);
    let y = rng.gen_range(5.0..SCREEN_SIZE.y / 2.0);
    Vec2::new(x, y)
}

pub fn clamp_player(player: &mut Actor) {
    let screen_width = SCREEN_SIZE.x - 4.0;
    let screen_height = SCREEN_SIZE.y - 4.0;
    if let TypeActor::Player = player.tag {
        player.position.x = player.position.x.clamp(0.0, screen_width);
        player.position.y = player.position.y.clamp(0.0, screen_height);
    }
}

pub fn random_offscreen_position(screen_width: f32, screen_height: f32) -> Vec2 {
    let mut rng = rand::thread_rng();

    let edge = rng.gen_range(0..4);

    match edge {
        0 => Vec2 {
            x: -50.0,
            y: rng.gen_range(0.0..screen_height),
        }, // Left
        1 => Vec2 {
            x: screen_width + 50.0,
            y: rng.gen_range(0.0..screen_height),
        }, // Right
        2 => Vec2 {
            x: rng.gen_range(0.0..screen_width),
            y: -50.0,
        }, // Top
        _ => Vec2 {
            x: rng.gen_range(0.0..screen_width),
            y: screen_height + 50.0,
        }, // Bottom
    }
}

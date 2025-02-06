use ggez::glam::Vec2;
use rand::Rng;

use crate::actors::{create_shot, Actor, Subtype, TypeActor};
use crate::helpers::{make_rand_pos, position_to_direction, smoothstep};
use crate::state::InputState;
use crate::SCREEN_SIZE;

pub fn update_player_position(player: &mut Actor, input: &mut InputState, dt: f32) {
    let acceleration_rate = 12.0;
    let deceleration_rate = 8.0;
    if input.velocity.length() > 0.0 {
        player.velocity = player.velocity.lerp(input.velocity, acceleration_rate * dt);
    } else {
        player.velocity = player.velocity.lerp(Vec2::ZERO, deceleration_rate * dt);
    }
    if let TypeActor::Player = player.tag {
        player.position += player.velocity * dt;
    }
}

pub fn update_basic_enemy_movement(enemies: &mut Vec<Actor>, dt: f32) {
    for enemy in enemies {
        let enemy_speed = enemy.velocity.x;
        let t = smoothstep(enemy_speed * dt);
        enemy.position = enemy.position.lerp(enemy.desired_pos, t);
    }
}
pub fn update_shot_movement(shots: &mut Vec<Actor>, dt: f32) {
    for shot in shots {
        let shot_speed = shot.velocity.x;
        let direction = position_to_direction(shot.position, shot.desired_pos);
        shot.position += direction * shot_speed;
        shot.life_points -= dt;
        match shot.subtag {
            Subtype::BasicReloadBuff | Subtype::BasicCountBuff => shot.rotation += dt,
            Subtype::AsteroidShot => shot.rotation += dt / 4.0,
            _ => (),
        }
    }
}
pub fn boss_enemy_behaviour(
    enemy: &mut Actor,
    player: &mut Actor,
    shots: &mut Vec<Actor>,
    dt: f32,
) {
    // velocity.y is used for the speed of attacks of the boss
    enemy.velocity.y += dt * enemy.velocity.x;
    let direction = position_to_direction(enemy.position, player.position);
    enemy.rotation = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
    let distance = (enemy.desired_pos - enemy.position).length();
    if (distance < 1.5) & (enemy.velocity.y >= enemy.velocity.x) {
        let mut rng = rand::thread_rng();
        let random_choice = rng.gen_range(0..=1);
        match random_choice {
            0 => {
                enemy.desired_pos = enemy.position
                    + direction.normalize() * ((player.position - enemy.position).length() * 1.2);
            }
            1 => {
                let source_pos = Vec2::new(enemy.position.x - 25.0, enemy.position.y + 3.0);
                shoot_at_player(source_pos, player, shots);
                let source_pos = Vec2::new(enemy.position.x + 25.0, enemy.position.y + 3.0);
                shoot_at_player(source_pos, player, shots);
            }
            _ => unreachable!(),
        }
        enemy.velocity.y = 0.0;
    }
}
pub fn basic_enemy_behaviour(enemy: &mut Actor, player: &mut Actor, shots: &mut Vec<Actor>) {
    let distance = (enemy.desired_pos - enemy.position).length();

    if distance < 0.05 {
        let source_pos = Vec2::new(enemy.position.x, enemy.position.y + 3.0);
        shoot_at_player(source_pos, player, shots);
        let destination = make_rand_pos();
        enemy.desired_pos = destination;
    }
}

fn shoot_at_player(source_pos: Vec2, player: &mut Actor, shots: &mut Vec<Actor>) {
    let mut dest_pos = Vec2::new(player.position.x, player.position.y);
    let direction = position_to_direction(source_pos, dest_pos);
    dest_pos += direction * SCREEN_SIZE;
    let shot = create_shot(source_pos, dest_pos, Subtype::EnemyShot);
    shots.push(shot);
}

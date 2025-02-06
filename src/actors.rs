use core::alloc;

use ggez::glam::Vec2;
use rand::Rng;

use crate::helpers::random_offscreen_position;
use crate::SCREEN_SIZE;

#[derive(Debug)]
pub enum TypeActor {
    Player,
    Enemy,
    Shot,
    None,
}

#[derive(Debug)]
pub enum Subtype {
    BasicEnemy,
    KamikazeEnemy,
    BossEnemy,
    TankEnemy,
    BasicShot,
    EnemyShot,
    AsteroidShot,
    BasicCountBuff,
    BasicReloadBuff,
    BasicShieldBuff,
    None,
}

#[derive(Debug)]
pub struct Actor {
    pub tag: TypeActor,
    pub subtag: Subtype,
    pub velocity: Vec2,
    pub position: Vec2,
    pub desired_pos: Vec2,
    pub life_points: f32,
    pub box_size: f32,
    pub rotation: f32,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            tag: TypeActor::None,
            subtag: Subtype::None,
            velocity: Vec2::ZERO,
            position: Vec2::new(-500.0, -500.0),
            desired_pos: Vec2::ZERO,
            life_points: 0.0,
            box_size: 0.0,
            rotation: 0.0,
        }
    }
}

pub fn create_player() -> Actor {
    Actor {
        tag: TypeActor::Player,
        subtag: Subtype::None,
        velocity: Vec2::ZERO,
        position: Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0),
        desired_pos: Vec2::ZERO,
        life_points: 1.0,
        box_size: 20.0,
        rotation: 0.0,
    }
}

pub fn create_enemy(desired_pos: Vec2, starting_speed: f32, current_score: f32) -> Actor {
    let mut rng = rand::thread_rng();

    let random_choice = rng.gen_range(0..=1);
    let position = random_offscreen_position(SCREEN_SIZE.x, SCREEN_SIZE.y);
    let subtag: Subtype;
    match random_choice {
        0 => subtag = Subtype::BasicEnemy,
        1 => subtag = Subtype::KamikazeEnemy,
        _ => unreachable!(),
    }
    // using velocity for enemy speed
    let velocity = Vec2::new(starting_speed, 0.0);
    Actor {
        tag: TypeActor::Enemy,
        subtag,
        velocity,
        position,
        desired_pos,
        life_points: 0.7 + current_score / 100.0,
        box_size: 22.0,
        rotation: 0.0,
    }
}

pub fn create_shot(position: Vec2, desired_pos: Vec2, mut subtag: Subtype) -> Actor {
    let mut rotation = 0.0;
    // using velocity to decide speed of shot
    let mut velocity = Vec2::ZERO;
    let mut box_size = 1.0;
    match subtag {
        Subtype::BasicShot | Subtype::EnemyShot => {
            let direction = desired_pos - position;
            rotation = (direction.y as f64).atan2(direction.x as f64) as f32;
            rotation += std::f32::consts::FRAC_PI_2;
            velocity.x = 15.0;
        }
        //uses Basic count buff as spawner for all other possible buffs
        Subtype::BasicCountBuff => {
            let random_choice = rand::thread_rng().gen_range(0..=2);

            match random_choice {
                0 => subtag = Subtype::BasicCountBuff,
                1 => subtag = Subtype::BasicReloadBuff,
                2 => subtag = Subtype::BasicShieldBuff,
                _ => unreachable!(),
            }
            velocity.x = 2.0;
        }
        Subtype::AsteroidShot => {
            box_size = 15.0;
            velocity.x = 2.0;
        }
        _ => (),
    }

    Actor {
        tag: TypeActor::Shot,
        subtag,
        velocity,
        position,
        desired_pos,
        life_points: 15.0,
        box_size,
        rotation,
    }
}

pub fn create_boss(subtag: Subtype) -> Actor {
    let position = Vec2::new(SCREEN_SIZE.x / 2.0, -10.0);
    let desired_pos = Vec2::new(SCREEN_SIZE.x / 2.0, 80.0);
    let rotation = 0.0;
    Actor {
        tag: TypeActor::Enemy,
        subtag,
        velocity: Vec2::new(8.0, 0.0),
        position,
        desired_pos,
        life_points: 8.0,
        box_size: 20.0,
        rotation,
    }
}

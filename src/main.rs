extern crate rand;

use ggez::winit::dpi::Position;
use rand::Rng;
use std::path;

use ggez::conf;
use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics::{self, Text};
use ggez::input::keyboard::KeyCode;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameError, GameResult};

#[derive(Debug)]
enum TypeActor {
    Player,
    Enemy,
    Shot,
    None,
}

#[derive(Debug)]
enum Subtype {
    BasicEnemy,
    KamikazeEnemy,
    BossEnemy,
    TankEnemy,
    BasicShot,
    EnemyShot,
    BasicCountBuff,
    BasicReloadBuff,
    BasicShieldBuff,
    None,
}
const SCREEN_SIZE: Vec2 = Vec2::new(800.0, 1000.0);

#[derive(Debug)]
struct Actor {
    tag: TypeActor,
    subtag: Subtype,
    velocity: Vec2,
    position: Vec2,
    desired_pos: Vec2,
    life_points: f32,
    box_size: f32,
    rotation: f32,
}

impl Actor {
    fn new() -> Actor {
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

fn create_player() -> Actor {
    Actor {
        tag: TypeActor::Player,
        subtag: Subtype::None,
        velocity: Vec2::ZERO,
        position: Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0),
        desired_pos: Vec2::ZERO,
        life_points: 1.0,
        box_size: 10.0,
        rotation: 0.0,
    }
}

fn create_enemy(desired_pos: Vec2) -> Actor {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..SCREEN_SIZE.x);
    let random_choice = rng.gen_range(0..=1);
    let subtag: Subtype;
    match random_choice {
        0 => subtag = Subtype::BasicEnemy,
        1 => subtag = Subtype::KamikazeEnemy,
        _ => unreachable!(),
    }
    // using velocity for enemy speed
    let velocity = Vec2::new(7.0, 0.0);
    Actor {
        tag: TypeActor::Enemy,
        subtag,
        velocity,
        position: Vec2::new(x, -10.0),
        desired_pos,
        life_points: 1.0,
        box_size: 15.0,
        rotation: 0.0,
    }
}

fn create_shot(position: Vec2, desired_pos: Vec2, mut subtag: Subtype) -> Actor {
    let mut rotation = 0.0;
    // using velocity to decide speed of shot
    let mut velocity = Vec2::ZERO;
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
        _ => (),
    }

    Actor {
        tag: TypeActor::Shot,
        subtag,
        velocity,
        position,
        desired_pos,
        life_points: 15.0,
        box_size: 1.0,
        rotation,
    }
}

fn create_boss(subtag: Subtype) -> Actor {
    let position = Vec2::new(SCREEN_SIZE.x / 2.0, -10.0);
    let desired_pos = Vec2::new(SCREEN_SIZE.x / 2.0, 10.0);
    let rotation = 0.0;
    Actor {
        tag: TypeActor::Enemy,
        subtag,
        velocity: Vec2::new(4.0, 0.0),
        position,
        desired_pos,
        life_points: 15.0,
        box_size: 20.0,
        rotation,
    }
}
struct Assets {
    player_sprite: graphics::Image,
    basic_enemy_sprite: graphics::Image,
    kamikaze_enemy_sprite: graphics::Image,
    shot_sprite: graphics::Image,
    enemy_shot_sprite: graphics::Image,
    rocket_sprite: graphics::Image,
    blue_rocket_sprite: graphics::Image,
    shield_pickup_sprite: graphics::Image,
    shield_ui_sprite: graphics::Image,
    boss_basic_sprite: graphics::Image,
    boss_tank_sprite: graphics::Image,
}
impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_sprite = graphics::Image::from_path(ctx, "/ship_player.png")?;
        let basic_enemy_sprite = graphics::Image::from_path(ctx, "/enemy_ship_01.png")?;
        let kamikaze_enemy_sprite = graphics::Image::from_path(ctx, "/kamikaze_enemy.png")?;
        let shot_sprite = graphics::Image::from_path(ctx, "/basic_shot.png")?;
        let enemy_shot_sprite = graphics::Image::from_path(ctx, "/enemy_shot.png")?;
        let rocket_sprite = graphics::Image::from_path(ctx, "/rocket.png")?;
        let blue_rocket_sprite = graphics::Image::from_path(ctx, "/speed_buff.png")?;
        let shield_pickup_sprite = graphics::Image::from_path(ctx, "/shield_pickup.png")?;
        let shield_ui_sprite = graphics::Image::from_path(ctx, "/shield_UI.png")?;
        let boss_basic_sprite = graphics::Image::from_path(ctx, "/boss_purple.png")?;
        let boss_tank_sprite = graphics::Image::from_path(ctx, "/boss_tank.png")?;

        Ok(Assets {
            player_sprite,
            basic_enemy_sprite,
            kamikaze_enemy_sprite,
            shot_sprite,
            enemy_shot_sprite,
            rocket_sprite,
            blue_rocket_sprite,
            shield_pickup_sprite,
            shield_ui_sprite,
            boss_basic_sprite,
            boss_tank_sprite,
        })
    }
    fn actor_image(&self, actor: &Actor) -> &graphics::Image {
        match actor.tag {
            TypeActor::Player => &self.player_sprite,
            TypeActor::Enemy => match actor.subtag {
                Subtype::KamikazeEnemy => &self.kamikaze_enemy_sprite,
                Subtype::BasicEnemy => &self.basic_enemy_sprite,
                Subtype::BossEnemy => &self.boss_basic_sprite,
                Subtype::TankEnemy => &self.boss_tank_sprite,
                _ => &self.player_sprite,
            },
            TypeActor::Shot => match actor.subtag {
                Subtype::EnemyShot => &self.enemy_shot_sprite,
                Subtype::BasicShot => &self.shot_sprite,
                Subtype::BasicCountBuff => &self.rocket_sprite,
                Subtype::BasicReloadBuff => &self.blue_rocket_sprite,
                Subtype::BasicShieldBuff => &self.shield_pickup_sprite,
                _ => &self.player_sprite,
            },
            _ => &self.player_sprite,
        }
    }
}

struct InputState {
    velocity: Vec2,
    is_firing: bool,
    firing_cooldown: (f32, f32),
    count_of_weapons: f32,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            velocity: Vec2::new(0.0, 0.0),
            is_firing: false,
            firing_cooldown: (0.5, 0.8),
            count_of_weapons: 1.0,
        }
    }
}

fn boss_enemy_behaviour(enemy: &mut Actor, player: &mut Actor, shots: &[Actor]) {}
fn basic_enemy_behaviour(enemy: &mut Actor, player: &mut Actor, shots: &mut Vec<Actor>) {
    let distance = (enemy.desired_pos - enemy.position).length();

    if distance < 0.05 {
        let source_pos = Vec2::new(enemy.position.x, enemy.position.y + 3.0);
        let mut dest_pos = Vec2::new(player.position.x, player.position.y);
        let direction = position_to_direction(source_pos, dest_pos);
        dest_pos += direction * SCREEN_SIZE;
        let shot = create_shot(source_pos, dest_pos, Subtype::EnemyShot);
        shots.push(shot);
        let destination = make_rand_pos();
        enemy.desired_pos = destination;
    }
}
struct State {
    player: Actor,
    enemies: Vec<Actor>,
    shots: Vec<Actor>,
    assets: Assets,
    input: InputState,
    enemy_timer: (f32, f32),
    ability_timer: (f32, f32),
    equipped_shields: i32,
    current_score: f32,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let player = create_player();
        let assets = Assets::new(ctx)?;
        let input = InputState::default();
        let enemies = Vec::new();
        let shots = Vec::new();
        let enemy_timer = (0.0, 3.0);
        let ability_timer = (0.0, 1.0);
        let equipped_shields = 0;
        let current_score = 0.0;
        Ok(State {
            player,
            enemies,
            shots,
            assets,
            input,
            enemy_timer,
            ability_timer,
            equipped_shields,
            current_score,
        })
    }

    fn spawn_enemy(&mut self, dt: f32) {
        self.enemy_timer.0 += dt;
        if self.enemy_timer.0 >= self.enemy_timer.1 {
            let destination = make_rand_pos();
            let enemy = create_enemy(destination);
            self.enemies.push(enemy);
            self.enemy_timer.0 = 0.0;
        }
    }

    fn spawn_player_shot(&mut self, dt: f32) {
        self.input.firing_cooldown.0 += dt;
        if self.input.is_firing && self.input.firing_cooldown.0 >= self.input.firing_cooldown.1 {
            let mut dest_x = self.player.position.x;
            for i in 0..(self.input.count_of_weapons as i32) {
                if i % 2 == 0 {
                    dest_x += (i as f32) * SCREEN_SIZE.x / 6.0;
                } else {
                    dest_x -= (i as f32) * SCREEN_SIZE.x / 6.0;
                }
                let source_pos = Vec2::new(self.player.position.x, self.player.position.y - 2.0);
                let dest_pos = Vec2::new(dest_x, -100.0);
                let shot = create_shot(source_pos, dest_pos, Subtype::BasicShot);
                self.shots.push(shot);
                self.input.firing_cooldown.0 = 0.0;
            }
        }
    }

    fn handle_collision(&mut self, _ctx: &mut Context) {
        for shot in &mut self.shots {
            let distance_to_player = self.player.position - shot.position;
            if distance_to_player.length() < self.player.box_size + shot.box_size {
                if let Subtype::EnemyShot = shot.subtag {
                    self.player.life_points -= 1.0;
                    self.equipped_shields -= 1;
                    if self.player.life_points <= 0.0 {
                        self.player = Actor::new();
                    }
                }
                match shot.subtag {
                    Subtype::BasicCountBuff => {
                        self.input.count_of_weapons += 1.0;
                    }
                    Subtype::BasicReloadBuff => {
                        self.input.firing_cooldown.1 -= 0.05;
                    }
                    Subtype::BasicShieldBuff => {
                        if self.player.life_points < 4.0 {
                            self.player.life_points += 1.0;
                            self.equipped_shields += 1;
                        }
                    }
                    _ => {}
                }

                shot.life_points = 0.0;
            }

            for enemy in &mut self.enemies {
                let distance_to_enemy = enemy.position - shot.position;
                if distance_to_enemy.length() < enemy.box_size + shot.box_size {
                    if let Subtype::BasicShot = shot.subtag {
                        enemy.life_points -= 1.0;
                        shot.life_points = 0.0;
                    }
                }
            }
        }

        for enemy in &mut self.enemies {
            //player to enemy collision handle script
            let distance_to_player = self.player.position - enemy.position;
            if distance_to_player.length() < enemy.box_size + self.player.box_size {
                enemy.life_points -= 1.0;
                self.player.life_points -= 1.0;
                self.equipped_shields -= 1;
            }

            //Shot collision handle script
            for shot in &mut self.shots {
                let distance_to_player = self.player.position - shot.position;
                if distance_to_player.length() < self.player.box_size + shot.box_size {
                    if let Subtype::EnemyShot = shot.subtag {
                        self.player.life_points -= 1.0;
                        shot.life_points = 0.0;
                    }
                }

                let distance_to_enemy = enemy.position - shot.position;
                if distance_to_enemy.length() < enemy.box_size + shot.box_size {
                    if let Subtype::BasicShot = shot.subtag {
                        enemy.life_points -= 1.0;
                        shot.life_points = 0.0;
                    }
                }
            }
            if self.player.life_points <= 0.0 {
                self.player = Actor::new();
            }
        }
    }

    fn handle_life_state(&mut self, _ctx: &mut Context) {
        let predicate = |actor: &Actor| actor.life_points > 0.0;
        self.shots.retain(&predicate);
        self.enemies.retain(&predicate);
    }

    fn trigger_enemy_ability(&mut self) {
        for enemy in &mut self.enemies {
            match enemy.subtag {
                Subtype::BasicEnemy => {
                    basic_enemy_behaviour(enemy, &mut self.player, &mut self.shots);
                }
                Subtype::KamikazeEnemy => {
                    enemy.desired_pos = self.player.position;
                }
                Subtype::BossEnemy => {
                    boss_enemy_behaviour(enemy, &mut self.player, &mut self.shots);
                }
                _ => (),
            }
        }
    }

    fn spawn_ability(&mut self, dt: f32) {
        self.ability_timer.0 += dt;
        if self.ability_timer.0 >= self.ability_timer.1 {
            let rand_pos = make_rand_pos();
            let position = Vec2::new(rand_pos.x, -10.0);
            let desired_pos = Vec2::new(rand_pos.x, SCREEN_SIZE.y + 10.0);
            let shot = create_shot(position, desired_pos, Subtype::BasicCountBuff);
            self.shots.push(shot);
            self.ability_timer.0 = 0.0;
        }
    }

    fn spawn_boss(&mut self) {
        if (self.current_score.floor() % 5.0) == 1.0 {
            println!("Boss Summoned!");
            let boss = create_boss(Subtype::BossEnemy);
            self.enemies.push(boss);
        }
    }
}

fn make_rand_pos() -> Vec2 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(5.0..SCREEN_SIZE.x);
    let y = rng.gen_range(5.0..SCREEN_SIZE.y / 2.0);
    Vec2::new(x, y)
}

fn draw_scoreboard(current_score: f32, canvas: &mut graphics::Canvas) {
    let rounded_score = (current_score * 100.0).round() / 100.0;
    let score_text = Text::new(format!("Score: {}", rounded_score));
    let position = Vec2::new(SCREEN_SIZE.x / 2.0 - 50.0, 10.0);

    let drawparams = graphics::DrawParam::new()
        .dest(position)
        .scale(Vec2::new(2.0, 2.0));
    canvas.draw(&score_text, drawparams);
}
fn draw_ui_element(image: &graphics::Image, canvas: &mut graphics::Canvas, index: i32) {
    let drawparams = graphics::DrawParam::new()
        .dest(Vec2::new(10.0 + (((index - 1) as f32) * 75.0), 10.0))
        .scale(Vec2::new(5.0, 5.0));
    canvas.draw(image, drawparams);
}
fn draw_actor(
    assets: &mut Assets,
    canvas: &mut graphics::Canvas,
    actor: &Actor,
    coords: Vec2,
    rotation: f32,
) {
    let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(coords + 0.5)
        .scale(Vec2::new(5.0, 5.0))
        .rotation(rotation)
        .offset(Vec2::new(0.5, 0.5));
    canvas.draw(image, drawparams);
}
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn update_player_position(player: &mut Actor, input: &mut InputState, dt: f32) {
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

fn update_basic_enemy_movement(enemies: &mut Vec<Actor>, dt: f32) {
    for enemy in enemies {
        let enemy_speed = enemy.velocity.x;
        let t = smoothstep(enemy_speed * dt);
        enemy.position = enemy.position.lerp(enemy.desired_pos, t);
    }
}
fn position_to_direction(current_position: Vec2, desired_pos: Vec2) -> Vec2 {
    let dx = desired_pos.x - current_position.x;
    let dy = desired_pos.y - current_position.y;
    let dist_sq = dx * dx + dy * dy;
    let distance = dist_sq.sqrt();
    Vec2::new(dx / distance, dy / distance)
}
fn update_shot_movement(shots: &mut Vec<Actor>, dt: f32) {
    for shot in shots {
        let shot_speed = shot.velocity.x;
        let direction = position_to_direction(shot.position, shot.desired_pos);
        shot.position += direction * shot_speed;
        shot.life_points -= dt;
        match shot.subtag {
            Subtype::BasicReloadBuff | Subtype::BasicCountBuff => shot.rotation += dt,
            _ => (),
        }
    }
}
fn clamp_player(player: &mut Actor) {
    let screen_width = SCREEN_SIZE.x - 4.0;
    let screen_height = SCREEN_SIZE.y - 4.0;
    if let TypeActor::Player = player.tag {
        player.position.x = player.position.x.clamp(0.0, screen_width);
        player.position.y = player.position.y.clamp(0.0, screen_height);
    }
}
impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let delta_time = ctx.time.delta().as_secs_f32().min(0.1);

        //Handles player movement
        update_player_position(&mut self.player, &mut self.input, delta_time);

        //clamps player to screen size
        clamp_player(&mut self.player);

        //Basic Enemy spawn script
        self.spawn_enemy(delta_time);
        update_basic_enemy_movement(&mut self.enemies, delta_time);
        self.trigger_enemy_ability();

        //Player shot spawn
        self.spawn_player_shot(delta_time);
        update_shot_movement(&mut self.shots, delta_time);

        //Spawn player ability_timer
        self.spawn_ability(delta_time);

        //Handles collision events
        self.handle_collision(ctx);

        //Handles events when actor life reaches 0
        self.handle_life_state(ctx);

        self.current_score += delta_time;

        // Boss Spawn script
        self.spawn_boss();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        {
            let assets = &mut self.assets;
            let player = &mut self.player;
            draw_actor(
                assets,
                &mut canvas,
                player,
                player.position,
                player.rotation,
            );
            for enemy in &mut self.enemies {
                draw_actor(assets, &mut canvas, enemy, enemy.position, enemy.rotation);
            }
            for shot in &mut self.shots {
                draw_actor(assets, &mut canvas, shot, shot.position, shot.rotation);
            }
            // drawing shield (if any are picked up at all)
            for i in 1..self.equipped_shields + 1 {
                let image = assets.shield_ui_sprite.clone();
                draw_ui_element(&image, &mut canvas, i);
            }
            draw_scoreboard(self.current_score, &mut canvas);
        }

        canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        let speed = 450.0;
        match input.keycode {
            Some(KeyCode::Right) => self.input.velocity.x = speed,
            Some(KeyCode::Left) => self.input.velocity.x = -speed,
            Some(KeyCode::Down) => self.input.velocity.y = speed,
            Some(KeyCode::Up) => self.input.velocity.y = -speed,
            Some(KeyCode::R) => {
                if let TypeActor::None = self.player.tag {
                    *self = State::new(ctx).unwrap()
                }
            }
            Some(KeyCode::Space) => self.input.is_firing = true,
            _ => (),
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Right) | Some(KeyCode::Left) => self.input.velocity.x = 0.0,
            Some(KeyCode::Up) | Some(KeyCode::Down) => self.input.velocity.y = 0.0,
            Some(KeyCode::Space) => self.input.is_firing = false,
            _ => (),
        }
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new()
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y));
    let (mut ctx, event_loop) = ContextBuilder::new("spaceship", "Pesho153Python")
        .default_conf(c)
        .add_resource_path(path::PathBuf::from("./resources"))
        .build()
        .unwrap();
    let state = State::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

use std::time::Duration;

//use ggez::audio::SoundSource;
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer, Context, GameError, GameResult};
use rand::Rng;

use crate::actors::{
    create_boss, create_enemy, create_player, create_shot, Actor, Subtype, TypeActor,
};
use crate::animation::Animation;
use crate::assets::Assets;
use crate::behaviour::{
    basic_enemy_behaviour, boss_enemy_behaviour, update_basic_enemy_movement,
    update_player_position, update_shot_movement,
};
use crate::helpers::{clamp_player, make_rand_pos, random_offscreen_position};
use crate::SCREEN_SIZE;

use ggez::graphics::Text;
pub struct InputState {
    pub velocity: Vec2,
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
        .scale(Vec2::new(5.5, 5.5));
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
        .scale(Vec2::new(5.5, 5.5))
        .rotation(rotation)
        .offset(Vec2::new(0.5, 0.5));
    canvas.draw(image, drawparams);
}
pub struct State {
    player: Actor,
    enemies: Vec<Actor>,
    shots: Vec<Actor>,
    assets: Assets,
    input: InputState,
    enemy_timer: (f32, f32),
    enemy_speed: f32,
    equipped_shields: i32,
    current_score: f32,
    is_boss_present: bool,
    animations: Vec<Animation>,
    paused: bool,
    ability_timer: (f32, f32),
    asteroid_timer: (f32, f32),
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        let player = create_player();
        let assets = Assets::new(ctx)?;
        let input = InputState::default();
        let enemies = Vec::new();
        let shots = Vec::new();
        let enemy_timer = (0.0, 4.0);
        let equipped_shields = 0;
        let current_score = 0.0;
        let is_boss_present = false;
        let enemy_speed = 7.0;
        let animations = Vec::new();
        let ability_timer = (0.0, 5.0);
        let asteroid_timer = (0.0, 9.0);
        Ok(State {
            player,
            enemies,
            shots,
            assets,
            input,
            enemy_speed,
            equipped_shields,
            current_score,
            is_boss_present,
            animations,
            paused: false,
            enemy_timer,
            ability_timer,
            asteroid_timer,
        })
    }

    fn spawn_enemy(&mut self, dt: f32) {
        self.enemy_timer.0 += dt;
        if self.enemy_timer.0 >= self.enemy_timer.1 {
            let destination = make_rand_pos();
            let enemy = create_enemy(destination, self.enemy_speed, self.current_score);
            self.enemies.push(enemy);
            self.enemy_timer.0 = 0.0;
            //difficulty enhancer
            if self.enemy_timer.1 >= 1.5 {
                self.enemy_timer.1 -= 0.1;
            }
            self.enemy_speed += 0.04;
        }
    }

    fn spawn_asteroid(&mut self, dt: f32) {
        self.enemy_timer.0 += dt;
        if self.enemy_timer.0 >= self.enemy_timer.1 {
            let source_pos = random_offscreen_position(SCREEN_SIZE.x, SCREEN_SIZE.y);
            let destination = random_offscreen_position(SCREEN_SIZE.x, SCREEN_SIZE.y);
            let shot = create_shot(source_pos, destination, Subtype::AsteroidShot);
            self.shots.push(shot);
            self.asteroid_timer.0 = 0.0;
        }
    }
    fn spawn_player_shot(&mut self, dt: f32) {
        self.input.firing_cooldown.0 += dt;
        if self.input.is_firing && self.input.firing_cooldown.0 >= self.input.firing_cooldown.1 {
            let mut dest_x = self.player.position.x;
            for i in 0..(self.input.count_of_weapons as i32) {
                if i % 2 == 0 {
                    dest_x += (i as f32) * SCREEN_SIZE.x / 4.0;
                } else {
                    dest_x -= (i as f32) * SCREEN_SIZE.x / 4.0;
                }
                let source_pos = Vec2::new(self.player.position.x, self.player.position.y - 2.0);
                let dest_pos = Vec2::new(dest_x, -100.0);
                let shot = create_shot(source_pos, dest_pos, Subtype::BasicShot);
                self.shots.push(shot);
                self.input.firing_cooldown.0 = 0.0;
            }
        }
    }

    fn handle_collision(&mut self, ctx: &mut Context) {
        for shot in &mut self.shots {
            let distance_to_player = self.player.position - shot.position;
            if distance_to_player.length() < self.player.box_size + shot.box_size {
                match shot.subtag {
                    Subtype::EnemyShot | Subtype::AsteroidShot => {
                        shot.life_points = 0.0;
                        self.player.life_points -= 1.0;
                        self.equipped_shields -= 1;
                        if self.player.life_points <= 0.0 {
                            self.animations.push(
                                Animation::new(
                                    ctx,
                                    "/explosion_sheet.png",
                                    4,
                                    Duration::from_millis(100),
                                    self.player.position,
                                )
                                .unwrap(),
                            );
                            self.player = Actor::new();
                            self.paused = true;
                        }
                    }
                    _ => (),
                }
                match shot.subtag {
                    Subtype::BasicCountBuff => {
                        self.input.count_of_weapons += 1.0;
                        shot.life_points = 0.0;
                    }
                    Subtype::BasicReloadBuff => {
                        self.input.firing_cooldown.1 -= 0.02;
                        shot.life_points = 0.0;
                    }
                    Subtype::BasicShieldBuff => {
                        if self.player.life_points < 4.0 {
                            self.player.life_points += 1.0;
                            self.equipped_shields += 1;
                        }
                        shot.life_points = 0.0;
                    }
                    _ => {}
                }
            }

            for enemy in &mut self.enemies {
                let distance_to_enemy = enemy.position - shot.position;
                if distance_to_enemy.length() < enemy.box_size + shot.box_size {
                    match shot.subtag {
                        Subtype::BasicShot | Subtype::AsteroidShot => {
                            match enemy.subtag {
                                Subtype::BasicEnemy | Subtype::KamikazeEnemy => {
                                    enemy.life_points -= 1.0;
                                }
                                Subtype::BossEnemy => {
                                    shot.life_points = 0.0;
                                    enemy.life_points -= 3.0;
                                }
                                _ => (),
                            }
                            if let Subtype::BasicShot = shot.subtag {
                                shot.life_points = 0.0;
                            }
                            self.animations.push(
                                Animation::new(
                                    ctx,
                                    "/explosion_sheet.png",
                                    4,
                                    Duration::from_millis(100),
                                    enemy.position,
                                )
                                .unwrap(),
                            );
                        }
                        _ => (),
                    }
                }
            }
        }

        for enemy in &mut self.enemies {
            //player to enemy collision handle script
            let distance_to_player = self.player.position - enemy.position;
            if distance_to_player.length() < enemy.box_size + self.player.box_size {
                match enemy.subtag {
                    Subtype::KamikazeEnemy | Subtype::BasicEnemy => {
                        enemy.life_points = 0.0;
                        self.animations.push(
                            Animation::new(
                                ctx,
                                "/explosion_sheet.png",
                                4,
                                Duration::from_millis(100),
                                enemy.position,
                            )
                            .unwrap(),
                        );
                    }
                    _ => (),
                }
                self.player.life_points -= 1.0;
                self.equipped_shields -= 1;
                if self.player.life_points <= 0.0 {
                    self.animations.push(
                        Animation::new(
                            ctx,
                            "/explosion_sheet.png",
                            4,
                            Duration::from_millis(100),
                            self.player.position,
                        )
                        .unwrap(),
                    );
                }
            }

            //Shot collision handle script
            for i in 0..self.shots.len() {
                let distance_to_player = self.player.position - self.shots[i].position;
                if distance_to_player.length() < self.player.box_size + self.shots[i].box_size {
                    if let Subtype::EnemyShot = self.shots[i].subtag {
                        self.player.life_points -= 1.0;
                        self.equipped_shields -= 1;
                        self.shots[i].life_points = 0.0;
                        self.animations.push(
                            Animation::new(
                                ctx,
                                "/explosion_sheet.png",
                                4,
                                Duration::from_millis(100),
                                enemy.position,
                            )
                            .unwrap(),
                        );
                    }
                }

                let distance_to_enemy = enemy.position - self.shots[i].position;
                if distance_to_enemy.length() < enemy.box_size + self.shots[i].box_size {
                    if let Subtype::BasicShot = self.shots[i].subtag {
                        enemy.life_points -= 1.0;
                        self.shots[i].life_points = 0.0;
                        //sets boss bool to false if boss is dead
                        if enemy.life_points <= -1.0 {
                            match enemy.subtag {
                                Subtype::BossEnemy /*| Subtype::TankEnemy*/ => {
                                    self.is_boss_present = false;
                                }
                                Subtype::BasicEnemy | Subtype::KamikazeEnemy => {
                                    self.animations.push(
                                        Animation::new(
                                            ctx,
                                            "/explosion_sheet.png",
                                            4,
                                            Duration::from_millis(100),
                                            enemy.position,
                                        )
                                        .unwrap(),
                                    );
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
            if self.player.life_points <= 0.0 {
                self.player = Actor::new();
                self.paused = true;
            }
        }
        self.handle_life_state();
    }

    //could optimise this, but no need at current moment
    fn handle_life_state(&mut self) {
        let predicate = |actor: &Actor| actor.life_points > 0.0;
        self.shots.retain(&predicate);
        self.enemies.retain(&predicate);
    }

    fn trigger_enemy_ability(&mut self, dt: f32) {
        for enemy in &mut self.enemies {
            match enemy.subtag {
                Subtype::BasicEnemy => {
                    basic_enemy_behaviour(enemy, &mut self.player, &mut self.shots);
                }
                Subtype::KamikazeEnemy => {
                    enemy.desired_pos = self.player.position;
                }
                Subtype::BossEnemy => {
                    boss_enemy_behaviour(enemy, &mut self.player, &mut self.shots, dt);
                }
                _ => (),
            }
        }
    }

    fn spawn_boss(&mut self) {
        if ((self.current_score.ceil() % 40.0) == 0.0) & (!self.is_boss_present) {
            self.is_boss_present = true;
            let boss = create_boss(Subtype::BossEnemy);
            self.enemies.push(boss);
            self.enemy_timer.1 += 1.5;
        }
    }

    fn animation_handler(&mut self) {
        self.animations.retain_mut(|animation| {
            animation.update(Duration::from_millis(16));
            !animation.finished
        });
    }

    fn spawn_behaviour(&mut self, dt: f32) {
        self.ability_timer.0 += dt;
        if self.ability_timer.0 >= self.ability_timer.1 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(5.0..SCREEN_SIZE.x);
            let y = -10.0;
            spawn_ability(Vec2::new(x, y), &mut self.shots);
            self.ability_timer.0 = 0.0;
        }
    }
}

//uses Basic count buff as spawner for all other possible buffs
fn spawn_ability(source_pos: Vec2, shots: &mut Vec<Actor>) {
    let position = Vec2::new(source_pos.x, source_pos.y);
    let desired_pos = Vec2::new(source_pos.x, SCREEN_SIZE.y + 30.0);
    let shot = create_shot(position, desired_pos, Subtype::BasicCountBuff);
    shots.push(shot);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if self.paused {
            return Ok(());
        }

        let delta_time = ctx.time.delta().as_secs_f32().min(0.1);
        //Handles player movement
        update_player_position(&mut self.player, &mut self.input, delta_time);

        //clamps player to screen size
        clamp_player(&mut self.player);

        //Basic Enemy spawn script
        self.spawn_enemy(delta_time);
        update_basic_enemy_movement(&mut self.enemies, delta_time);
        self.trigger_enemy_ability(delta_time);

        //Player shot spawn
        self.spawn_player_shot(delta_time);
        update_shot_movement(&mut self.shots, delta_time);

        self.spawn_asteroid(delta_time);

        //Handles collision events
        self.handle_collision(ctx);

        self.current_score += delta_time;

        // Boss Spawn script
        self.spawn_boss();

        self.animation_handler();

        self.spawn_behaviour(delta_time);
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

            for animation in &mut self.animations {
                animation.draw(ctx, &mut canvas);
            }

            if self.paused {
                draw_game_over_screen(&mut canvas);
            }
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
        let speed = 600.0;
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
            Some(KeyCode::Space) => {
                self.input.is_firing = true;
                // It can play a sound - but let's not do that :)
                //self.assets.shoot_sound.play_later();
            }
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

fn draw_game_over_screen(canvas: &mut graphics::Canvas) {
    let score_text = Text::new("GAME OVER");
    let position = SCREEN_SIZE / 2.0 - 150.0;
    let drawparams = graphics::DrawParam::new()
        .dest(position)
        .scale(Vec2::new(5.0, 5.0));
    canvas.draw(&score_text, drawparams);
}

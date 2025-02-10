use crate::actors::{Actor, Subtype, TypeActor};
use ggez::{/*audio,*/ graphics};
use ggez::{Context, GameResult};

pub struct Assets {
    pub player_sprite: graphics::Image,
    pub basic_enemy_sprite: graphics::Image,
    pub kamikaze_enemy_sprite: graphics::Image,
    pub shot_sprite: graphics::Image,
    pub enemy_shot_sprite: graphics::Image,
    pub rocket_sprite: graphics::Image,
    pub blue_rocket_sprite: graphics::Image,
    pub shield_pickup_sprite: graphics::Image,
    pub shield_ui_sprite: graphics::Image,
    pub boss_basic_sprite: graphics::Image,
    //pub boss_tank_sprite: graphics::Image,
    pub asteroid_sprite: graphics::Image,
    //pub shoot_sound: audio::Source,
}
impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
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
        //let boss_tank_sprite = graphics::Image::from_path(ctx, "/boss_tank.png")?;
        let asteroid_sprite = graphics::Image::from_path(ctx, "/asteroid_shot.png")?;
        //let shoot_sound =
        //    audio::Source::new(ctx, "/Bluezone_BC0295_sci_fi_weapon_gun_shot_008.wav")?;

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
            //boss_tank_sprite,
            asteroid_sprite,
            //shoot_sound,
        })
    }
    pub fn actor_image(&self, actor: &Actor) -> &graphics::Image {
        match actor.tag {
            TypeActor::Player => &self.player_sprite,
            TypeActor::Enemy => match actor.subtag {
                Subtype::KamikazeEnemy => &self.kamikaze_enemy_sprite,
                Subtype::BasicEnemy => &self.basic_enemy_sprite,
                Subtype::BossEnemy => &self.boss_basic_sprite,
                //Subtype::TankEnemy => &self.boss_tank_sprite,
                _ => &self.player_sprite,
            },
            TypeActor::Shot => match actor.subtag {
                Subtype::EnemyShot => &self.enemy_shot_sprite,
                Subtype::BasicShot => &self.shot_sprite,
                Subtype::BasicCountBuff => &self.rocket_sprite,
                Subtype::BasicReloadBuff => &self.blue_rocket_sprite,
                Subtype::BasicShieldBuff => &self.shield_pickup_sprite,
                Subtype::AsteroidShot => &self.asteroid_sprite,
                _ => &self.player_sprite,
            },
            _ => &self.player_sprite,
        }
    }
}

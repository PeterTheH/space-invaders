#[cfg(test)]
mod test {
    use crate::actors::{Actor, TypeActor};
    use crate::helpers::{
        clamp_player, make_rand_pos, position_to_direction, random_offscreen_position,
    };
    use crate::SCREEN_SIZE;
    use ggez::glam::Vec2;

    #[test]
    fn test_position_to_direction() {
        let current = Vec2::new(0.0, 0.0);
        let desired = Vec2::new(3.0, 4.0);
        let direction = position_to_direction(current, desired);
        assert!((direction.x - 0.6).abs() < f32::EPSILON);
        assert!((direction.y - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_make_rand_pos() {
        let pos = make_rand_pos();
        assert!(pos.x >= 5.0 && pos.x <= SCREEN_SIZE.x);
        assert!(pos.y >= 0.0 && pos.y <= SCREEN_SIZE.y / 2.0);
    }

    #[test]
    fn test_clamp_player() {
        let mut player = Actor {
            position: Vec2::new(-10.0, SCREEN_SIZE.y + 10.0),
            tag: TypeActor::Player,
            subtag: crate::actors::Subtype::BasicEnemy,
            velocity: Vec2::new(10.0, 10.0),
            desired_pos: Vec2::new(0.0, 0.0),
            life_points: 0.0,
            box_size: 0.0,
            rotation: 0.0,
        };
        clamp_player(&mut player);
        assert!(player.position.x >= 0.0 && player.position.x <= SCREEN_SIZE.x - 4.0);
        assert!(player.position.y >= 0.0 && player.position.y <= SCREEN_SIZE.y - 4.0);
    }

    #[test]
    fn test_random_offscreen_position() {
        let screen_width = SCREEN_SIZE.x;
        let screen_height = SCREEN_SIZE.y;
        let pos = random_offscreen_position(screen_width, screen_height);

        let is_offscreen =
            pos.x < 0.0 || pos.x > screen_width || pos.y < 0.0 || pos.y > screen_height;
        assert!(is_offscreen);
    }
}

use ggez::glam::Vec2;
use ggez::graphics::{self, DrawParam, Image, Rect};
use ggez::{Context, GameResult};
use std::time::Duration;

pub struct Animation {
    sprite_sheet: Image,
    frame_count: usize,
    current_frame: usize,
    frame_time: Duration,
    elapsed_time: Duration,
    position: Vec2,
    pub finished: bool,
}

impl Animation {
    pub fn new(
        ctx: &mut Context,
        path: &str,
        frame_count: usize,
        frame_time: Duration,
        position: Vec2,
    ) -> GameResult<Animation> {
        let sprite_sheet = Image::from_path(ctx, path)?;
        let finished = false;
        Ok(Self {
            sprite_sheet,
            frame_count,
            current_frame: 0,
            frame_time,
            elapsed_time: Duration::new(0, 0),
            position,
            finished,
        })
    }

    pub fn update(&mut self, dt: Duration) {
        self.elapsed_time += dt;
        if self.elapsed_time >= self.frame_time {
            self.elapsed_time = Duration::from_secs_f32(0.0);
            self.current_frame = (self.current_frame + 1) % self.frame_count;
        }
        if self.current_frame + 1 == self.frame_count {
            self.finished = true;
        }
    }
    pub fn draw(&self, _ctx: &mut Context, canvas: &mut graphics::Canvas) {
        let frame_width_ratio = 1.0 / self.frame_count as f32;
        let padding = 0.002;
        let src_rect = Rect::new(
            self.current_frame as f32 * frame_width_ratio + padding,
            padding,
            frame_width_ratio - 2.0 * padding,
            1.0 - 2.0 * padding,
        );

        let params = DrawParam::default()
            .src(src_rect)
            .scale([4.0, 4.0])
            .dest(self.position);

        canvas.draw(&self.sprite_sheet, params)
    }
}

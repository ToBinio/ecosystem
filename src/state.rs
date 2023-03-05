use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, FillOptions, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameError};

pub struct State {}

impl State {}

impl EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        let mesh = Mesh::new_circle(
            &ctx.gfx,
            DrawMode::Fill(FillOptions::default()),
            Point2 { x: 0.0, y: 0.0 },
            100.0,
            0.6,
            Color::RED,
        )?;

        canvas.draw(&mesh, Vec2::new(ctx.gfx.size().0 / 2.0, ctx.gfx.size().1 / 2.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

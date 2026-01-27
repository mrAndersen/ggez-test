use ggez::conf::{Backend, FullscreenType, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Text, TextFragment};
use ggez::winit::window::Fullscreen;
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("td", "mrAndersen")
        .window_setup(WindowSetup {
            title: "TD".to_string(),
            vsync: false,
            ..Default::default()
        })
        .build()
        .expect("Unable to create ggez context");

    ctx.gfx
        .window()
        .set_fullscreen(Some(Fullscreen::Borderless(None)));

    event::run(ctx, event_loop, TestGame {});
}

struct TestGame {}

impl EventHandler for TestGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(195, 227, 226));
        let fps = ctx.time.fps();
        let fps_text = Text::new(TextFragment::new(format!("{:.0}", fps)).scale(80.0));
        let sz = fps_text.measure(ctx)?;
        let (w, h) = ctx.gfx.drawable_size();

        canvas.draw(
            &fps_text,
            DrawParam::from([w / 2.0 - sz.x / 2.0, h / 2.0 - sz.y / 2.0])
                .color(Color::BLACK)
                .scale([1.0, 1.0]),
        );
        canvas.finish(ctx)
    }
}

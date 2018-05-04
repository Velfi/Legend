use super::{ggez::{event,
                   graphics::{self, DrawMode, Point2},
                   Context,
                   GameResult},
            State,
            StateTransition};

/// The main state for now. Will maybe become a menu later.
pub struct MainState {
    pos_x: f32,
    quit: bool,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            quit: false,
        };
        Ok(s)
    }
}

impl State for MainState {
    fn process_event(&mut self, event: &event::Event) {
        match event {
            event::Event::Quit { .. }
            | event::Event::KeyDown {
                keycode: Some(event::Keycode::Escape),
                ..
            } => {
                println!("Quitting");
                self.quit = true;
            }
            input => println!("Event fired: {:?}", input),
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<StateTransition, ()> {
        if self.quit {
            Ok(StateTransition::Quit)
        } else {
            self.pos_x = self.pos_x % 800.0 + 1.0;
            Ok(StateTransition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
        )?;
        graphics::present(ctx);
        Ok(())
    }
}

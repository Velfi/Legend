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
    debug_battle: bool,
    state_label: graphics::Text,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/font.ttf", 48)?;
        let s = MainState {
            pos_x: 0.0,
            quit: false,
            state_label: graphics::Text::new(ctx, "Main State", &font)?,
            debug_battle: false,
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
            | event::Event::KeyDown {
                keycode: Some(event::Keycode::B),
                ..
            } => {
                println!("Battle Starting");
                self.debug_battle = true;
            }
            input => println!("Event fired: {:?}", input),
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Result<StateTransition, ()> {
        if self.quit {
            Ok(StateTransition::Quit)
        } else if self.debug_battle {
            let battle = super::battle_state::BattleState::new(ctx).expect("Failed to create Battle State.");

            Ok(StateTransition::Push(Box::new(battle)))
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
        graphics::draw(
            ctx,
            &self.state_label,
            Point2::new(600.0, 0.0),
            0.0
        )?;
        graphics::present(ctx);
        Ok(())
    }
}

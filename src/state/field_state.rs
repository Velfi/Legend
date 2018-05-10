use super::{ggez::{event,
                   graphics::{self, Point2},
                   Context,
                   GameResult},
            State,
            StateTransition};

use super::super::ui::{button::Button, Ui};

/// The main state for now. Will maybe become a menu later.
pub struct FieldState {
    quit: bool,
    state_label: graphics::Text,
    ui: Ui,
}

impl FieldState {
    pub fn new(ctx: &mut Context) -> GameResult<FieldState> {
        let font_sm = graphics::Font::new(ctx, "/font.ttf", 12)?;
        let font_md = graphics::Font::new(ctx, "/font.ttf", 24)?;
        let font_lg = graphics::Font::new(ctx, "/font.ttf", 48)?;

        let mut battle_ui = Ui::default();
        battle_ui.add_font("small".to_string(), font_sm);
        battle_ui.add_font("medium".to_string(), font_md);
        battle_ui.add_font("large".to_string(), font_lg);

        let button_label =
            graphics::Text::new(ctx, "Test Button", &battle_ui.fonts.get("medium").unwrap())
                .unwrap();

        let button = Button::new(
            ctx,
            12,
            button_label,
            graphics::Rect::new(200.0, 200.0, 120.0, 48.0),
            graphics::Color::from_rgb(0, 50, 200),
            graphics::Color::from_rgb(200, 50, 0),
        );

        battle_ui.add_element(Box::new(button));

        let s = FieldState {
            quit: false,
            state_label: graphics::Text::new(
                ctx,
                "Field State",
                &battle_ui.fonts.get("large").unwrap(),
            )?,
            ui: battle_ui,
        };
        Ok(s)
    }
}

impl State for FieldState {
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
            input => {
                self.ui.process_event(input);
            }
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<StateTransition, ()> {
        if self.quit {
            Ok(StateTransition::Quit)
        } else {
            Ok(StateTransition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw(ctx, &self.state_label, Point2::new(600.0, 10.0), 0.0)?;
        self.ui.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }
}

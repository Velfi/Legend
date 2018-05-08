use super::ggez::{
    Context,
    GameResult,
    graphics
};

use super::shrev::EventChannel;

use std::default::Default;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Ui {
    elements: Vec<Box<Element>>,
    events: EventChannel<UiEvent>,
    fonts: HashMap<String, graphics::Font>,
}

impl Ui {
    pub fn add_font(&mut self, font_name: String, font_file: graphics::Font) {
        match self.fonts.entry(font_name) {
            Entry::Occupied(o) => println!("Attempted to add font '{}' but it was already loaded by Ui", &o.key()),
            Entry::Vacant(v) => {v.insert(font_file);},
        };
    }

    pub fn add_element(&mut self, element: Box<Element>) {
        self.elements.push(element)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // TODO: This will swallow errors if more than one draw call fails!
        let mut result = Ok(());
        for element in self.elements.iter() {
            if let Err(error) = element.draw(ctx) {
                result = Err(error)
            }
        }
        result
    }
}

impl Default for Ui {
    fn default() -> Self {
        Ui {
            elements: Vec::new(),
            events: EventChannel::new(),
            fonts: HashMap::new(),
        }
    }
}

pub trait Element {
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;

    fn set_hover(&mut self, is_hovered: bool);

    fn id(&self) -> usize;
}

pub struct Button {
    pub bg_color: graphics::Color,
    pub height: usize,
    pub hover_bg_color: graphics::Color,
    pub id: usize,
    pub is_hovered: bool,
    pub label: graphics::Text,
    pub position: graphics::Point2,
    pub width: usize,
}

impl Button {
    fn get_center_point(&self) -> graphics::Point2 {
        let offset_x = self.width as f32 / 2.0;
        let offset_y = self.height as f32 / 2.0;

        let pos_x = self.position.x;
        let pos_y = self.position.y;

        graphics::Point2::new(
            pos_x + offset_x,
            pos_y + offset_y
        )
    }
}

impl Element for Button {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        let bg_color = if self.is_hovered {
            self.hover_bg_color
        } else {
            self.bg_color
        };

        let rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: self.width as f32,
            h: self.height as f32,
        };

        let bg_params = graphics::DrawParam {
            dest: self.position,
            color: Some(bg_color),
            ..Default::default()
        };

        let label_params = graphics::DrawParam {
            offset: graphics::Point2::new(0.5, 0.5),
            dest: self.get_center_point(),
            ..Default::default()
        };

        graphics::set_color(ctx, bg_color);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect);
        graphics::draw_ex(ctx, &self.label, label_params);

        Ok(())
    }

    fn set_hover(&mut self, is_hovered: bool) {
        self.is_hovered = is_hovered;
    }

    fn id(&self) -> usize {
        self.id
    }
}

pub enum UiEvent {
    Click(usize),
    MouseEnter(usize),
    MouseLeave(usize)
}

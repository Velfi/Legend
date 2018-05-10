use super::ggez::{graphics::{self, DrawMode, Mesh, Point2, Rect, Text},
                  Context,
                  GameResult,
                  event};

use super::shrev::EventChannel;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::default::Default;

pub struct Ui {
    elements: Vec<Box<Element>>,
    events: EventChannel<UiEvent>,
    fonts: HashMap<String, graphics::Font>,
}

impl Ui {
    pub fn add_font(&mut self, font_name: String, font_file: graphics::Font) {
        match self.fonts.entry(font_name) {
            Entry::Occupied(o) => println!(
                "Attempted to add font '{}' but it was already loaded by Ui",
                &o.key()
            ),
            Entry::Vacant(v) => {
                v.insert(font_file);
            }
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

    pub fn process_event(&mut self, event: &event::Event) {
        ()
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
    pub hover_bg_color: graphics::Color,
    pub id: usize,
    is_hovered: bool,
    pub label: Text,
    mesh: Mesh,
    rect: Rect,
}

impl Button {
    pub fn new(
        ctx: &mut Context,
        id: usize,
        label: Text,
        rect: graphics::Rect,
        bg_color: graphics::Color,
        hover_bg_color: graphics::Color,
    ) -> Self {
        let x1 = rect.x;
        let x2 = rect.x + rect.w;
        let y1 = rect.y;
        let y2 = rect.y + rect.h;
        let vertices = [
            Point2::new(x1, y1),
            Point2::new(x2, y1),
            Point2::new(x2, y2),
            Point2::new(x1, y2),
        ];

        let m = Mesh::new_polygon(ctx, DrawMode::Fill, &vertices).unwrap();

        Button {
            id,
            mesh: m,
            label,
            bg_color,
            hover_bg_color,
            rect,
            is_hovered: false,
        }
    }

    fn get_center_point(&self) -> graphics::Point2 {
        let offset_x = self.rect.w / 2.0;
        let offset_y = self.rect.h / 2.0;

        let pos_x = self.rect.x;
        let pos_y = self.rect.y;

        graphics::Point2::new(pos_x + offset_x, pos_y + offset_y)
    }
}

impl Element for Button {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let bg_color = if self.is_hovered {
            self.hover_bg_color
        } else {
            self.bg_color
        };

        let bg_params = graphics::DrawParam {
            // dest: self.position,
            color: Some(bg_color),
            ..Default::default()
        };

        let label_params = graphics::DrawParam {
            offset: graphics::Point2::new(0.5, 0.5),
            dest: self.get_center_point(),
            ..Default::default()
        };

        graphics::draw_ex(ctx, &self.mesh, bg_params);
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
    MouseLeave(usize),
}

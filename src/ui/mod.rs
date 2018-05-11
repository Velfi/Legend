use super::ggez::{event::{self, Event, MouseButton},
                  graphics::{self, Point2, Rect},
                  Context,
                  GameResult};

use super::shrev::EventChannel;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::default::Default;

pub mod button;

pub struct Ui {
    elements: Vec<Box<Element>>,
    pub events: EventChannel<UiEvent>,
    pub fonts: HashMap<String, graphics::Font>,
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
        match *event {
            Event::MouseMotion { x, y, .. } => {
                let point = Point2::new(x as f32, y as f32);
                for element in self.elements.iter_mut() {
                    let collision = element.check_collision(&point);
                    if collision && !element.is_hovered() {
                        element.mouse_enter();
                    } else if !collision && element.is_hovered() {
                        element.mouse_leave();
                    }
                }
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } if mouse_btn == MouseButton::Left =>
            {
                let point = Point2::new(x as f32, y as f32);
                for element in self.elements.iter_mut() {
                    if element.check_collision(&point) {
                        element.click_start();
                    }
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } if mouse_btn == MouseButton::Left =>
            {
                let point = Point2::new(x as f32, y as f32);
                for element in self.elements.iter_mut() {
                    if element.check_collision(&point) {
                        element.click_end();
                    }
                }
            }
            _ => (),
        };
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

    fn id(&self) -> usize;

    fn check_collision(&self, point: &Point2) -> bool;

    fn mouse_enter(&mut self) {}

    fn mouse_leave(&mut self) {}

    fn click(&mut self) {}

    fn click_start(&mut self) {}

    fn click_end(&mut self) {}

    fn is_hovered(&self) -> bool {
        false
    }
}

pub enum UiEvent {
    Click(usize),
    MouseEnter(usize),
    MouseLeave(usize),
}

pub fn point_collision(point: &Point2, bounds: &Rect) -> bool {
    point.x > bounds.x && point.x < bounds.x + bounds.w && point.y > bounds.y
        && point.y < bounds.y + bounds.h
}

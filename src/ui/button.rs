use super::super::ggez::{graphics::{self, Color, DrawMode, Mesh, Point2, Rect, Text},
                         Context,
                         GameResult};

use super::{point_collision, Element};

pub struct Button {
    pub bg_color: Color,
    pub hover_bg_color: Color,
    pub id: usize,
    is_hovered: bool,
    is_clicked: bool,
    pub label: Text,
    mesh: Mesh,
    rect: Rect,
    pub click_callback: Box<FnMut()>,
}

impl Button {
    pub fn new(
        ctx: &mut Context,
        id: usize,
        label: Text,
        rect: Rect,
        bg_color: Color,
        hover_bg_color: Color,
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
            is_clicked: false,
            is_hovered: false,
            click_callback: Box::new(move || ()),
        }
    }

    fn get_center_point(&self) -> Point2 {
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

        graphics::draw_ex(ctx, &self.mesh, bg_params)?;
        graphics::draw_ex(ctx, &self.label, label_params)?;

        Ok(())
    }

    fn id(&self) -> usize {
        self.id
    }

    fn check_collision(&self, point: &Point2) -> bool {
        point_collision(&point, &self.rect)
    }

    fn click_start(&mut self) {
        self.is_clicked = true;
    }

    fn click_end(&mut self) {
        if self.is_clicked {
            self.click();
        }
        self.is_clicked = false;
    }

    fn mouse_enter(&mut self) {
        self.is_hovered = true;
    }

    fn mouse_leave(&mut self) {
        self.is_hovered = false;
        self.is_clicked = false;
    }

    fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    fn click(&mut self) {
        (self.click_callback)();
    }
}

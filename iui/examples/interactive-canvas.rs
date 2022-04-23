extern crate iui;
extern crate ui_sys;

use iui::controls::{Area, AreaDrawParams, AreaHandler, AreaMouseEvent, HorizontalBox, LayoutStrategy};
use iui::draw::{Brush, FillMode, Path, SolidBrush};
use iui::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

// This example shows how to use a reference-counted AreaHandler implementation to create an
// interactive canvas. The canvas is a simple colored box that changes color when the mouse is
// clicked. The value of the color can be changed by the user, or programmatically. The handler
// also includes an event handler that can be used to respond to user-initiated changes.

enum Color {
    Red,
    Green,
    Blue,
}

struct ColorCanvas {
    color: Color,
    on_changed: Option<Box<dyn FnMut(&mut ColorCanvas, &Area)>>,
}

impl ColorCanvas {
    fn new(color: Color) -> Self {
        Self {
            color,
            on_changed: None,
        }
    }

    fn raise_changed(&mut self, area: &Area) {
        if self.on_changed.is_some() {
            let mut f = self.on_changed.take().unwrap();
            f(self, area);
            if self.on_changed.is_none() {
                self.on_changed = Some(f);
            }
        }
    }
}

impl AreaHandler for ColorCanvas {
    fn draw(&mut self, _area: &Area, draw_params: &AreaDrawParams) {
        let ctx = &draw_params.context;
        let path = Path::new(ctx, FillMode::Winding);
        path.add_rectangle(ctx, 0.0, 0.0, draw_params.area_width, draw_params.area_height);
        path.end(ctx);
        let brush = match self.color {
            Color::Red => Brush::Solid(SolidBrush { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }),
            Color::Green => Brush::Solid(SolidBrush { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }),
            Color::Blue => Brush::Solid(SolidBrush { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }),
        };
        ctx.fill(&path, &brush);
    }

    fn mouse_event(&mut self, area: &Area, evt: &AreaMouseEvent) {
        if evt.down > 0 {
            match self.color {
                Color::Red => self.color = Color::Green,
                Color::Green => self.color = Color::Blue,
                Color::Blue => self.color = Color::Red,
            }
            self.raise_changed(area);
        }
    }
}

fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Interactive Canvas Example", 200, 200, WindowType::NoMenubar);

    let mut hbox = HorizontalBox::new(&ui);

    let mut color_canvas = ColorCanvas::new(Color::Red);
    let c = Rc::new(RefCell::new(color_canvas));
    let area = Area::new(&ui, c.clone());

    // Demonstrate changing the color externally after the canvas has been created. This wouldn't
    // be possible if the AreaHandler was a Box rather than a Rc<RefCell>
    c.borrow_mut().color = Color::Blue;

    // Hook up an event handler that redraws the box when the color changes.
    let ui_clone = ui.clone();
    c.borrow_mut().on_changed = Some(Box::new(move |c, area| { area.queue_redraw_all(&ui_clone); }));

    hbox.append(&ui, area, LayoutStrategy::Stretchy);

    win.set_child(&ui, hbox);
    win.show(&ui);
    ui.main();
}

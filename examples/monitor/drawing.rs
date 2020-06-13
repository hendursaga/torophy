use torophy::{Space, Shape};
use rand::{Rng, SeedableRng, rngs::StdRng};


use super::renderer::{Renderer, Color};

pub fn draw_space(renderer: &Renderer, target: &mut glium::Frame, space: &Space) {
    let mut rng = StdRng::seed_from_u64(1);
    for body in space.bodies() {
        let position = body.position();
        let color = Color::rgb(rng.gen_range(0.2, 0.8), rng.gen_range(0.2, 0.8), rng.gen_range(0.2, 0.8));
        match body.shape() {
            Shape::None => {
                renderer.stroke_circle(target, (position.x, position.y), 2.0, 4, color);
            },
            Shape::Circle(radius) => {
                let points = (radius * 2.0) as usize;
                renderer.stroke_circle(target, (position.x, position.y), *radius, points, color);
            },
        }
    }
}


use imgui::{Window, Condition, im_str};

pub fn draw_ui(ui: &mut imgui::Ui, space: &Space) {
    Window::new(im_str!("Inspector"))
        .position([5.0, 5.0], Condition::FirstUseEver)
        .size([200.0, 0.0], Condition::FirstUseEver)
        .collapsed(true, Condition::FirstUseEver)
        .build(&ui, || {
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!("Bodies: {}", space.bodies().len()));
            ui.text(format!("Mouse position: ({:.0},{:.0})", mouse_pos[0], mouse_pos[1]));
        });
}


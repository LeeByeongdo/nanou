use nannou::prelude::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const DAMPENING: f32 = 0.99;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    current: Vec<Vec<f32>>,
    previous: Vec<Vec<f32>>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model {
        current: vec![vec![0.0; HEIGHT]; WIDTH],
        previous: vec![vec![0.0; HEIGHT]; WIDTH],
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    let win = app.window_rect();
    let x = ((app.mouse.x - win.left()) / win.w() * WIDTH as f32) as usize;
    let y = ((app.mouse.y - win.top()) / win.h() * HEIGHT as f32) as usize;

    if x > 0 && x < WIDTH - 1 && y > 0 && y < HEIGHT - 1 {
        model.previous[x][y] = 2000.0;
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 1..WIDTH - 1 {
        for j in 1..HEIGHT - 1 {
            model.current[i][j] = (model.previous[i - 1][j]
                + model.previous[i + 1][j]
                + model.previous[i][j - 1]
                + model.previous[i][j + 1])
                / 2.0
                - model.current[i][j];
            model.current[i][j] *= DAMPENING;
        }
    }

    std::mem::swap(&mut model.previous, &mut model.current);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win = app.window_rect();
    let w = win.w() / WIDTH as f32;
    let h = win.h() / HEIGHT as f32;

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = win.left() + i as f32 * w;
            let y = win.top() - j as f32 * h;
            let c = model.current[i][j];
            let color = nannou::color::gray(c.min(1.0).max(0.0));
            draw.rect().x_y(x + w * 0.5, y - h * 0.5).w_h(w, h).color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

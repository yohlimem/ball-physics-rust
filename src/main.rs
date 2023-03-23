mod ball;
use crate::ball::Ball;
use nannou::prelude::*;

const NUM_BALLS: u32 = 10;
static GRAVITY: f32 = -9.8;

struct Model{
    _window: window::Id,
    ball_array: Vec<Ball>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut ball_array = vec![];
    for i in 0..NUM_BALLS {
        ball_array.push(Ball {
            pos: vec2((i * 40) as f32, 0.0),
            radius: 20.0,
            mass: 10.0,
            velocity: vec2(0.0,0.0),
            acceleration: vec2(0.0,0.0),
            id: i,
        });
    }

    Model {
        _window,
        ball_array,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for ball in 0..model.ball_array.len() - 1 {
        model.ball_array[ball].update(&mut model.ball_array[ball + 1]);
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    Ball::draw_balls(&model.ball_array, &draw);

    draw.to_frame(app, &frame).unwrap();
}




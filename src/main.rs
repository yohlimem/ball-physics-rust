mod ball;

use crate::ball::Ball;
use nannou::prelude::*;
use rand::Rng;

const NUM_BALLS: u32 = 10;
static GRAVITY: f32 = -9.8;

struct Model{
    _window: window::Id,
    ball_array: Vec<Ball>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .size(800, 450)
        .run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut ball_array = vec![];
    for i in 0..NUM_BALLS {
        ball_array.push(Ball {
            pos: vec2(rand::thread_rng().gen_range(0..100) as f32, (i * 40) as f32),
            radius: 20.0,
            mass: 10.0,
            velocity: vec2(0.0,0.0),
            acceleration: vec2(0.0,0.0),
            bouncy: 1.0
        });
    }

    Model {
        _window,
        ball_array,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for ball in 0..model.ball_array.len() {
        model.ball_array[ball].update();
        let circles_collision = model.ball_array[ball].clone().check_collision(&model.ball_array.clone());
        if circles_collision.0 {
            // while model.ball_array[ball].clone().check_collision(&model.ball_array.clone()).0 {
            //     let go_dir = model.ball_array[ball].clone().check_collision(&model.ball_array.clone()).2;
            //     model.ball_array[ball].pos -= go_dir;
            // }
            model.ball_array[ball].velocity = circles_collision.1;
        }
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    Ball::draw_balls(&model.ball_array, &draw);

    draw.to_frame(app, &frame).unwrap();
}




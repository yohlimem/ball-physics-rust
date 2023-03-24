use std::ops::{Add, Div, Mul};
use nannou::prelude::*;
use crate::{GRAVITY};

#[derive(Copy, Clone)]
pub struct Ball{

    pub pos: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub bouncy: f32,

}

impl Ball {


    pub fn draw_balls(ball_array: &Vec<Ball>, draw: &Draw) {
        for i in ball_array {
            draw.ellipse().xy(i.pos).radius(i.radius).color(BLACK);
        }
    }

    pub fn  update(&mut self) {
        self.update_movement();
        self.add_gravity();
        let border_collision = self.bound_col(800.0, 450.0);
        if border_collision.0 {
            while self.bound_col(800.0, 450.0).0 {
                self.pos -= self.bound_col(800.0, 450.0).1;
            }
            if border_collision.1.x.abs() == 1.0 {
                self.velocity.x *= -1.0 * 0.99;
            } else if border_collision.1.y.abs() == 1.0 {
                self.velocity.y *= -1.0 * 0.99;
            }
            // self.velocity *= -border_collision.1.abs().add(vec2(1.0,1.0)).normalize() * 0.89;
        }
    }


}

impl Ball {
    pub fn bound_col(&self, width: f32, height: f32,) -> (bool, Vec2){
        let right = width/2.0 - self.radius;
        let left = -width/2.0 + self.radius;
        let top = height/2.0 - self.radius;
        let bottom = -height/2.0 + self.radius;

        if self.pos.x < left {
            return (true, vec2(-1.0,0.0));
        } else if self.pos.x > right {
            return (true, vec2(1.0,0.0));
        } else if self.pos.y > top {
            return (true, vec2(0.0,1.0));
        } else if self.pos.y < bottom {
            return (true, vec2(0.0,-1.0));
        }
        return (false, vec2(1.0,1.0));
    }

    pub fn check_collision(&mut self, ball_array: &Vec<Ball>) -> (bool, Vec2, Vec2) {
        for i in ball_array{
            if i.pos == self.pos {
                continue;
            }
            let d = ((self.pos.x - i.pos.x) * (self.pos.x - i.pos.x) + (self.pos.y - i.pos.y) * (self.pos.y - i.pos.y)).sqrt();
            if (i.radius - self.radius).abs() <= d && d <= (i.radius + self.radius).abs(){
                // let force_vec: Vec2 = (self.velocity.mul(self.mass) + i.velocity.mul(i.mass)).div(self.mass + i.mass);
                // let force = (force_vec.x * force_vec.x + force_vec.y * force_vec.y).sqrt();
                // let normal_vector = (self.pos - i.pos).normalize().mul(force);
                let dir = (i.pos - self.pos).normalize();
                let dir_out = (self.pos - i.pos).normalize().mul(self.velocity.length());
                return (true, dir_out, dir);
            }
        }
        return (false, vec2(0.0, 0.0), vec2(0.0, 0.0));
    }
}

impl Ball {
    pub fn update_movement(&mut self) {
        self.velocity += self.acceleration * 0.1;
        self.pos += self.velocity * 0.1;
    }

    fn add_gravity(&mut self) {
        self.velocity.y += GRAVITY / 10.0;
    }
}
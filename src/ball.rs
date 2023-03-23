use nannou::prelude::*;
use crate::{GRAVITY, Model};


pub struct Ball{

    pub pos: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,

}

impl Ball {


    pub fn draw_balls(ball_array: &Vec<Ball>, draw: &Draw) {
        for i in ball_array {
            draw.ellipse().xy(i.pos).radius(i.radius).color(BLACK);
        }
    }

    pub fn update(&mut self) {
        self.update_movement();
        self.add_gravity();
        let collision_normal = self.bound_col(600.0, 400.0);
        if collision_normal.0 {
            self.velocity *= -collision_normal.1.abs();
        }

    }


}

impl Ball {
    pub fn bound_col(&mut self, width: f32, height: f32,) -> (bool, Vec2){
        let right = width/2.0 - self.radius;
        let left = -width/2.0 - self.radius;
        let top = height/2.0 - self.radius;
        let bottom = -height/2.0 - self.radius;

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

    pub fn check_collision(&mut self, ball_array: &mut Vec<Ball>) -> bool {
        for i in ball_array{
            if i.pos == self.pos {
                continue;
            }
            let d = ((self.pos.x - i.pos.x) * (self.pos.x - i.pos.x) + (self.pos.y - i.pos.y) * (self.pos.y - i.pos.y)).sqrt();
            if (i.radius - self.radius).abs() <= d && d <= (i.radius + self.radius).abs(){
                return true;
            }
        }
        return false;
    }
}

impl Ball {
    pub fn update_movement(&mut self) {
        self.velocity += self.acceleration;
        self.pos += self.velocity;
    }

    fn add_gravity(&mut self) {
        self.velocity.y += GRAVITY;
    }
}
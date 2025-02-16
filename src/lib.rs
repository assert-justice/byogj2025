use std::thread::ThreadId;

use macroquad::prelude::*;
use gamepads::Gamepads;

#[derive(Default)]
struct Ship{
    pos: Vec2,
    velocity: Vec2,
    angle: f32,
}

pub struct Game{
    pub is_running: bool,
    ship: Ship,
    render_target: RenderTarget,
    gamepads: Gamepads,
}

impl Game{
    pub fn new() -> Self{
        let render_target = render_target(640, 360);
        render_target.texture.set_filter(FilterMode::Nearest);

        Self{
            is_running: true,
            ship: Ship{..Default::default()},
            render_target,
            gamepads: Gamepads::new(),
        }
    }
    pub fn update(&mut self, dt: f32){
        // handle input
        if is_key_pressed(KeyCode::Escape){self.is_running = false;}
        let accel = 0.01;
        let mut dv = vec2(0.0, 0.0);
        self.gamepads.poll();
        for gamepad in self.gamepads.all(){
            dv.x = gamepad.left_stick_x();
            dv.y = gamepad.left_stick_y();
        }
        let turn_speed = 180.;
        if is_key_down(KeyCode::Q) {self.ship.angle -= turn_speed * dt;}
        if is_key_down(KeyCode::E) {self.ship.angle += turn_speed * dt;}
        if is_key_down(KeyCode::S) {dv.y += 1.;}
        if is_key_down(KeyCode::W) {dv.y += -1.;}
        if is_key_down(KeyCode::A) {dv.x += -1.;}
        if is_key_down(KeyCode::D) {dv.x += 1.;}
        dv = dv.normalize_or_zero() * accel;
        let dvx = dv.x;
        let dvy = dv.y;
        let angle = self.ship.angle.to_radians();
        dv.x = angle.cos() * dvx + angle.sin() * dvy;
        dv.y = -angle.sin() * dvx + angle.cos() * dvy;
        self.ship.velocity += dv;
        self.ship.pos += self.ship.velocity * dt;
    }
    pub fn draw(&self){
        // set camera
        set_camera(&Camera2D {
            zoom: vec2(1., 640.0 / 360.0),
            rotation: self.ship.angle,
            target: self.ship.pos,
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });
        clear_background(BLACK);
        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(self.ship.pos.x, self.ship.pos.y, 0.01, YELLOW);

        // draw render target
        set_default_camera();
        draw_texture_ex(
            &self.render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

    }
}
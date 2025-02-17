use macroquad::prelude::*;
use gamepads::{Button, Gamepads};
mod planet;
use planet::Planet;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 180;
const WIDTH_F: f32 = 320.0;
const HEIGHT_F: f32 = 180.0;


#[derive(Default)]
struct Ship{
    pos: Vec2,
    velocity: Vec2,
    angle: f32,
}

pub struct Game{
    pub is_running: bool,
    ship: Ship,
    // render_target: RenderTarget,
    gamepads: Gamepads,
    planets: Vec<Planet>,
    camera: Camera2D,
}

impl Game{
    pub fn new() -> Self{
        let render_target = render_target(WIDTH, HEIGHT);
        render_target.texture.set_filter(FilterMode::Nearest);
        let mut planets = Vec::new();
        planets.push(Planet{pos: vec2(50., 50.), code: 0, tether:"A".to_owned(), color:RED});
        let mut camera = Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: WIDTH_F, h: HEIGHT_F });
        camera.render_target = Some(render_target);

        Self{
            is_running: true,
            ship: Ship{..Default::default()},
            gamepads: Gamepads::new(),
            planets,
            camera,
        }
    }
    pub fn update(&mut self, dt: f32){
        // handle input
        if is_key_pressed(KeyCode::Escape){self.is_running = false;}
        let accel = 1.0;
        let mut dv = vec2(0.0, 0.0);
        let mut d_angle = 0.0;
        self.gamepads.poll();
        for gamepad in self.gamepads.all(){
            dv.x = gamepad.left_stick_x();
            dv.y = -gamepad.left_stick_y();
            if gamepad.is_currently_pressed(Button::FrontLeftUpper) {d_angle -= 1.;}
            if gamepad.is_currently_pressed(Button::FrontRightUpper) {d_angle += 1.;}
        }
        let turn_speed = 180.;
        if is_key_down(KeyCode::Q) {d_angle += 1.;}
        if is_key_down(KeyCode::E) {d_angle -= 1.;}
        if d_angle > 1.{d_angle = 1.;}
        if d_angle < -1.{d_angle = -1.;}
        self.ship.angle += d_angle * turn_speed * dt;
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
        self.camera.target = self.ship.pos;
        self.camera.rotation = self.ship.angle;
    }
    pub fn draw(&self){
        // set camera
        clear_background(RED);
        set_camera(&self.camera);
        clear_background(BLACK);
        // draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        // draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        for p in &self.planets{
            p.draw();
        }
        draw_circle(self.ship.pos.x, self.ship.pos.y, 5.0, YELLOW);

        // draw render target
        set_default_camera();
        let scale_x = screen_width() / WIDTH_F;
        let scale_y = screen_height() / HEIGHT_F;
        let scale = if scale_x > scale_y {scale_y} else {scale_x};
        let width = WIDTH_F * scale; 
        let height = HEIGHT_F * scale;
        let x = screen_width() - width;
        let y = screen_height() - height;
        draw_texture_ex(
            &self.camera.render_target.as_ref().unwrap().texture,
            x/2.,
            y/2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                flip_y: true,
                ..Default::default()
            },
            
        );

    }
}
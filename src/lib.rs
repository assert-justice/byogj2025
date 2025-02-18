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
    // code: u8,
}

struct Input{
    thrust: f32,
    turn: f32,
    code: u8,
    is_repelling: bool,
}

impl Input{
    pub fn new(gamepads: &mut Gamepads) -> Self{
        let mut thrust = 0.0;
        let mut turn = 0.0;
        let mut code = 0;
        let mut is_repelling = false;
        gamepads.poll();
        for gamepad in gamepads.all(){
            turn += gamepad.left_stick_x();
            if gamepad.is_currently_pressed(Button::FrontRightLower) {thrust += 1.0;}
            if gamepad.is_currently_pressed(Button::FrontLeftUpper) {is_repelling = true;}
            if code != 0{continue;}
            // TODO: switch to bitmap stuff
            if gamepad.is_currently_pressed(Button::ActionUp) {code += 1;}
            if gamepad.is_currently_pressed(Button::ActionLeft) {code += 2;}
            if gamepad.is_currently_pressed(Button::ActionDown) {code += 4;}
            if gamepad.is_currently_pressed(Button::ActionRight) {code += 8;}
        }
        if is_key_down(KeyCode::Up){thrust += 1.0;}
        if is_key_down(KeyCode::Left){turn -= 1.0;}
        if is_key_down(KeyCode::Right){turn += 1.0;}
        if code == 0{
            if is_key_down(KeyCode::W) {code += 1;}
            if is_key_down(KeyCode::A) {code += 2;}
            if is_key_down(KeyCode::S) {code += 4;}
            if is_key_down(KeyCode::D) {code += 8;}
        }
        if thrust > 1.0{thrust = 1.0;}
        if turn < -1.0{turn = -1.0;}
        if turn > 1.0{turn = 1.0;}
        Self{
            thrust,
            turn,
            code,
            is_repelling,
        }
    }
}

pub struct Game{
    pub is_running: bool,
    ship: Ship,
    // render_target: RenderTarget,
    gamepads: Gamepads,
    planets: Vec<Planet>,
    camera: Camera2D,
    closest_planet: Option<Planet>,
}

impl Game{
    pub fn new() -> Self{
        let render_target = render_target(WIDTH, HEIGHT);
        render_target.texture.set_filter(FilterMode::Nearest);
        let planets = Vec::new();
        let mut camera = Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: WIDTH_F, h: HEIGHT_F });
        camera.render_target = Some(render_target);

        let mut s = Self{
            is_running: true,
            ship: Ship{..Default::default()},
            gamepads: Gamepads::new(),
            planets,
            camera,
            closest_planet: None,
        };
        s.planet_gen();
        s
    }
    pub fn update(&mut self, dt: f32){
        // handle input
        if is_key_pressed(KeyCode::Escape){self.is_running = false; return;}
        let accel = 1.0;
        let turn_speed = 180.;
        let input = Input::new(&mut self.gamepads);
        let mut dv = vec2(0.0, -input.thrust);
        self.ship.angle += input.turn * turn_speed * dt;
        dv = dv.normalize_or_zero() * accel;
        let dvx = dv.x;
        let dvy = dv.y;
        let angle = self.ship.angle.to_radians();
        dv.x = angle.cos() * dvx + angle.sin() * dvy;
        dv.y = -angle.sin() * dvx + angle.cos() * dvy;
        self.get_closest_planet(input.code);
        if let Some(p) = &self.closest_planet {
            let r = p.pos.distance(self.ship.pos);
            let m = p.mass;
            let mut v = (p.pos - self.ship.pos).normalize() * m / r;
            let max_v = 30.0;
            if v.length() > max_v{
                v = v.normalize() * max_v;
            }
            if input.is_repelling {dv -= v;}
            else{dv += v;}

        }
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
        for p in &self.planets{
            p.draw();
        }
        if let Some(p) = &self.closest_planet {
            draw_line(p.pos.x, p.pos.y, self.ship.pos.x, self.ship.pos.y, 2.0, BLUE);
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
    fn get_closest_planet(&mut self, code: u8){
        let mut res = None;
        let mut dis = f32::INFINITY;
        for p in &self.planets{
            if p.code != code {continue;}
            let d = self.ship.pos.distance_squared(p.pos);
            if d < dis{
                dis = d;
                res = Some(p);
            }
        }
        if let Some(p) = res {
            let p: Planet = p.clone();
            self.closest_planet = Some(p);
        }
        else {self.closest_planet = None;}
    }
    fn planet_gen(&mut self){
        let stride = 150.0;
        let rows = 10;
        let columns = 10;
        let start_x = -150.0*5.0;
        let start_y = -150.0*5.0;
        let lookup = [
            (1, "W", RED),
            (2, "A", GREEN),
            (4, "S", YELLOW),
            (8, "D", BLUE),
        ];
        let mut y = start_y;
        for i in 0..rows{
            let mut x = start_x;
            for f in 0..columns{
                let idx = i % 2 + (f % 2) * 2;
                let (code, tether, color) = lookup[idx];
                self.planets.push(Planet{pos: vec2(x,y), code, tether:tether.to_string(), color, mass:100.0});
                x += stride;
            }
            y += stride;
        }
    }
}
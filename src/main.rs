use macroquad::prelude::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;
const WIDTH_F: f32 = 640.;
const HEIGHT_F: f32 = 360.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(WIDTH, HEIGHT);
    render_target.texture.set_filter(FilterMode::Nearest);
    let mut pos = vec2(0., 0.);
    let mut velocity = vec2(0., 0.);
    let mut angle = 0.;
    loop {
        // clear_background(LIGHTGRAY);

        // Render some primitives in camera space

        set_camera(&Camera2D {
            zoom: vec2(1., WIDTH_F / HEIGHT_F),
            rotation: angle,
            target: pos,
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        clear_background(BLACK);
        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(pos.x, pos.y, 0.01, YELLOW);
        // draw_circle(0.0,0.0,0.01, WHITE);

        // Back to screen space, render some text

        set_default_camera();
        // draw_text("HELLO", 30.0, 200.0, 30.0, BLACK);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        // draw_texture(&render_target.texture, 0., 0., WHITE);
        let dt = get_frame_time();
        // let speed = 1.;
        let accel = 0.01;
        let mut dv = vec2(0.0, 0.0);
        let turn_speed = 180.;
        if is_key_pressed(KeyCode::Escape){break;}
        if is_key_down(KeyCode::Q) {angle -= turn_speed * dt;}
        if is_key_down(KeyCode::E) {angle += turn_speed * dt;}
        if is_key_down(KeyCode::S) {dv.y += 1.;}
        if is_key_down(KeyCode::W) {dv.y += -1.;}
        if is_key_down(KeyCode::A) {dv.x += -1.;}
        if is_key_down(KeyCode::D) {dv.x += 1.;}
        velocity += dv.normalize_or_zero() * accel;
        pos += velocity * dt;

        next_frame().await
    }
}
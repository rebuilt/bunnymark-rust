use raylib::prelude::*;

const MAX_BUNNIES: usize = 300_000;
const MAX_BATCH_ELEMENTS: usize = 8192;

fn main() {
    let screen_width = 1280;
    let screen_height = 720;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("raylib [textures] example - bunnymark")
        .build();

    let tex = rl.load_texture(&thread, "resources/raybunny.png").expect("failed to load texture");
    let tex_w = tex.width();
    let tex_h = tex.height();

    let mut positions: Vec<Vector2> = Vec::with_capacity(MAX_BUNNIES);
    let mut speeds: Vec<Vector2> = Vec::with_capacity(MAX_BUNNIES);
    let mut colors: Vec<Color> = Vec::with_capacity(MAX_BUNNIES);

    let mut paused = false;

    while !rl.window_should_close() {
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            for _ in 0..100 {
                if positions.len() < MAX_BUNNIES {
                    let mouse_pos = rl.get_mouse_position();
                    positions.push(mouse_pos);
                    speeds.push(Vector2::new(
                        rl.get_random_value::<i32>(-250..=250) as f32,
                        rl.get_random_value::<i32>(-250..=250) as f32,
                    ));
                    colors.push(Color::new(
                        rl.get_random_value::<i32>(50..=240) as u8,
                        rl.get_random_value::<i32>(80..=240) as u8,
                        rl.get_random_value::<i32>(100..=240) as u8,
                        255,
                    ));
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            paused = !paused;
        }

        if !paused {
            let dt = rl.get_frame_time();
            let screen_w = rl.get_screen_width() as f32;
            let screen_h = rl.get_screen_height() as f32;

            for (position, speed) in positions.iter_mut().zip(speeds.iter_mut()) {
                position.x += speed.x * dt;
                position.y += speed.y * dt;

                if (position.x + tex_w as f32 / 2.0) > screen_w
                    || (position.x + tex_w as f32 / 2.0) < 0.0
                {
                    speed.x *= -1.0;
                }
                if (position.y + tex_h as f32 / 2.0) > screen_h
                    || (position.y + tex_h as f32 / 2.0 - 40.0) < 0.0
                {
                    speed.y *= -1.0;
                }
            }
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::RAYWHITE);

            for (position, color) in positions.iter().zip(colors.iter()) {
                d.draw_texture(&tex, position.x as i32, position.y as i32, *color);
            }

            d.draw_rectangle(0, 0, screen_width, 40, Color::BLACK);
            d.draw_text(
                &format!("bunnies: {}", positions.len()),
                120,
                10,
                20,
                Color::GREEN,
            );
            d.draw_text(
                &format!(
                    "batched draw calls: {}",
                    1 + positions.len() / MAX_BATCH_ELEMENTS
                ),
                320,
                10,
                20,
                Color::MAROON,
            );

            d.draw_fps(10, 10);
        }
    }
}

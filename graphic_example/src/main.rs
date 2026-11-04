use macroquad::prelude::*;

struct Shape {
    size: f32, speed: f32, x: f32, y: f32, color: Color,
}

#[macroquad::main("My game")]
async fn main() {

    const MOVEMENT_SPEED: f32 = 200.0;
    let delta_time = get_frame_time();
    let smooth_fac = MOVEMENT_SPEED* delta_time;

    let color = Color {
        r: (rand::gen_range(0.0, 1.0)),
        g: (rand::gen_range(0.0, 1.0)),
        b: (rand::gen_range(0.0, 1.0)),
        a: (1.0),
    };

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0, speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: color
    };

    rand::srand(miniquad::date::now() as u64);

    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) { circle.x+= smooth_fac; }
        if is_key_down(KeyCode::Left) { circle.x-= smooth_fac; }
        if is_key_down(KeyCode::Down) { circle.y+= smooth_fac; }
        if is_key_down(KeyCode::Up) { circle.y-= smooth_fac; }

        circle.x = clamp(circle.x, 0.0 + 16.0, screen_width() - 16.0);
        circle.y = clamp(circle.y, 0.0 + 16.0, screen_height() -16.0);

        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);

        let color_sqr = Color {
            r: (rand::gen_range(0.0, 1.0)),
            g: (rand::gen_range(0.0, 1.0)),
            b: (rand::gen_range(0.0, 1.0)),
            a: (1.0),
        };

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: color_sqr
            })
        }

        for squre in &mut squares {
            squre.y += squre.speed * delta_time;
        }

        // remove squares
        squares.retain(|squre| squre.y < screen_height() + squre.size);

        for squre in &squares {
            draw_rectangle(
                squre.x - squre.size / 2.0,
                squre.y - squre.size / 2.0,
                squre.size, squre.size, 
                squre.color
            );
        }
        next_frame().await;
    }
}
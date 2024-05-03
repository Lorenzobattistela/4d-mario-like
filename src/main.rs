use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::sys::KeyCode;
use std::time::Duration;

// TODO refactor animation to use PLAYER
// Refactor movement keypress logic

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;

    canvas.present();

    Ok(())
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn animate(sprite: &mut Rect, position: &mut Point, direction: Direction) {
    let step = 5;
    let expected_y_value = match direction {
        Direction::Down => 0,
        Direction::Left => 36,
        Direction::Right => 72,
        Direction::Up => 108,
    };

    match direction {
        Direction::Down => position.y += step,
        Direction::Left => position.x -= step,
        Direction::Right => position.x += step,
        Direction::Up => position.y -= step,
    }

    let x = sprite.x();
    let y = sprite.y();

    if y != expected_y_value {
        sprite.set_y(expected_y_value)
    }

    match x {
        26 => sprite.set_x(0),
        0 => sprite.set_x(52),
        52 => sprite.set_x(26),
        _ => sprite.set_x(26),
    }
}

// assets/bardo image is 312px by 288px. We have approx 12 cols and 8 rows. Each sprite occupies
// 26px by 36 approx
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("4d mario", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut position = Point::new(0, 0);
    // width = 26, height = 26
    // x, y
    let mut sprite = Rect::new(26, 0, 26, 36);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => animate(&mut sprite, &mut position, Direction::Left),

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => animate(&mut sprite, &mut position, Direction::Right),

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => animate(&mut sprite, &mut position, Direction::Up),

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => animate(&mut sprite, &mut position, Direction::Down),

                _ => {}
            }
        }
        // update

        // render
        render(
            &mut canvas,
            Color::RGB(130, 130, 130),
            &texture,
            position,
            sprite,
        )?;

        // this is telling us that it will run on 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}

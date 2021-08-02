#![feature(allocator_api)]
#![feature(in_band_lifetimes)]

mod data_source;
mod row_manager;

extern crate sdl2;

use crate::data_source::HomeData;
use crate::row_manager::RowManager;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

static SCREEN_WIDTH: u32 = 1600;
static SCREEN_HEIGHT: u32 = 1000;
static BACKGROUND_COLOR: Color = Color::RGBA(51, 51, 255, 255);
static FONT_POINT_SIZE: u16 = 128;
static WINDOW_TITLE: &str = "dss thca";

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let home_data = HomeData::request_home_data();

    let window = video_subsystem
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font("fonts/FiraSans-Bold.ttf", FONT_POINT_SIZE)?;
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let mut row_manager = RowManager::new();

    row_manager.create_row_and_draw_text(&mut canvas, &font, "Row 1", &texture_creator);

    for _ in 0..8 {
        row_manager.add_image_to_row(1, &home_data, &texture_creator);
    }

    row_manager.create_row_and_draw_text(&mut canvas, &font, "Row 2", &texture_creator);

    for _ in 0..7 {
        row_manager.add_image_to_row(2, &home_data, &texture_creator);
    }

    row_manager.create_row_and_draw_text(&mut canvas, &font, "Row 3", &texture_creator);

    for _ in 0..30 {
        row_manager.add_image_to_row(3, &home_data, &texture_creator);
    }

    row_manager.create_row_and_draw_text(&mut canvas, &font, "Row 4", &texture_creator);

    for _ in 0..1 {
        row_manager.add_image_to_row(4, &home_data, &texture_creator);
    }

    row_manager.create_row_and_draw_text(&mut canvas, &font, "Row 5", &texture_creator);

    for _ in 0..3 {
        row_manager.add_image_to_row(5, &home_data, &texture_creator);
    }

    row_manager.redraw_all(&mut canvas);
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Up
                        || keycode == Keycode::Down
                        || keycode == Keycode::Left
                        || keycode == Keycode::Right
                    {
                        // Redraw screen if a new image is selected
                        canvas.clear();
                        row_manager.move_image_selection(keycode);
                        row_manager.redraw_all(&mut canvas);
                        canvas.present();
                    }
                    if keycode == Keycode::Escape {
                        break 'running;
                    }
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

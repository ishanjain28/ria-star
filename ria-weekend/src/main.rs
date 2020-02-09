#![feature(impl_trait_in_bindings)]

mod camera;
mod demo;
mod demos;
mod hitable;
mod render;
mod shapes;
mod types;

use demo::Demo;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::PixelFormatEnum,
};

const NUM_SAMPLES: u8 = 10;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;

    let (mut width, mut height): (usize, usize) = (1200, 600);

    let window = video_subsys
        .window("Ray tracing in a weekend", width as u32, height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .build()
        .map_err(|e| e.to_string())?;

    // RGBA framebuffer
    let mut buffer = vec![0; height * width * 4];

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(PixelFormatEnum::BGR888, width as u32, height as u32)
        .map_err(|e| e.to_string())?;

    //println!("{:?} {:?} {:?}", texture.query(), texture.color_mod(), texture.alpha_mod());

    let active_demo: impl Demo = demos::SurfaceNormalSphere;
    // TODO: Should update when window is unfocus since the project window retains
    // data from overlapped window
    // TODO: Maybe consider using condition variable to make loop {} not run at full
    // speed at all times pinning a core at 100%
    let mut should_update = true;
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(()),
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => active_demo.save_as_ppm(&buffer, width, height),
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    width = w as usize;
                    height = h as usize;
                    buffer.resize(width * height * 4, 0);
                    texture = texture_creator
                        .create_texture_static(PixelFormatEnum::BGR888, width as u32, height as u32)
                        .expect("error in resizing texture");
                    should_update = true;
                }
                _ => {}
            };
        }
        if should_update {
            active_demo.render(&mut buffer, width, height, NUM_SAMPLES);
            texture.update(None, &buffer, width * 4);
            canvas.copy(&texture, None, None);
            canvas.present();
            should_update = false;
        }
    }
}
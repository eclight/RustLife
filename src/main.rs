extern crate sdl2;
extern crate rand;

mod world;
mod color;
mod cell_state;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use world::{CellIndex, World};

const MODE_NORMAL: i32 = 0;
const MODE_COLOR: i32 = 1;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx.window("Life", 800, 600).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err),
    };


    let mut renderer = match window.into_canvas().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err),
    };

    const SIDE: u32 = 4;
    let mut events = ctx.event_pump().unwrap();
    let mut timer = ctx.timer().unwrap();
    let mut last_time = timer.ticks();
    let mut world = World::new(200, 150);
    let mut draw_mode = 0;
    let back_color = sdl2::pixels::Color::RGB(0, 0, 0);

    world.fill_random();

    'event: loop {
        let time = timer.ticks();
        let delta = time - last_time;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event,
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    world.fill_random();
                }   
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    draw_mode = (draw_mode + 1) % 2;
                }  
                _ => continue,
            }
        }

        if delta > 50 {
            last_time = time;

            renderer.set_draw_color(back_color);
            renderer.clear();

            for col in 0..world.cols {
                for row in 0..world.rows {
                    let state = world.get_state(CellIndex {
                        row: row,
                        col: col,
                    });

                    if draw_mode == MODE_NORMAL {
                        if state.alive {
                            renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                            renderer.draw_rect(Rect::new(col * SIDE as i32,
                                                         row * SIDE as i32,
                                                         SIDE,
                                                         SIDE)).expect("Draw operation failed");
                        }
                    } else if draw_mode == MODE_COLOR {
                        if state.alive {
                            renderer.set_draw_color(state.color);
                            renderer.fill_rect(Rect::new(col * SIDE as i32,
                                                                row * SIDE as i32,
                                                                SIDE,
                                                                SIDE)).expect("Draw operation failed");
                        } else {
                            renderer.set_draw_color(state.color);
                            renderer.fill_rect(Rect::new(col * SIDE as i32,
                                                                row * SIDE as i32,
                                                                SIDE,
                                                                SIDE)).expect("Draw operation failed");
                        }
                    }
                }
            }

            world.tick();
            renderer.present();
        }
    }
}

use std::thread;
use std::time::{Duration, Instant};
use raylib::prelude::*;
mod framebuffers;
mod gameoflife;
mod pixel_array;


fn main() {
    let window_width = 100;
    let window_height = 100;
    let framebuffer_width = 100;
    let framebuffer_height = 100;
    let framebuffer_color = Color::GREEN;

    let (mut window, mut raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Game of Life")
        .log_level(TraceLogLevel::LOG_ALL)
        .build();

    let mut framebuffer = framebuffers::FrameBuffer::new(
        framebuffer_width,
        framebuffer_height,
        framebuffer_color
    );

    let starting_points = pixel_array::PIXEL_ART;

    for point in starting_points {
        framebuffer.set_pixel(point.x as i32, point.y as i32, Color::BLACK);
    }

    let target_frame_time = Duration::from_millis(100); // 10 FPS

    while !window.window_should_close() {
        let frame_start = Instant::now();

        gameoflife::game_of_life(
            &mut framebuffer,
            window_width as i32,
            window_height as i32,
            framebuffer_color,
            Color::BLACK,
        );

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // Wait to maintain 60Hz
        let frame_duration = frame_start.elapsed();
        if frame_duration < target_frame_time {
            thread::sleep(target_frame_time - frame_duration);
        }
    }


}


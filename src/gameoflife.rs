use raylib::prelude::*;
use crate::framebuffers;

pub fn game_of_life(
    framebuffer: &mut framebuffers::FrameBuffer,
    width: i32,
    height: i32,
    bg_color: Color,
    living_color: Color
) {
    let image_data = framebuffer.color_buffer.get_image_data();
    let mut birth_points: Vec<(i32, i32)> = Vec::new();
    let mut death_points: Vec<(i32, i32)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let index = (y as u32 * width as u32 + x as u32) as usize;
            let current_color = image_data[index];

            if current_color != bg_color {
                if death(&image_data, x, y, width, height, living_color) {
                    death_points.push((x, y));
                }
            } else {
                if birth(&image_data, x, y, width, height, living_color) {
                    birth_points.push((x, y));
                }
            }
        }
    }

    for (x, y) in death_points {
        framebuffer.set_pixel(x, y, bg_color);
    }

    for (x, y) in birth_points {
        framebuffer.set_pixel(x, y, living_color);
    }
}

fn death(
    data: &[Color],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    living_color: Color
) -> bool {
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut alive_neighbors = 0;

    for &(dx, dy) in &neighbors {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            let idx = (ny as u32 * width as u32 + nx as u32) as usize;
            if data[idx] == living_color {
                alive_neighbors += 1;
            }
        }
    }

    alive_neighbors < 2 || alive_neighbors > 3
}

fn birth(
    data: &[Color],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    living_color: Color
) -> bool {
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut alive_neighbors = 0;

    for &(dx, dy) in &neighbors {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            let idx = (ny as u32 * width as u32 + nx as u32) as usize;
            if data[idx] == living_color {
                alive_neighbors += 1;
            }
        }
    }

    alive_neighbors == 3
}
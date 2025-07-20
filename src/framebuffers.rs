use raylib::prelude::*;

pub struct FrameBuffer{
    pub color_buffer: Image,
    image_width: i32, 
    image_height: i32,
    _color: Color
}

impl FrameBuffer {

    pub fn new(image_width: i32, image_height: i32, color: Color) -> Self {
        let color_buffer = Image::gen_image_color(image_width, image_height, color);
        Self {
            color_buffer,
            image_width,
            image_height,
            _color: color
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.image_width && y >= 0 && y < self.image_height {
            self.color_buffer.draw_pixel(
                x as i32, 
                y as i32, 
                color
            )
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.image_width && y >= 0 && y < self.image_height {
            let image_data = self.color_buffer.get_image_data();
            let idx = (y * self.image_width + x) as usize;
            if idx < image_data.len() {
                image_data[idx]
            } else {
                Color::BLANK
            }
        }
        else {
            Color::BLANK
        }
    }

    pub fn draw_image(&self, output_file_name: &str) {
        self.color_buffer.export_image(output_file_name);
        println!("Image created and saved as '{}'!", output_file_name);
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.image_width, self.image_height, self._color);
    }

    pub fn swap_buffers(
        &mut self, 
        window: &mut RaylibHandle, 
        raylib_thread: &RaylibThread
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);

        }
    }

    
}


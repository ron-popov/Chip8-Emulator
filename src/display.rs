use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::EventPump;

use crate::consts;

pub struct Display {
    canvas: Canvas::<Window>,
    sdl_context: Sdl
}

impl Display {
    // pub fn new() -> Result<(Display, EventPump), String> {

    //     return Ok((Display{canvas: canvas, sdl_context: sdl_context}, event_pump));
    // }

    pub fn draw_sprite(&mut self, sprite_content: Vec<u8>, x_coord: u8, y_coord: u8) -> Result<(), String> {
        debug!("Displaying sprite");
        trace!("Sprite content : {:?}", sprite_content);

        let mut y = (y_coord as i32) % consts::DISPLAY_HEIGHT as i32;
        for sprite in sprite_content {
            let mut value = sprite;
            
            
            for i in 0..8 {
                let x = (x_coord as i32 + (7 - i)) % consts::DISPLAY_WIDTH as i32;

                if (value & 0b1) == 1 {
                    // self.canvas.set_draw_color(Color::BLACK);
                    self.canvas.set_draw_color(Color::WHITE);
                    trace!("Drawing black pixel at ({}, {})", x, y);
                } else {
                    // self.canvas.set_draw_color(Color::WHITE);
                    self.canvas.set_draw_color(Color::BLACK);
                    trace!("Drawing white pixel at ({}, {})", x, y);
                }

                value = value >> 1;

                self.canvas.fill_rect(Rect::new(x, y, 1, 1))?;
                
            }
            y += 1;
        }

        self.canvas.present();

        return Ok(());
    }
}
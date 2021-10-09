use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::consts;

pub struct Display {
    canvas: Canvas::<Window>
}

impl Display {
    pub fn new() -> Result<Display, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("Chip8 Emulator", consts::DISPLAY_WIDTH as u32, consts::DISPLAY_HEIGHT as u32).build().unwrap();

        // Let's create a Canvas which we will use to draw in our Window
        let mut canvas : Canvas<Window> = window.into_canvas()
            .present_vsync() //< this means the screen cannot
            // render faster than your display rate (usually 60Hz or 144Hz)
            .build().unwrap();

        // canvas.set_draw_color(Color::RGB(255, 210, 0));
        // canvas.fill_rect(Rect::new(10, 10, 200, 400))?;
        // canvas.present();

        canvas.clear();

        return Ok(Display{canvas});
    }

    pub fn draw_sprite(&mut self, sprite_content: Vec<u8>, x_coord: u8, y_coord: u8) -> Result<(), String> {
        debug!("Displaying sprite");
        debug!("Sprite content : {:?}", sprite_content);

        
        for sprite in sprite_content {
            let mut value = sprite;
            let mut y = (y_coord as i32) % consts::DISPLAY_HEIGHT as i32;
            
            
            for i in 0..8 {
                let x = (x_coord as i32 + (7 - i)) % consts::DISPLAY_WIDTH as i32;

                if (value & 0b1) == 1 {
                    self.canvas.set_draw_color(Color::BLACK);
                } else {
                    self.canvas.set_draw_color(Color::WHITE);
                }

                value = value >> 1;

                self.canvas.fill_rect(Rect::new(x, y, 1, 1))?;
                
                y += 1;
            }
        }

        self.canvas.present();

        return Ok(());
    }
}
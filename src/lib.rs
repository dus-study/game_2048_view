extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Square {
    x: i32,
    y: i32,
    value: u32,
}

type State = Vec<Square>;

pub struct View {
    canvas: Canvas<Window>,
    lines: Vec<(Point, Point)>,
    bg_color: Color,
    line_color: Color,
    squares: Vec<Square>,
    // font:
}

impl View {
    pub fn new(
        sdl_context: &Sdl,
        bg_color: Color,
        line_color: Color,
        game_size: i32,
        window_size: u32,
    ) -> View {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", window_size, window_size)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let mut lines: Vec<(Point, Point)> = vec![];
        let window_size = window_size as i32;
        for i in 1..game_size {
            lines.push((
                Point::new(0, window_size / game_size * i),
                Point::new(window_size, window_size / game_size * i),
            ));
            lines.push((
                Point::new(window_size / game_size * i, 0),
                Point::new(window_size / game_size * i, window_size),
            ));
        }

        // let ttf_context = sdl2::ttf::init().unwrap();
        // let font = ttf_context.load_font("fonts/DejaVuSansMono-Bold.ttf", 128).unwrap();

        View {
            canvas,
            lines,
            bg_color,
            line_color,
            squares: vec![],
        }
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.line_color);
        for line in self.lines.iter() {
            self.canvas.draw_line(line.0, line.1).unwrap();
        }
        for square in self.squares.iter() {
            let color = if square.value == 2 {
                Color::RGB(0, 0, 255)
            } else if square.value == 4 {
                Color::RGB(255, 0, 0)
            } else if square.value == 8 {
                Color::RGB(0, 255, 0)
            } else {
                Color::RGB(255, 255, 255)
            };
            let x = square.x * 200;
            let y = square.y * 200;

            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(Rect::new(x, y, 200, 200)).unwrap();
            // let surface = font.render(square.value as str).unwrap();
        }
        self.canvas.present();
    }

    pub fn update(&mut self, squares: State) {
        self.squares = squares;
    }
}

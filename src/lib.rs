extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
// use sdl2::ttf::Font;
// use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::Sdl;
use std::format;

// fn t<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

macro_rules! hline {
    ($window_size:expr, $game_size:expr, $i:expr) => {
        (
            Point::new(0, $window_size as i32 / $game_size as i32 * $i as i32),
            Point::new(
                $window_size as i32,
                $window_size as i32 / $game_size as i32 * $i as i32,
            ),
        )
    };
}

macro_rules! vline {
    ($window_size:expr, $game_size:expr, $i:expr) => {
        (
            Point::new($window_size as i32 / $game_size as i32 * $i as i32, 0),
            Point::new(
                $window_size as i32 / $game_size as i32 * $i as i32,
                $window_size as i32,
            ),
        )
    };
}

macro_rules! square {
    ($square:expr, $view:expr) => {
        let x = $square.x * $view.window_size as i32 / $view.game_size as i32;
        let y = $square.y * $view.window_size as i32 / $view.game_size as i32;
        let color = hsv2rgb(&HSV {
            h: $square.value as f64 * 65.0,
            s: 0.3,
            v: 0.8,
        });
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("fonts/DejaVuSansMono-Bold.ttf", 128)
            .unwrap();
        // let font: Font<'ttf_module, 'rwops> = match self.font {
        //     Some(font) => font,
        //     None => self
        //         .ttf_context
        //         .load_font("fonts/DejaVuSansMono-Bold.ttf", 128)
        //         .unwrap(),
        // };
        // self.font = Some(font);
        let surface = font
            .render(format!("{}", $square.value).as_str())
            .blended(Color::RGB(0, 0, 0))
            .unwrap();
        let texture_creator = $view.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        $view.canvas.set_draw_color(color);
        let square = Rect::new(
            x,
            y,
            $view.window_size as u32 / $view.game_size as u32,
            $view.window_size as u32 / $view.game_size as u32,
        );
        $view.canvas.fill_rect(square).unwrap();
        $view.canvas.copy(&texture, None, Some(square)).unwrap();
    };
}

pub struct Square {
    pub x: i32,
    pub y: i32,
    pub value: u32,
}

type Line = (Point, Point);

struct HSV {
    h: f64,
    s: f64,
    v: f64,
}

fn hsv2rgb(hsv: &HSV) -> Color {
    let c = hsv.v * hsv.s;
    let h = (hsv.h % 360.0) / 60.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = hsv.v - c;
    let ra = [c, x, 0.0, 0.0, x, c];
    let ga = [x, c, c, x, 0.0, 0.0];
    let ba = [0.0, 0.0, x, c, c, x];
    let hi = h as usize;
    let mk = |a: [f64; 6]| ((a[hi] + m) * 255.0) as u8;
    Color::RGB(mk(ra), mk(ga), mk(ba))
}
type State = Vec<Square>;

// pub struct View<'ttf_module, 'rwops> {
pub struct View {
    canvas: Canvas<Window>,
    lines: Vec<(Point, Point)>,
    bg_color: Color,
    line_color: Color,
    squares: Vec<Square>,
    window_size: i32,
    game_size: i32,
    // font: Option<Font<'ttf_module, 'rwops>>,
    // ttf_context: Sdl2TtfContext,
}

// impl<'ttf_module, 'rwops> View<'ttf_module, 'rwops> {
impl View {
    pub fn new(
        sdl_context: &Sdl,
        bg_color: Color,
        line_color: Color,
        game_size: i32,
        window_size: u32,
        // ) -> View<'ttf_module, 'rwops> {
    ) -> View {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", window_size, window_size)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let mut lines: Vec<Line> = vec![];
        let window_size = window_size as i32;
        for i in 1..game_size {
            lines.push(hline!(window_size, game_size, i));
            lines.push(vline!(window_size, game_size, i));
        }

        // let ttf_context = sdl2::ttf::init().unwrap();
        // let font = ttf_context
        //     .load_font("fonts/DejaVuSansMono-Bold.ttf", 128)
        //     .unwrap();
        // t(&ttf_context);

        View {
            canvas,
            lines,
            bg_color,
            line_color,
            squares: vec![],
            window_size,
            game_size,
            // font: None,
            // ttf_context: ttf_context,
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
            square!(square, self);
        }

        self.canvas.present();
    }

    pub fn update(&mut self, squares: State) {
        self.squares = squares;
    }
}

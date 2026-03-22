use macroquad::{prelude::*, window};
use miniquad::EventHandler;

#[unsafe(no_mangle)]
extern "C" fn stop_simulation() {
    // TODO: destroy Stage
}

struct Stage;
impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
    }
}

fn main() {
    miniquad::start(window::Conf::default(), || Box::new(Stage))
}

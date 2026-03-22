use macroquad::prelude::*;

static mut RUNNING: bool = true;

#[unsafe(no_mangle)]
pub extern "C" fn stop_simulation() {
    unsafe {
        RUNNING = false;
    }
}

#[macroquad::main("Hello World")]
async fn main() {
    while unsafe { RUNNING } {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}

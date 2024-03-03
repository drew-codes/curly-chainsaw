use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let padding = 20.0;
        let screen_w = screen_width();
        let square_size = screen_w * 0.35;
        let x_position = 0. + padding;
        let y_position = 0. + padding;

        // Clear the background
        clear_background(WHITE);

        // Draw the square
        draw_rectangle(x_position, y_position, square_size, square_size, DARKGRAY);

        // Move to the next frame
        next_frame().await
    }
}
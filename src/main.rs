use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let padding = 20.0;
        let screen_w = screen_width();
        let square_size = (screen_w / 2.0) - (padding * 2.0);
        let x_position = (screen_w - square_size) / 2.0;
        let y_position = x_position; // Assuming you want to center it vertically as well

        // Clear the background
        clear_background(BLUE);

        // Draw the square
        draw_rectangle(x_position, y_position, square_size, square_size, DARKGRAY);

        // Move to the next frame
        next_frame().await
    }
}
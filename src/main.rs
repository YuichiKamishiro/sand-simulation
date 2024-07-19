use macroquad::prelude::*;

const ROWS: i32 = 10;
const COLS: i32 = 20;
const CELL_SIZE: f32 = 32.;
#[derive(Clone, Copy)]
struct Cell {
    x: f32,
    y: f32,
}

fn draw_field() {
    for r in 0..ROWS {
        for c in 0..COLS {
            draw_rectangle_lines(c as f32 * CELL_SIZE, r as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, 1., WHITE);
        }
    }
}

fn draw_sand(cells: &Vec<Cell>) {
    for cell in cells.iter() {
        draw_rectangle(cell.x, cell.y, CELL_SIZE, CELL_SIZE, RED);
    }
}

#[macroquad::main("Main")]
async fn main() {
    let mut cells: Vec<Cell> = vec![];
    let mut time_since_placed: f32 = 0.;
    let mut tick_time: f32 = 0.;
    loop {
        clear_background(BLACK);

        time_since_placed = time_since_placed + get_frame_time();
        tick_time = tick_time + get_frame_time();

        if is_mouse_button_down(MouseButton::Left) && time_since_placed > 0.050 {
            time_since_placed = 0.;
            let pos = mouse_position();
            cells.push(Cell {
                x: pos.0 - pos.0 % CELL_SIZE,
                y: pos.1 - pos.1 % CELL_SIZE,
            });
        }

        if tick_time > 0.1 {
            tick_time = 0.;
            for cell in cells.iter_mut() {
                if cell.y != ((ROWS - 1) * 32) as f32 {
                    cell.y = cell.y + CELL_SIZE;
                }
            }
        }

        draw_field();
        draw_sand(&cells);

        next_frame().await
    }
}

use macroquad::{prelude::*};

const ROWS: i32 = 30;
const COLS: i32 = 50;
const CELL_SIZE: f32 = 25.;

fn draw_field() {
    for r in 0..ROWS {
        for c in 0..COLS {
            draw_rectangle_lines(
                c as f32 * CELL_SIZE,
                r as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                1.,
                WHITE,
            );
        }
    }
}
#[macroquad::main("Main")]
async fn main() {
    let mut cells = vec![vec![false; COLS as usize]; ROWS as usize];
    let mut time_since_placed: f32 = 0.;
    let mut tick_time: f32 = 0.;

    loop {
        clear_background(BLACK);

        time_since_placed = time_since_placed + get_frame_time();
        tick_time = tick_time + get_frame_time();

        if is_mouse_button_down(MouseButton::Left) && time_since_placed > 0.005 {
            time_since_placed = 0.;
            let pos = mouse_position();
            if pos.0 <= COLS as f32 * CELL_SIZE && pos.1 <= ROWS as f32 * CELL_SIZE {
                cells[(pos.1 / CELL_SIZE) as usize][(pos.0 / CELL_SIZE) as usize] = true;
            }
        }

        if tick_time > 0.005 {
            tick_time = 0.;
            let copied = cells.clone();
            for r in 0..ROWS {
                for c in 0..COLS {
                    if copied[r as usize][c as usize] == true && r + 1 != ROWS {

                        if cells[(r + 1) as usize][c as usize] == false {
                            cells[r as usize][c as usize] = false;
                            cells[(r + 1) as usize][c as usize] = true;
                        } else if c + 1 != COLS && cells[(r + 1) as usize][(c + 1) as usize] == false {
                            cells[r as usize][c as usize] = false;
                            cells[(r + 1) as usize][(c + 1) as usize] = true;
                        } else if c - 1 != -1 && cells[(r + 1) as usize][(c - 1) as usize] == false {
                            cells[r as usize][c as usize] = false;
                            cells[(r + 1) as usize][(c - 1) as usize] = true;
                        }

                    }
                }
            }
        }

        for r in 0..ROWS {
            for c in 0..COLS {
                if cells[r as usize][c as usize] == true {
                    draw_rectangle(
                        c as f32 * CELL_SIZE,
                        r as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        YELLOW
                    );
                }
            }
        }

        draw_field();

        next_frame().await
    }
}

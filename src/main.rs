use macroquad::prelude::*;

const ROWS: i32 = 100;
const COLS: i32 = 250;
const CELL_SIZE: f32 = 5.;
const MATRIX_SIZE: i32 = 20;

// randomly gives sand colors
fn get_sand_color() -> Color {
    match rand::gen_range(0, 3) {
        0 => Color::from_rgba(240, 230, 140, 255),
        1 => Color::from_rgba(242, 226, 166, 255),
        _ => Color::from_rgba(247, 235, 195, 255),
    }
}
// draw simple rectangle as borders
fn draw_field() {
    draw_rectangle_lines(
        0.,
        0.,
        COLS as f32 * CELL_SIZE,
        ROWS as f32 * CELL_SIZE,
        10.,
        BLACK,
    )
}

#[derive(Clone, Copy, PartialEq)]
enum Element {
    Nothing,
    Sand(Color),
}

fn is_cursor_inside(pos: (f32, f32), x: i32, y: i32) -> bool {
    if pos.0 + x as f32 <= (COLS as f32 * CELL_SIZE) - 1.
        && pos.1 + y as f32 <= (ROWS as f32 * CELL_SIZE) - 1.
    {
        return true;
    }
    return false;
}

fn click_event(cells: &mut Vec<Vec<Element>>, time_since_placed: &mut f32) {
    if is_mouse_button_down(MouseButton::Left) && *time_since_placed > 0.005 {
        *time_since_placed = 0.;
        let pos = mouse_position();

        for x in 0..MATRIX_SIZE {
            for y in 0..MATRIX_SIZE {
                println!("{y}");
                // checking if cursor pos inside the rect
                if is_cursor_inside(pos, x, y) {
                    let sand_color = get_sand_color();
                    cells[((pos.1 + y as f32) / CELL_SIZE) as usize]
                        [((pos.0 + x as f32) / CELL_SIZE) as usize] = Element::Sand(sand_color);
                }
            }
        }
    }
}

fn tick_event(cells: &mut Vec<Vec<Element>>, tick_time: &mut f32) {
    if *tick_time > 0.005 {
        *tick_time = 0.;
        let copied = cells.clone();
        for r in 0..ROWS {
            for c in 0..COLS {
                // check if this element can go down
                if copied[r as usize][c as usize] != Element::Nothing && r + 1 != ROWS {
                    // check if under element nothing
                    if cells[(r + 1) as usize][c as usize] == Element::Nothing {
                        cells[(r + 1) as usize][c as usize] = cells[r as usize][c as usize];
                        cells[r as usize][c as usize] = Element::Nothing;
                    } else if c + 1 != COLS
                        && cells[(r + 1) as usize][(c + 1) as usize] == Element::Nothing
                        && c - 1 != -1
                        && cells[(r + 1) as usize][(c - 1) as usize] == Element::Nothing
                    {
                        match rand::gen_range(0, 2) {
                            1 => {
                                cells[(r + 1) as usize][(c + 1) as usize] =
                                    cells[r as usize][c as usize];
                                cells[r as usize][c as usize] = Element::Nothing;
                            }
                            _ => {
                                cells[(r + 1) as usize][(c - 1) as usize] =
                                    cells[r as usize][c as usize];
                                cells[r as usize][c as usize] = Element::Nothing;
                            }
                        }
                    } else {
                        if c + 1 != COLS
                            && cells[(r + 1) as usize][(c + 1) as usize] == Element::Nothing
                        {
                            cells[(r + 1) as usize][(c + 1) as usize] =
                                cells[r as usize][c as usize];
                            cells[r as usize][c as usize] = Element::Nothing;
                        } else if c - 1 != -1
                            && cells[(r + 1) as usize][(c - 1) as usize] == Element::Nothing
                        {
                            cells[(r + 1) as usize][(c - 1) as usize] =
                                cells[r as usize][c as usize];
                            cells[r as usize][c as usize] = Element::Nothing;
                        }
                    }
                }
            }
        }
    }
}

#[macroquad::main("Main")]
async fn main() {
    let mut current_element = Element::Sand;
    let mut cells = vec![vec![Element::Nothing; COLS as usize]; ROWS as usize];
    let mut time_since_placed: f32 = 0.;
    let mut tick_time: f32 = 0.;

    loop {
        clear_background(WHITE);

        time_since_placed = time_since_placed + get_frame_time();
        tick_time = tick_time + get_frame_time();

        click_event(&mut cells, &mut time_since_placed);
        tick_event(&mut cells, &mut tick_time);

        for r in 0..ROWS {
            for c in 0..COLS {
                if cells[r as usize][c as usize] != Element::Nothing {
                    let color = match cells[r as usize][c as usize] {
                        Element::Sand(color) => color,
                        _ => Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 0.,
                        },
                    };

                    draw_rectangle(
                        c as f32 * CELL_SIZE,
                        r as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        color,
                    );
                }
            }
        }

        draw_field();

        next_frame().await
    }
}

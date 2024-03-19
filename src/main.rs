use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::*;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy)]
struct Cell {
    value: u32,
    combined: bool,
}

impl Cell {
    fn empty() -> Self {
        Cell {
            value: 0,
            combined: false,
        }
    }

    fn occupied(value: u32) -> Self {
        Cell {
            value,
            combined: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.value == 0
    }

    fn is_occupied(&self) -> bool {
        !self.is_empty()
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { 
        if self.value == 0 {
            write!(f, "Empty")
        } else {
            write!(f, "Occupied(value={}, combined={}", self.value, self.combined)
        }
    }
}

const BOARD_SIZE: f32 = 400.0;
const CELL_DIM: usize = 4;
const CELL_PAD: f32 = 10.0;
const CELL_SIZE: f32 = (BOARD_SIZE - (CELL_PAD * (CELL_DIM + 1) as f32)) / CELL_DIM as f32;
const MAX_SCORE: u32 = 2048;
const COLORS: u32 = MAX_SCORE.ilog2() + 2;

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(500, 500)
    .title("Hello, World")
        .build();
    let mut rng = rand::thread_rng();
    
    let board = Rectangle { x: 50.0, y: 50.0, width: BOARD_SIZE, height: BOARD_SIZE };
    let mut cells = random_cells(&mut rng);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            cells = random_cells(&mut rng);
        }

        let mut needs_combined_reset = false;
        let mut moved = false;
        // Movement
        if rl.is_key_released(KeyboardKey::KEY_RIGHT) {
            moved = true;
            for y in 0..cells.len() {
                for x in (0..(cells[0].len() - 1)).rev() {
                    if cells[y][x].is_empty() {
                        continue;
                    }
                    let mut cell_x = x;
                    while cell_x < cells[0].len() - 1 {
                        if cells[y][cell_x + 1].is_occupied() {
                            if cells[y][cell_x + 1] == cells[y][x] && !cells[y][cell_x + 1].combined && !cells[y][x].combined {
                                cells[y][cell_x + 1] = Cell { value: cells[y][x].value * 2, combined: true};
                                cells[y][x] = Cell::empty();
                                needs_combined_reset = true;
                            }
                            break;
                        } 
                        cell_x += 1;
                    }
                    if cell_x != x {
                        cells[y][cell_x] = cells[y][x];
                        cells[y][x] = Cell::empty();
                    }
                }
            }
        } else if rl.is_key_released(KeyboardKey::KEY_LEFT) {
            moved = true;
            for y in 0..cells.len() {
                for x in 1..cells[0].len() {
                    if cells[y][x].is_empty() {
                        continue;
                    }
                    let mut cell_x = x;
                    while cell_x > 0 {
                        if cells[y][cell_x - 1].is_occupied() {
                            if cells[y][cell_x - 1] == cells[y][x] && !cells[y][cell_x - 1].combined && !cells[y][x].combined {
                                cells[y][cell_x - 1] = Cell { value: cells[y][x].value * 2, combined: true};
                                cells[y][x] = Cell::empty();
                                needs_combined_reset = true;
                            }
                            break;
                        }
                        cell_x -= 1;
                    }
                    if cell_x != x {
                        cells[y][cell_x] = cells[y][x];
                        cells[y][x] = Cell::empty();
                    }
                }
            }
        } else if rl.is_key_released(KeyboardKey::KEY_DOWN) {
            moved = true;
            for y in (0..(cells.len() - 1)).rev() {
                for x in 0..cells[0].len() {
                    if cells[y][x].is_empty() {
                        continue;
                    }
                    let mut cell_y = y;
                    while cell_y < cells.len() - 1 {
                        if cells[cell_y + 1][x].is_occupied() {
                            if cells[cell_y + 1][x] == cells[y][x] && !cells[cell_y + 1][x].combined && !cells[y][x].combined {
                                cells[cell_y + 1][x] = Cell { value: cells[y][x].value * 2, combined: true};
                                cells[y][x] = Cell::empty();
                                needs_combined_reset = true;
                            }
                            break;
                        }
                        cell_y += 1;
                    }
                    if cell_y != y {
                        cells[cell_y][x] = cells[y][x];
                        cells[y][x] = Cell::empty();
                    }
                }
            }
        } else if rl.is_key_released(KeyboardKey::KEY_UP) {
            moved = true;
            for y in 1..cells.len() {
                for x in 0..cells[0].len() {
                    if cells[y][x].is_empty() {
                        continue;
                    }
                    let mut cell_y = y;
                    while cell_y > 0 {
                        if cells[cell_y - 1][x].is_occupied() {
                            if cells[cell_y - 1][x] == cells[y][x] && !cells[cell_y - 1][x].combined && !cells[y][x].combined {
                                cells[cell_y - 1][x] = Cell { value: cells[y][x].value * 2, combined: true};
                                cells[y][x] = Cell::empty();
                                needs_combined_reset = true;
                            }
                            break;
                        }
                        cell_y -= 1;
                    }
                    if cell_y != y {
                        cells[cell_y][x] = cells[y][x];
                        cells[y][x] = Cell::empty();
                    }
                }
            }
        }

        if needs_combined_reset {
            for y in 0..cells.len() {
                for x in 0..cells[y].len() {
                    cells[y][x].combined = false;
                }
            }
        }
        
        if moved { 
            let mut has_empty_cell = false;
            for y in 0..cells.len() {
                for x in 0..cells[y].len() {
                    has_empty_cell |= cells[y][x].is_empty();
                }
            }
            if has_empty_cell {
                loop {
                    let x = rng.gen_range(0..CELL_DIM);
                    let y = rng.gen_range(0..CELL_DIM);
                    if cells[y][x].is_empty() {
                       cells[y][x] = Cell::occupied(2_i32.pow(rng.gen::<u32>() % 2 + 1) as u32);
                       break;
                    }
                }
            } else {
                println!("Game OwOver");
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from_hex("181818").unwrap());

        
        draw_board(d, cells, board);
    }
}

fn draw_board(mut d: RaylibDrawHandle, cells: [[Cell; CELL_DIM]; CELL_DIM], board: Rectangle) {
    d.draw_rectangle_rounded_lines(board, 0.05, 10, 2.0, Color::BEIGE);
    let base_color = Color::color_to_hsv(&Color::from_hex("4444FF").unwrap());
    for y in 0..cells.len() {
        for x in 0..cells[0].len() {
            let cell_x = board.x + CELL_PAD * (x as f32 + 1.0) + x as f32 * CELL_SIZE;
            let cell_y = board.y + CELL_PAD * (y as f32 + 1.0) + y as f32 * CELL_SIZE;
            let mut cell_color = base_color;
            let cell = cells[y][x];
            if !cell.is_empty() {
                cell_color.z = 1.0 - (cell_color.z / COLORS as f32 * cell.value.ilog2() as f32);
                d.draw_rectangle_rounded(
                    Rectangle { 
                        x: cell_x, 
                        y: cell_y, 
                        width: CELL_SIZE, 
                        height: CELL_SIZE 
                    }, 
                    0.1,
                    10,
                    Color::color_from_hsv(cell_color.x, cell_color.y, cell_color.z)
                );
            }
            d.draw_rectangle_rounded_lines(
                Rectangle { 
                    x: cell_x, 
                    y: cell_y, 
                    width: CELL_SIZE, 
                    height: CELL_SIZE 
                }, 
                0.1,
                10,
                2.0,
                Color::BEIGE
            );
            if !cell.is_empty() {
                let text_size = d.get_font_default().measure_text(format!("{}", cell.value).as_str(), 30.0, 2.0);
                d.draw_text(
                    format!("{}", cell.value).as_str(), 
                    (cell_x + CELL_SIZE / 2.0 - text_size.x / 2.0) as i32,
                    (cell_y + CELL_SIZE / 2.0 - text_size.y / 2.0) as i32,
                    30, 
                    Color::BEIGE
                );
            }
        }
    }
}

fn random_cells(rng: &mut ThreadRng) -> [[Cell; CELL_DIM]; CELL_DIM] {
    let mut cells = [[Cell::empty(); CELL_DIM]; CELL_DIM];
    let num_cells = rng.gen_range(2..=3);
    let mut generated = 0;
    while generated < num_cells {
        let x = rng.gen_range(0..CELL_DIM);
        let y = rng.gen_range(0..CELL_DIM);
        if cells[y][x].is_empty() {
            let cell_value = 2_i32.pow(rng.gen::<u32>() % 2 + 1) as u32;
            cells[y][x] = Cell::occupied(cell_value);
            generated += 1;
        }
    }
    cells
}
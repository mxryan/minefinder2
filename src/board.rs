use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug)]
pub struct Board {
    pub mines: i32,
    pub rows: usize,
    pub columns: usize,
    pub flags_placed: i32,
    pub cells: Vec<Cell>,
}

#[derive(Debug, PartialEq)]
pub enum Click {
    Left,
    Right,
}

impl Board {
    pub fn new(mines: i32, rows: usize, columns: usize) -> Board {
        let num_cells = (rows * columns);
        let mut cells: Vec<Cell> = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            cells.push(Cell {
                neighboring_mines: 0,
                state: CellState::Hidden,
                has_mine: false,
            });
        }
        Board { mines, rows, columns, flags_placed: 0, cells }
    }

    pub fn place_mines(&mut self, x_avoid: usize, y_avoid: usize) {
        let index_to_avoid = self.coords_to_index(x_avoid, y_avoid);
        let mut mines_to_place = self.mines;
        let num_tiles = (self.rows * self.columns) as usize;
        let threshold = self.mines as f64 / num_tiles as f64;
        let mut loop_count = 0;

        loop {
            loop_count += 1;
            for i in 0..num_tiles {
                if mines_to_place < 1 {
                    break;
                }
                if self.cells[i].has_mine || i == index_to_avoid {
                    continue;
                }
                let rand = get_random_f64();
                if rand <= threshold {
                    self.cells[i as usize].has_mine = true;
                    mines_to_place -= 1;
                }
            }
            if mines_to_place < 1 {
                break;
            }
        }
        log(&format!("number of iterations required was {}", loop_count));
    }

    pub fn set_cells_num_bomb_neighbors(&mut self) {
        for i in 0..self.get_total_number_of_cells() {
            self.cells[i].neighboring_mines = self.count_neighboring_bombs(i);
        }
    }

    pub fn count_neighboring_bombs(&self, i: usize) -> i32 {
        let mut count = 0;
        let neighbor_indices = self.get_neighbor_indices(i);
        for i in neighbor_indices {
            if self.cells[i].has_mine {
                count += 1;
            }
        }
        count
    }

    pub fn coords_to_index(&self, x: usize, y: usize) -> usize {
        (y * self.columns + x) as usize
    }

    pub fn index_to_coords(&self, i: usize) -> (usize, usize) {
        (i % self.columns, i / self.columns)
    }

    pub fn get_total_number_of_cells(&self) -> usize {
        return self.rows * self.columns;
    }

    pub fn update_state(&mut self, x: usize, y: usize, click: Click) {
        let i = self.coords_to_index(x, y);
        match (click, &self.cells[i].state) {
            (Click::Left, CellState::Hidden) => {
                self.cells[i].state = CellState::Revealed;
                if self.cells[i].neighboring_mines == 0 {
                    self.reveal_neighbors(i);
                }
            }

            (Click::Left, CellState::Revealed) => {
                // if the number of neighboring flags == the number on this tile
                //  then reveal all unrevealed neighboring flags
                // count num neighboring flags
                let num_neighboring_flags = self.count_neighboring_flags(i);
                if num_neighboring_flags == self.cells[i].neighboring_mines {
                    self.reveal_neighbors(i);
                }
            }

            (Click::Right, CellState::Hidden) => {
                self.cells[i].state = CellState::Flagged;
                self.flags_placed += 1;
            }

            (Click::Right, CellState::Flagged) => {
                self.cells[i].state = CellState::Hidden;
                self.flags_placed -= 1;
            }

            _ => {

                // do nothing
            }
        }
    }

    pub fn count_neighboring_flags(&self, target_idx: usize) -> i32 {
        let indices_neighbors = self.get_neighbor_indices(target_idx);
        let mut count = 0;
        for i in indices_neighbors {
            if self.cells[i].state == CellState::Flagged {
                count += 1;
            }
        }

        count
    }

    pub fn get_neighbor_indices(&self, i: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let (x, y) = self.index_to_coords(i);

        for offset_x in -1i32..=1i32 {
            let x_to_check = x as i32 + offset_x;
            if x_to_check < 0 || x_to_check >= self.columns as i32 {
                continue;
            }
            for offset_y in -1i32..=1i32 {
                if offset_y == 0 && offset_x == 0 {
                    continue;
                }
                let y_to_check = y as i32 + offset_y;
                if y_to_check < 0 || y_to_check >= self.rows as i32 {
                    continue;
                }
                let i_neighbor = self.coords_to_index(x_to_check as usize,
                                                      y_to_check as usize);
                out.push(i_neighbor)
            }
        }

        out
    }

    pub fn reveal_neighbors(&mut self, index: usize) {
        let neighbor_indices = self.get_neighbor_indices(index);
        for i in neighbor_indices {
            if self.cells[i].state == CellState::Hidden {
                self.cells[i].state = CellState::Revealed;
                if self.cells[i].neighboring_mines == 0 {
                    self.reveal_neighbors(i);
                }
            }
        }
    }

    pub fn game_lost(&self) {}

    pub fn game_won(&self) {}

    pub fn print_js(&self) {
        for y in 0..self.rows {
            let mut s = String::new();
            for x in 0..self.columns {
                let i = self.coords_to_index(x, y);
                s.push(self.cells[i].repr_val());
            }

            log(&s);
        }
    }

    pub fn print_rust(&self) {
        for y in 0..self.rows {
            let mut s = String::new();
            for x in 0..self.columns {
                let i = self.coords_to_index(x, y);
                s.push(self.cells[i].repr_val());
            }
            println!("{}", s);
        }
    }

    pub fn row_as_string(&self, row: usize) -> String {
        let mut s = String::new();
        for x in 0..self.columns {
            let i = self.coords_to_index(x, row);
            s.push(self.cells[i].repr_val());
        }

        s
    }
}

#[derive(Debug, PartialEq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug)]
pub struct Cell {
    pub neighboring_mines: i32,
    pub state: CellState,
    pub has_mine: bool,
}

impl Cell {
    fn e() -> Cell {
        Cell {
            neighboring_mines: 0,
            state: CellState::Hidden,
            has_mine: false,
        }
    }

    fn m() -> Cell {
        Cell {
            neighboring_mines: 0,
            state: CellState::Hidden,
            has_mine: true,
        }
    }

    fn repr_val(&self) -> char {
        if self.has_mine {
            'x'
        } else {
            std::char::from_digit(self.neighboring_mines as u32, 10)
                .expect("Failed to convert digit to string in repr_val()")
        }
    }

    fn reveal(&mut self) {
        if self.state == CellState::Hidden {
            self.state = CellState::Revealed;
        }
    }

    fn flag(&mut self) {
        if self.state == CellState::Hidden {
            self.state = CellState::Flagged
        }
    }
}

fn get_random_buf() -> Result<[u8; 4], getrandom::Error> {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

fn get_random_f64() -> f64 {
    let four_random_bytes = get_random_buf().unwrap();
    let mut bytes_as_u32 = four_random_bytes[0] as u32;
    bytes_as_u32 += (four_random_bytes[1] as u32) << 8;
    bytes_as_u32 += (four_random_bytes[2] as u32) << 16;
    bytes_as_u32 += (four_random_bytes[3] as u32) << 24;
    bytes_as_u32 as f64 / u32::MAX as f64
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Cell};

    #[test]
    fn coords_to_index_conversions() {
        assert_eq!(2 + 2, 4);

        let board = Board::new(40, 16, 16);
        for i in 0..board.get_total_number_of_cells() {
            let (x1, y1) = board.index_to_coords(i);
            let j = board.coords_to_index(x1, y1);
            let (x2, y2) = board.index_to_coords(j);
            assert_eq!(x1, x2);
            assert_eq!(y1, y2);
            assert_eq!(i, j);
        }
    }

    #[test]
    fn test_mine_counts() {
        let mut board = Board {
            mines: 6,
            rows: 5,
            columns: 5,
            flags_placed: 0,
            cells: vec![
                Cell::e(), Cell::e(), Cell::e(), Cell::e(), Cell::m(),
                Cell::m(), Cell::e(), Cell::e(), Cell::e(), Cell::e(),
                Cell::m(), Cell::e(), Cell::e(), Cell::e(), Cell::m(),
                Cell::e(), Cell::e(), Cell::e(), Cell::e(), Cell::m(),
                Cell::e(), Cell::e(), Cell::e(), Cell::e(), Cell::m(),
            ],
        };

        board.set_cells_num_bomb_neighbors();
        board.print_rust();
        assert_eq!("1101x", board.row_as_string(0));
        assert_eq!("x2022", board.row_as_string(1));
        assert_eq!("x202x", board.row_as_string(2));
        assert_eq!("1103x", board.row_as_string(3));
        assert_eq!("0002x", board.row_as_string(4));
    }
}
use rand::prelude::*;

#[derive(Debug)]
pub struct Board {
    pub mines: i32,
    pub rows: i32,
    pub columns: i32,
    pub flags_placed: i32,
    pub cells: Vec<Cell>,
}

#[derive(Debug)]
enum TileState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug)]
pub struct Cell {
    neighboring_mines: i32,
    state: TileState,
    has_mine: bool,
}

impl Board {
    pub fn new(mines: i32, rows: i32, columns: i32) -> Board {
        let num_cells = (rows * columns) as usize;
        let mut cells: Vec<Cell> = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            cells.push(Cell {
                neighboring_mines: 0,
                state: TileState::Hidden,
                has_mine: false,
            });
        }

        Board { mines, rows, columns, flags_placed: 0, cells: cells, }
    }

    /// TODO: make sure the initial square clicked doesn't have a mine
    pub fn place_mines(&mut self) {
        let mut mines_to_place = self.mines;
        let num_tiles = self.rows * self.columns;
        let threshold = self.mines as f64 / num_tiles  as f64;
        loop {
            for i in 0..num_tiles {
                if mines_to_place < 1 {
                    break;
                }

                if self.cells[i as usize].has_mine {
                    continue;
                }

                let mut rng = rand::thread_rng();
                let rand: f64 = rng.gen();

                if rand <= threshold {
                    self.cells[i as usize].has_mine = true;
                    mines_to_place -= 1;
                }
            }

            if mines_to_place < 1 {
                break;
            }
        }
    }

    pub fn get_total_number_of_cells(&self) -> i32 {
        return self.rows * self.columns;
    }
}
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
    pub fn e() -> Cell {
        Cell {
            neighboring_mines: 0,
            state: CellState::Hidden,
            has_mine: false,
        }
    }

    pub fn m() -> Cell {
        Cell {
            neighboring_mines: 0,
            state: CellState::Hidden,
            has_mine: true,
        }
    }

    pub fn repr_val(&self) -> char {
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
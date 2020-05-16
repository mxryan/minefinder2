pub struct Board {
    mines_start: i32,
    mines_remaining: i32,
}

impl Board {
    pub fn new() -> Board {
        Board{mines_start: 5, mines_remaining: 5}
    }

    pub fn place_mines(&mut self) {
        unimplemented!()
    }
}
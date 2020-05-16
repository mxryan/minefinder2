enum TileState {
    Flagged,
    Revealed,
    Hidden
}

struct Tile {
    neighboring_mines: i8,
    state: TileState,
    has_mine: bool,
}
// Conway Game of Life components

use rustica_ecs::prelude::*;

/// Position component - Represents a cell's position in the grid
#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Component for Position {}

/// CellState component - Represents whether a cell is alive or dead
#[derive(Debug, Clone)]
pub struct CellState {
    pub alive: bool,
}

impl Component for CellState {}

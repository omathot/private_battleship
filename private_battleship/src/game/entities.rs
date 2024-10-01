mod ship;
pub use ship::{Direction, Ship, ShipModel};
use std::fmt;

const BOARD_SIZE: usize = 10;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
    ships: Option<Vec<Ship>>,
}

impl Board {
    pub fn init(ships: Vec<Ship>) -> Self {
        let mut outer = Vec::new();
        let mut inner = Vec::new();
        for _ in 0..10 {
            outer.push("O");
            inner.push("O");
        }
        Board {
            cells: vec![vec![Cell::O; BOARD_SIZE]; BOARD_SIZE],
            ships: Some(ships),
        }
    }

    pub fn get_first_available_spawn(&self, ship: &Ship) -> (usize, usize) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == Cell::Empty
                    && self.check_space_ship_len(y, x, ship.len(), Direction::Down)
                {
                    return (x, y);
                }
            }
        }
        (0, 0)
    }

    pub fn check_space_ship_len(
        &self,
        y: usize,
        x: usize,
        ship_len: usize,
        dir: Direction, // takes direction right now but doesn't use it, idea is to check any given direction
    ) -> bool {
        if y + ship_len > self.cells.len() {
            return false;
        }

        for i in y..y + ship_len {
            if self.cells[i][x] != Cell::Empty {
                return false;
            }
        }
        true
    }

    pub fn check_right(&self, ship: &Ship) -> bool {
        let mut x = ship.root_x();
        if x == 9 {
            x = 0;
        } else {
            x += 1;
        }
        if ship.direction() == Direction::Up || ship.direction() == Direction::Down {
            // ship is vertical, check every cell that will be moved horizontally
            for i in ship.root_y()..ship.root_y() + ship.len() {
                if self.cells[i][x] != Cell::Empty {
                    return false;
                }
            }
        } else if self.cells[ship.root_y()][x] != Cell::Empty {
            // ship is horizontal, just check the cell to the right
            return false;
        }
        true
    }

    pub fn check_left(&self, ship: &Ship) -> bool {
        let mut x = ship.root_x();
        if x == 0 {
            x = 9;
        } else {
            x -= 1;
        }
        if ship.direction() == Direction::Up || ship.direction() == Direction::Down {
            // ship is vertical, check every cell that will be moved horizontally
            for i in ship.root_y()..ship.root_y() + ship.len() {
                if self.cells[i][x] != Cell::Empty {
                    return false;
                }
            }
        } else if self.cells[ship.root_y()][x] != Cell::Empty {
            // ship is horizontal, just check the cell to the right
            return false;
        }
        true
    }

    pub fn check_down(&self, ship: &Ship) -> bool {
        let mut y = ship.root_y();
        if y == 9 {
            y = 0;
        } else {
            y += 1;
        }
        if ship.direction() == Direction::Left || ship.direction() == Direction::Right {
            for i in ship.root_x()..ship.root_x() + ship.len() {
                if self.cells[y][i] != Cell::Empty {
                    return false;
                }
            }
        } else if self.cells[y][ship.root_x()] != Cell::Empty {
            return false;
        }
        true
    }

    pub fn check_up(&self, ship: &Ship) -> bool {
        let mut y = ship.root_y();
        if y == 0 {
            y = 9;
        } else {
            y -= 1;
        }
        if ship.direction() == Direction::Left || ship.direction() == Direction::Right {
            for i in ship.root_x()..ship.root_x() + ship.len() {
                if self.cells[y][i] != Cell::Empty {
                    return false;
                }
            }
        } else if self.cells[y][ship.root_x()] != Cell::Empty {
            return false;
        }
        true
    }

    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut outer = Vec::new();
        let mut inner = Vec::new();
        for _ in 0..10 {
            outer.push("O");
            inner.push("O");
        }
        Board {
            cells: vec![vec![Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: None,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.iter() {
            for cell in line {
                let symbol = match cell {
                    Cell::Empty => " 0 ",
                    Cell::Ship => " S ",
                    Cell::X => " X ",
                    Cell::O => " O ",
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Ship,
    X,
    O,
}

pub struct Player {
    name: String,
    ships: Vec<Ship>,
    state: PlayerState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    Placing,
    Waiting,
    Bombing,
}

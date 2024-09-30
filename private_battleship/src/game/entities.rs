use std::fmt;

const BOARD_SIZE: usize = 10;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ship {
    len: usize,
    root: Option<(usize, usize)>,
    direction: Option<Direction>,
    on_board: bool,
}

impl Ship {
    pub fn place(&mut self, board: &mut Board) {
        self.root = Some(board.get_first_available_spawn(self));
        self.direction = Some(Direction::Down);
        self.on_board = true;
        for i in self.root_y()..self.root_y() + self.len {
            board.cells[i][self.root_x()] = Cell::Ship;
        }
    }

    pub fn move_right(&mut self, board: &mut Board) {
        if board.check_right(self) {
            match self.direction {
                Some(Direction::Up) | Some(Direction::Down) => {
                    for i in self.root_y()..self.root_y() + self.len {
                        board.cells[i][self.root_x()] = Cell::Empty;
                    }
                    let (mut x, y) = self.root.unwrap();
                    if x == 9 {
                        x = 0;
                    } else {
                        x += 1;
                    }
                    self.root = Some((x, y));
                    for i in self.root_y()..self.root_y() + self.len {
                        board.cells[i][self.root_x()] = Cell::Ship;
                    }
                }
                Some(Direction::Right) | Some(Direction::Left) => {
                    board.cells[self.root_y()][self.root_x() + self.len] = Cell::Empty;
                    let (mut x, y) = self.root.unwrap();
                    if x == 9 {
                        x = 0;
                    } else {
                        x += 1;
                    }
                    self.root = Some((x, y));
                    board.cells[self.root_y()][x] = Cell::Ship;
                }
                None => {}
            }
        }
    }

    pub fn move_left(&mut self, board: &mut Board) {
        if board.check_left(self) {
            match self.direction {
                Some(Direction::Up) | Some(Direction::Down) => {
                    for i in self.root_y()..self.root_y() + self.len {
                        board.cells[i][self.root_x()] = Cell::Empty;
                    }
                    let (mut x, y) = self.root.unwrap();
                    if x == 0 {
                        x = 9;
                    } else {
                        x -= 1;
                    }
                    self.root = Some((x, y));
                    for i in self.root_y()..self.root_y() + self.len {
                        board.cells[i][self.root_x()] = Cell::Ship;
                    }
                }
                Some(Direction::Right) | Some(Direction::Left) => {
                    board.cells[self.root_y()][self.root_x() + self.len] = Cell::Empty;
                    let (mut x, y) = self.root.unwrap();
                    if x == 0 {
                        // not sure I wanna let you wrap if you're in a direction that can be cut across borders
                        x = 9;
                    } else {
                        x -= 1;
                    }
                    self.root = Some((x, y));
                    board.cells[self.root_y()][x] = Cell::Ship;
                }
                None => {}
            }
        }
    }

    pub fn move_up(&mut self, board: &mut Board) {
        if board.check_up(self) {
            match self.direction {
                Some(Direction::Right) | Some(Direction::Left) => {
                    for i in self.root_x()..self.root_x() + self.len {
                        board.cells[self.root_y()][i] = Cell::Empty;
                    }
                    let (x, mut y) = self.root.unwrap();
                    if y == 0 {
                        y = 9;
                    } else {
                        y -= 1;
                    }
                    self.root = Some((x, y));
                    for i in self.root_x()..self.root_x() + self.len {
                        board.cells[y][i] = Cell::Ship;
                    }
                }
                Some(Direction::Up) | Some(Direction::Down) => {
                    board.cells[self.root_y() + self.len - 1][self.root_x()] = Cell::Empty;
                    println!(
                        "y{} + len {}: {}",
                        self.root_y(),
                        self.len,
                        self.root_y() + self.len
                    );
                    let (x, mut y) = self.root.unwrap();
                    if y == 0 {
                        y = 9;
                    } else {
                        y -= 1;
                    }
                    self.root = Some((x, y));
                    board.cells[y][x] = Cell::Ship;
                }
                None => {}
            }
        }
    }

    pub fn move_down(&mut self, board: &mut Board) {
        if board.check_up(self) {
            match self.direction {
                Some(Direction::Right) | Some(Direction::Left) => {
                    for i in self.root_x()..self.root_x() + self.len {
                        board.cells[self.root_y()][i] = Cell::Empty;
                    }
                    let (x, mut y) = self.root.unwrap();
                    if y == 9 {
                        y = 0;
                    } else {
                        y += 1;
                    }
                    self.root = Some((x, y));
                    for i in self.root_x()..self.root_x() + self.len {
                        board.cells[y][i] = Cell::Ship;
                    }
                }
                Some(Direction::Up) | Some(Direction::Down) => {
                    board.cells[self.root_y()][self.root_x()] = Cell::Empty;
                    println!("x: {}, y: {}", self.root_x(), self.root_y());
                    let (x, mut y) = self.root.unwrap();
                    if y == 9 {
                        y = 0;
                    } else {
                        y += 1;
                    }
                    println!("{y}, + len = {}", y + self.len);
                    self.root = Some((x, y));
                    let mut place_y = y + self.len - 1;
                    if place_y > 9 {
                        place_y -= 9;
                    }
                    board.cells[place_y][x] = Cell::Ship;
                }
                None => {}
            }
        }
    }

    // pub fn rotate(&self, board: &mut Board) {

    // }

    pub fn root_y(&self) -> usize {
        self.root.unwrap().1
    }

    pub fn root_x(&self) -> usize {
        self.root.unwrap().0
    }
}

impl From<ShipModel> for Ship {
    fn from(value: ShipModel) -> Self {
        Ship {
            len: match value {
                ShipModel::Carrier => 5,
                ShipModel::Battleship => 4,
                ShipModel::Cruiser => 3,
                ShipModel::Submarine => 3,
                ShipModel::Destroyer => 2,
            },
            root: None,
            direction: None,
            on_board: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShipModel {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

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
                    && self.check_space_ship_len(y, x, ship.len, Direction::Down)
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
        if ship.direction == Some(Direction::Up) || ship.direction == Some(Direction::Down) {
            // ship is vertical, check every cell that will be moved horizontally
            for i in ship.root_y()..ship.root_y() + ship.len {
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
        if ship.direction == Some(Direction::Up) || ship.direction == Some(Direction::Down) {
            // ship is vertical, check every cell that will be moved horizontally
            for i in ship.root_y()..ship.root_y() + ship.len {
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
        if ship.direction == Some(Direction::Left) || ship.direction == Some(Direction::Right) {
            for i in ship.root_x()..ship.root_x() + ship.len {
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
        if ship.direction == Some(Direction::Left) || ship.direction == Some(Direction::Right) {
            for i in ship.root_x()..ship.root_x() + ship.len {
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
                    Cell::Empty => '0',
                    Cell::Ship => 'S',
                    Cell::X => 'X',
                    Cell::O => 'O',
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

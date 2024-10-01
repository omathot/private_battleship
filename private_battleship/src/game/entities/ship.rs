use super::*;

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

    pub fn rotate(&mut self, board: &mut Board) {
        if let Some(dir) = self.direction {
            let (range, is_vertical) = self.get_range_and_orientation(dir); // fancy system, returns range and bool.
            self.set_cells(board, &range, is_vertical, Cell::Empty);
            println!("{}", board);
            self.direction = self.next_direction();

            let (new_range, new_is_vertical) =
                self.get_range_and_orientation(self.direction.unwrap());
            self.set_cells(board, &new_range, new_is_vertical, Cell::Ship);
        }
    }

    fn get_range_and_orientation(&self, dir: Direction) -> (std::ops::Range<usize>, bool) {
        let (start, is_vertical) = match dir {
            Direction::Up => (
                (self.root_y() + BOARD_SIZE - self.len + 1) % BOARD_SIZE,
                true,
            ),
            Direction::Down => (self.root_y() + 1, true),
            Direction::Left => (
                (self.root_x() + BOARD_SIZE - self.len + 1) % BOARD_SIZE,
                false,
            ),
            Direction::Right => (self.root_x(), false),
        };
        (start..start + self.len, is_vertical)
    }

    fn set_cells(
        &self,
        board: &mut Board,
        range: &std::ops::Range<usize>,
        is_vertical: bool,
        cell_t: Cell,
    ) {
        for i in range.clone() {
            let (y, x) = if is_vertical {
                (i % BOARD_SIZE, self.root_x())
            } else {
                (self.root_y(), i % BOARD_SIZE)
            };
            board.cells[y][x] = cell_t;
        }
    }

    fn next_direction(&self) -> Option<Direction> {
        self.direction.map(|dir| match dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        })
    }

    pub fn root_y(&self) -> usize {
        self.root.unwrap().1
    }

    pub fn root_x(&self) -> usize {
        self.root.unwrap().0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn direction(&self) -> Direction {
        self.direction.unwrap_or(Direction::Down)
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

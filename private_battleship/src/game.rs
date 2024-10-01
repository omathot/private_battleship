mod entities;

use palette::{rgb::Rgb, Srgb};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum UserError {
    #[error("Can't play now, enemy's turn")]
    EnemyTurn,
    #[error("Cannot start game")]
    CannotStartGame,
    #[error("Invalid Input")]
    InvalidInput,
}

pub struct GameSettings {
    colour: Rgb,
}
impl GameSettings {
    pub fn new(colour: Rgb) -> Self {
        GameSettings { colour }
    }
}

#[cfg(test)]
mod game_tests {
    use entities::{Board, Cell, Ship, ShipModel};
    use palette::cast::TryComponentsInto;

    use super::*;

    #[test]
    fn init_board() {
        let board = Board::default();
        println!("{}", board); // successfully prints a 10x10 board. use -- --nocapture to see println! output in tests
        assert_eq!(&vec![vec![Cell::Empty; 10]; 10], board.cells());
    }

    #[test]
    fn place_ship() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        assert_eq!(Cell::Ship, board.cells()[0][0]);
        assert_eq!(Cell::Ship, board.cells()[1][0]);
        assert_eq!(Cell::Ship, board.cells()[2][0]);
        assert_eq!(Cell::Ship, board.cells()[3][0]);
        assert_eq!(Cell::Ship, board.cells()[4][0]);
        println!("{}", board);
    }

    #[test]
    fn move_ship_right() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.move_right(&mut board);
        assert_eq!(Cell::Ship, board.cells()[0][1]);
        assert_eq!(Cell::Ship, board.cells()[1][1]);
        assert_eq!(Cell::Ship, board.cells()[2][1]);
        assert_eq!(Cell::Ship, board.cells()[3][1]);
        assert_eq!(Cell::Ship, board.cells()[4][1]);

        assert_eq!(Cell::Empty, board.cells()[0][0]);
        assert_eq!(Cell::Empty, board.cells()[1][0]);
        assert_eq!(Cell::Empty, board.cells()[2][0]);
        assert_eq!(Cell::Empty, board.cells()[3][0]);
        assert_eq!(Cell::Empty, board.cells()[4][0]);
    }

    #[test]
    fn move_ship_right_then_left() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.move_right(&mut board);
        ship.move_left(&mut board);
        assert_eq!(Cell::Empty, board.cells()[0][9]);
        assert_eq!(Cell::Empty, board.cells()[1][9]);
        assert_eq!(Cell::Empty, board.cells()[2][9]);
        assert_eq!(Cell::Empty, board.cells()[3][9]);
        assert_eq!(Cell::Empty, board.cells()[4][9]);

        assert_eq!(Cell::Ship, board.cells()[0][0]);
        assert_eq!(Cell::Ship, board.cells()[1][0]);
        assert_eq!(Cell::Ship, board.cells()[2][0]);
        assert_eq!(Cell::Ship, board.cells()[3][0]);
        assert_eq!(Cell::Ship, board.cells()[4][0]);
    }

    #[test]
    fn move_ship_down() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.move_down(&mut board);
        println!("{}", board);
        assert_eq!(Cell::Empty, board.cells()[0][0]);

        assert_eq!(Cell::Ship, board.cells()[5][0]);
    }

    #[test]
    fn move_ship_up_wrap_to_bottom() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.move_up(&mut board);
        println!("{}", board);
        assert_eq!(Cell::Empty, board.cells()[4][0]);

        assert_eq!(Cell::Ship, board.cells()[9][0]);
    }

    #[test]
    fn move_ship_left_wrap_to_right() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.move_left(&mut board);
        assert_eq!(Cell::Ship, board.cells()[0][9]);
        assert_eq!(Cell::Ship, board.cells()[1][9]);
        assert_eq!(Cell::Ship, board.cells()[2][9]);
        assert_eq!(Cell::Ship, board.cells()[3][9]);
        assert_eq!(Cell::Ship, board.cells()[4][9]);

        assert_eq!(Cell::Empty, board.cells()[0][0]);
        assert_eq!(Cell::Empty, board.cells()[1][0]);
        assert_eq!(Cell::Empty, board.cells()[2][0]);
        assert_eq!(Cell::Empty, board.cells()[3][0]);
        assert_eq!(Cell::Empty, board.cells()[4][0]);
    }

    #[test]
    fn move_ship_right_wrap_to_left() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        for _ in 0..9 {
            ship.move_right(&mut board);
        }
        ship.move_right(&mut board);

        assert_eq!(Cell::Empty, board.cells()[0][9]);
        assert_eq!(Cell::Empty, board.cells()[1][9]);
        assert_eq!(Cell::Empty, board.cells()[2][9]);
        assert_eq!(Cell::Empty, board.cells()[3][9]);
        assert_eq!(Cell::Empty, board.cells()[4][9]);

        assert_eq!(Cell::Ship, board.cells()[0][0]);
        assert_eq!(Cell::Ship, board.cells()[1][0]);
        assert_eq!(Cell::Ship, board.cells()[2][0]);
        assert_eq!(Cell::Ship, board.cells()[3][0]);
        assert_eq!(Cell::Ship, board.cells()[4][0]);
        println!("{}", board);
    }

    #[test]
    fn rotate_from_zero_zero() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.rotate(&mut board);
        println!("{}", board);
        assert_eq!(Cell::Ship, board.cells()[0][0]);
        assert_eq!(Cell::Ship, board.cells()[0][9]);
        assert_eq!(Cell::Ship, board.cells()[0][8]);
        assert_eq!(Cell::Ship, board.cells()[0][7]);
        assert_eq!(Cell::Ship, board.cells()[0][6]);

        assert_eq!(Cell::Empty, board.cells()[1][0]);
        assert_eq!(Cell::Empty, board.cells()[2][0]);
    }

    #[test]
    fn rotate_from_zero_zero_twice() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.rotate(&mut board);
        ship.rotate(&mut board);
        println!("{}", board);
        assert_eq!(Cell::Ship, board.cells()[0][0]);
        assert_eq!(Cell::Ship, board.cells()[9][0]);
        assert_eq!(Cell::Ship, board.cells()[8][0]);
        assert_eq!(Cell::Ship, board.cells()[7][0]);
        assert_eq!(Cell::Ship, board.cells()[6][0]);

        assert_eq!(Cell::Empty, board.cells()[0][9]);
        assert_eq!(Cell::Empty, board.cells()[0][8]);
    }

    #[test]
    fn rotate_from_zero_zero_thrice() {
        let mut board = Board::default();
        let mut ship = Ship::from(ShipModel::Carrier);
        ship.place(&mut board);
        ship.rotate(&mut board);
        ship.rotate(&mut board);
        ship.rotate(&mut board);
        println!("{}", board);
        assert_eq!(Cell::Ship, board.cells()[0][0]);
        assert_eq!(Cell::Ship, board.cells()[0][1]);
        assert_eq!(Cell::Ship, board.cells()[0][2]);
        assert_eq!(Cell::Ship, board.cells()[0][3]);
        assert_eq!(Cell::Ship, board.cells()[0][4]);

        assert_eq!(Cell::Empty, board.cells()[9][0]);
        assert_eq!(Cell::Empty, board.cells()[8][0]);
    }
}

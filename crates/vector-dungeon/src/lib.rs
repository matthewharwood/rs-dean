use garde::Validate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const GRID_MIN: i32 = -2;
pub const GRID_MAX: i32 = 2;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize, Validate)]
pub struct Coordinate {
    #[garde(range(min = GRID_MIN, max = GRID_MAX))]
    pub x: i32,
    #[garde(range(min = GRID_MIN, max = GRID_MAX))]
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Move {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum MoveError {
    #[error("move must be exactly one cardinal step")]
    NotCardinal,
    #[error("target is outside the dungeon grid")]
    OutOfBounds,
}

pub fn apply_move(position: Coordinate, movement: Move) -> Result<Coordinate, MoveError> {
    if movement.dx.abs().strict_add(movement.dy.abs()) != 1 {
        return Err(MoveError::NotCardinal);
    }

    let target = Coordinate {
        x: position.x.strict_add(movement.dx),
        y: position.y.strict_add(movement.dy),
    };

    if !(GRID_MIN..=GRID_MAX).contains(&target.x) || !(GRID_MIN..=GRID_MAX).contains(&target.y) {
        return Err(MoveError::OutOfBounds);
    }

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Move, apply_move};

    #[test]
    fn cardinal_move_updates_position() {
        let next = apply_move(Coordinate::default(), Move { dx: 1, dy: 0 });
        assert_eq!(next, Ok(Coordinate { x: 1, y: 0 }));
    }
}

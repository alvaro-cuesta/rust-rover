use num::{One, Zero};
use std::borrow::Borrow;
use std::ops::{Add, Sub};

use crate::Direction;

/// A position in a 2d plane
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position<N>(pub N, pub N)
where
    N: Zero + One + Add<Output = N> + Sub<Output = N>;

/// Add two positions
/// 
/// # Examples
/// ```
/// # use rust_rover::Position;
/// assert_eq!(
///     Position(1, 1) + Position(3, 4),
///     Position(4, 5),
/// )
/// ```
impl<N> Add for Position<N>
where
    N: Zero + One + Add<Output = N> + Sub<Output = N>,
{
    type Output = Position<N>;

    fn add(self, Position(x, y): Position<N>) -> Position<N> {
        Position(self.0 + x, self.1 + y)
    }
}

/// Subtract one position from another
/// 
/// # Examples
/// ```
/// # use rust_rover::Position;
/// assert_eq!(
///     Position(3, 4) - Position(1, 1),
///     Position(2, 3),
/// )
/// ```
impl<N> Sub for Position<N>
where
    N: Zero + One + Add<Output = N> + Sub<Output = N>,
{
    type Output = Position<N>;

    fn sub(self, Position(x, y): Position<N>) -> Position<N> {
        Position(self.0 - x, self.1 - y)
    }
}

/// Add a direction to a position
/// 
/// # Examples
/// ```
/// # use rust_rover::{Position, Direction};
/// assert_eq!(
///     Position(1, 1) + Direction::N,
///     Position(1, 2),
/// );
/// 
/// assert_eq!(
///     Position(1, 1) + &Direction::N,
///     Position(1, 2),
/// );
/// 
/// assert_eq!(
///     &Position(1, 1) + Direction::N,
///     Position(1, 2),
/// );
/// 
/// assert_eq!(
///     &Position(1, 1) + &Direction::N,
///     Position(1, 2),
/// );
/// ```
impl<N, D> Add<D> for Position<N>
where
    D: Borrow<Direction>,
    N: Zero + One + Add<Output = N> + Sub<Output = N>,
{
    type Output = Position<N>;

    fn add(self, direction: D) -> Position<N> {
        match direction.borrow() {
            Direction::N => self + Position(Zero::zero(), One::one()),
            Direction::S => self - Position(Zero::zero(), One::one()),
            Direction::E => self + Position(One::one(), Zero::zero()),
            Direction::W => self - Position(One::one(), Zero::zero()),
        }
    }
}

/// Add a direction to a position
/// 
/// # Examples
/// ```
/// # use rust_rover::{Position, Direction};/// 
/// assert_eq!(
///     &Position(1, 1) + Direction::N,
///     Position(1, 2),
/// );
/// 
/// assert_eq!(
///     &Position(1, 1) + &Direction::N,
///     Position(1, 2),
/// );
/// ```
impl<'a, N, D> Add<D> for &'a Position<N>
where
    D: Borrow<Direction>,
    N: Zero + One + Add<Output = N> + Sub<Output = N> + Clone,
{
    type Output = Position<N>;

    fn add(self, direction: D) -> Position<N> {
        self.clone() + direction
    }
}

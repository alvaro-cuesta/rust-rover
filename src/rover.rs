use num::{One, Zero};
use std::ops::{Add, Sub};

use crate::{Direction, Position, Instruction};

/// A rover with a particular position and direction in a plane
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Rover<N>
where
    N: Zero + One + Add<Output = N> + Sub<Output = N> + Copy,
{
    position: Position<N>,
    direction: Direction,
}

impl<N> Rover<N>
where
    N: Zero + One + Add<Output = N> + Sub<Output = N> + Copy,
{
    /// Create a new rover
    /// 
    /// # Example
    /// 
    /// ```
    /// # use rust_rover::{Rover, Position, Direction};
    /// let rover = Rover::new(
    ///     Position(0, 0),
    ///     Direction::N
    /// );
    /// ```
    pub fn new(position: Position<N>, direction: Direction) -> Rover<N> {
        Rover {
            position,
            direction,
        }
    }

    /// Execute an instruction
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rust_rover::{Rover, Instruction, Position, Direction};
    /// let rover = Rover::new(
    ///     Position(0, 0),
    ///     Direction::N
    /// );
    /// 
    /// assert_eq!(
    ///     rover.execute(Instruction::Forward),
    ///     Rover::new(
    ///         Position(0, 1),
    ///         Direction::N
    ///     ),
    /// );
    /// 
    /// assert_eq!(
    ///     rover.execute(Instruction::RotateCW),
    ///     Rover::new(
    ///         Position(0, 0),
    ///         Direction::E
    ///     ),
    /// );
    /// 
    /// assert_eq!(
    ///     rover.execute(Instruction::RotateCCW),
    ///     Rover::new(
    ///         Position(0, 0),
    ///         Direction::W
    ///     ),
    /// );
    /// ```
    pub fn execute(&self, instruction: impl std::borrow::Borrow<Instruction>) -> Rover<N> {
        match instruction.borrow() {
            Instruction::Forward => Rover {
                position: self.position + self.direction,
                direction: self.direction,
            },
            Instruction::RotateCW => Rover {
                position: self.position,
                direction: self.direction.rotate_cw(),
            },
            Instruction::RotateCCW => Rover {
                position: self.position,
                direction: self.direction.rotate_ccw(),
            },
        }
    }

    /// Execute many instructions
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rust_rover::{Rover, Instruction, Position, Direction};
    /// use rust_rover::Instruction::*;
    /// 
    /// let rover = Rover::new(
    ///     Position(0, 0),
    ///     Direction::N
    /// );
    /// 
    /// let instructions = vec![
    ///     Forward,
    ///     RotateCW,
    ///     Forward,
    ///     Forward,
    ///     RotateCCW,
    ///     Forward,
    /// ];
    /// 
    /// assert_eq!(
    ///     rover.execute_many(instructions),
    ///     Rover::new(
    ///         Position(2, 2),
    ///         Direction::N
    ///     ),
    /// );
    /// ```
    pub fn execute_many<I: AsRef<[Instruction]>>(&self, instructions: I) -> Rover<N> {
        instructions
            .as_ref()
            .iter()
            .fold(*self, |rover, instruction| rover.execute(instruction))
    }
}

//! # Kata
//! 
//! Your rover has a position (in a 2d plane) and a direction (where he's looking at).
//! 
//! It must accept a list of instructions, and handle:
//! 
//! - Move one unit forward in the direction he's looking at
//! - Rotate clockwise 90 degrees
//! - Rotate counterclockwise 90 degrees
//! 
//! # Examples
//! 
//! ```
//! use rust_rover::{Rover, Instruction, Position, Direction};
//! use rust_rover::Instruction::*;
//! use rust_rover::Direction::*;
//! 
//! let rover = Rover::new(Position(0, 0), N);
//! let instructions = Instruction::from_string("FRFFLF")
//!     .unwrap();
//! 
//! assert_eq!(
//!     rover.execute_many(instructions),
//!     Rover::new(Position(2, 2), N),
//! );
//! ```
//! 
//! ```
//! use rust_rover::{Rover, Instruction, Position, Direction};
//! use rust_rover::Instruction::*;
//! use rust_rover::Direction::*;
//! 
//! let rover = Rover::new(Position(12345678912 3456789u64, 0), N);
//! let instructions = Instruction::from_string("FRFFLF")
//!     .unwrap();
//! 
//! assert_eq!(
//!     rover.execute_many(instructions),
//!     Rover::new(Position(123456789123456791, 2), N),
//! );
//! ```

#![feature(try_from)]

mod direction;
mod position;
mod rover;
mod instruction;

pub use crate::direction::Direction;
pub use crate::position::Position;
pub use crate::rover::Rover;
pub use crate::instruction::Instruction;

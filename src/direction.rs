/// The four cardinal directions
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Direction {
  N,
  S,
  E,
  W,
}

impl Direction {
  /// Rotate clockwise
  /// 
  /// # Examples
  ///
  /// ```
  /// # use rust_rover::Direction;
  /// assert_eq!(
  ///   Direction::N.rotate_cw(),
  ///   Direction::E,
  /// )
  /// ```
  pub fn rotate_cw(self) -> Direction {
    match self {
      Direction::N => Direction::E,
      Direction::S => Direction::W,
      Direction::E => Direction::S,
      Direction::W => Direction::N,
    }
  }

  /// Rotate counterclockwise
  /// 
  /// # Examples
  ///
  /// ```
  /// # use rust_rover::Direction;
  /// assert_eq!(
  ///   Direction::N.rotate_ccw(),
  ///   Direction::W,
  /// )
  /// ```
  pub fn rotate_ccw(self) -> Direction {
    match self {
      Direction::N => Direction::W,
      Direction::S => Direction::E,
      Direction::E => Direction::N,
      Direction::W => Direction::S,
    }
  }
}

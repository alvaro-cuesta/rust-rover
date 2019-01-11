use core::convert::TryFrom;

/// Instructions for the rover
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Instruction {
    /// Advances forward one unit
    Forward,
    /// Rotates clockwise 90 degrees
    RotateCW,
    /// Rotates counterclockwise 90 degrees
    RotateCCW,
}

/// Parse an instruction from a `char`
/// 
/// # Examples
/// 
/// ```
/// #![feature(try_from)]
/// # use rust_rover::Instruction;
/// 
/// use std::convert::TryFrom;
/// 
/// assert_eq!(
///     Instruction::try_from('F'),
///     Ok(Instruction::Forward),
/// )
/// ```
/// 
/// ```
/// #![feature(try_from)]
/// # use rust_rover::Instruction;
/// 
/// use std::convert::TryFrom;
/// 
/// assert_eq!(
///     Instruction::try_from('w'),
///     Err('w'),
/// )
/// ```
impl TryFrom<char> for Instruction {
    type Error = char;

    fn try_from(from: char) -> Result<Instruction, char> {
        match from {
            'F' => Ok(Instruction::Forward),
            'R' => Ok(Instruction::RotateCW),
            'L' => Ok(Instruction::RotateCCW),
            from => Err(from),
        }
    }
}

impl Instruction {
    /// Parse a string into a list of instructions
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rust_rover::Instruction;
    /// assert_eq!(
    ///     Instruction::from_string("FFRL"),
    ///     Ok(vec![
    ///         Instruction::Forward,
    ///         Instruction::Forward,
    ///         Instruction::RotateCW,
    ///         Instruction::RotateCCW,
    ///     ]),
    /// )
    /// ```
    /// 
    /// ```
    /// # use rust_rover::Instruction;
    /// assert_eq!(
    ///     Instruction::from_string("FFRasdfL"),
    ///     Err('a'),
    /// )
    /// ```
    /// 
    pub fn from_string<S: AsRef<str>>(string: S) -> Result<Vec<Instruction>, char> {
        string.as_ref()
            .chars()
            .map(Instruction::try_from)
            .collect()
    }
}
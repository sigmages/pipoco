use std::{fmt::Display, str::FromStr};

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerShape {
    Circle,
    Cross,
    Empty,
}

#[derive(Debug, Error)]
#[error("Failed to parse PlayerShape to string")]
pub struct PlayerShapeError {
    msg: String,
}

impl FromStr for PlayerShape {
    type Err = PlayerShapeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" | "X" => Ok(Self::Cross),
            "o" | "O" => Ok(Self::Circle),
            "-" => Ok(Self::Empty),
            e => Err(
                PlayerShapeError{
                    msg: format!("Wrong shape, must be a Cross=X, Circle=O or Empty=<single whitespace> (ignore case), found={}", e),
                }
            )
        }
    }
}

impl Display for PlayerShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = match self {
            Self::Circle => "O",
            Self::Cross => "X",
            Self::Empty => "-",
        };
        write!(f, "{}", r)
    }
}

impl PlayerShape {
    pub fn to_integer(&self) -> i8 {
        match self {
            PlayerShape::Circle => 1,
            PlayerShape::Cross => -1,
            PlayerShape::Empty => 0,
        }
    }
    pub fn from_integer(i: i8) -> Result<Self, PlayerShapeError> {
        match i {
            1 => Ok(Self::Circle),
            -1 => Ok(Self::Cross),
            0 => Ok(Self::Empty),
            e => Err(PlayerShapeError {
                msg: format!(
                    "Wrong value, must be -1=Cross, 1=Circle or 0=Empty, found={}",
                    e
                ),
            }),
        }
    }
}

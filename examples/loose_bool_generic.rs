//! This example creates a repr bool with 0 as False, 1 aa True, and anything else as Unknown.
//! Also included are `From`/`TryFrom` implementations to convert to/from a standard bool.
//!
//! A non-generic version of this example is also available in `loose_bool.rs`.

use core::error::Error;
use core::fmt::{Display, Formatter};
use loose_enum::loose_enum;
use num_traits::{ConstOne, ConstZero, PrimInt};

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as Unknown.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum LooseBool<T: PrimInt + ConstZero + ConstOne> {
        #[default]
        False = T::ZERO,
        True = T::ONE,
    }
}

impl<T: PrimInt + ConstZero + ConstOne> LooseBool<T> {
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }

    // Orphan rule forbids `From` implementation, so we create our own method.
    pub fn from_bool(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl<T: PrimInt + ConstZero + ConstOne> TryFrom<LooseBool<T>> for bool {
    type Error = UnknownBoolError;

    fn try_from(value: LooseBool<T>) -> Result<Self, Self::Error> {
        match value {
            LooseBool::False => Ok(false),
            LooseBool::True => Ok(true),
            LooseBool::Unknown(_) => Err(UnknownBoolError),
        }
    }
}

/// Error returned when attempting to convert a [`LooseBool::Unknown`] into a `bool`.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct UnknownBoolError;

impl Display for UnknownBoolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cannot convert `LooseBool::Unknown` into `bool`.")
    }
}

impl Error for UnknownBoolError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loose_to_bool() {
        assert_eq!(LooseBool::<u8>::True.try_into(), Ok(true));
        assert_eq!(LooseBool::<u8>::False.try_into(), Ok(false));

        for i in 2..u8::MAX {
            let b: Result<bool, UnknownBoolError> = LooseBool::Unknown(i).try_into();
            assert_eq!(b, Err(UnknownBoolError), "Failed for i={i}");
        }
    }

    #[test]
    fn bool_to_loose() {
        assert_eq!(LooseBool::<i64>::from_bool(false), LooseBool::False);
        assert_eq!(LooseBool::<i64>::from_bool(true), LooseBool::True);
    }
}

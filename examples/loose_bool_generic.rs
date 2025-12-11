//! This example creates a repr bool with 0 as False, 1 aa True, and anything else as Undefined.
//! Also included are `From`/`TryFrom` implementations to convert to/from a standard bool.
//!
//! A non-generic version of this example is also available in `loose_bool.rs`.

use core::error::Error;
use core::fmt::{Display, Formatter};
use loose_enum::loose_enum;
use num_traits::{ConstOne, ConstZero, PrimInt};

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as Undefined.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum LooseBool<T: PrimInt + ConstZero + ConstOne> {
        /// A falsy value of zero.
        #[default]
        False = T::ZERO,
        /// A truthy value of one.
        True = T::ONE,
    }
}

impl<T: PrimInt + ConstZero + ConstOne> LooseBool<T> {
    /// Returns true if the value is [`True`](Self::True).
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns true if the value is [`False`](Self::False).
    pub fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }

    /// Orphan rule forbids `From` implementation, so we create our own method.
    pub fn from_bool(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl<T: PrimInt + ConstZero + ConstOne> TryFrom<LooseBool<T>> for bool {
    type Error = UndefinedBoolError;

    fn try_from(value: LooseBool<T>) -> Result<Self, Self::Error> {
        match value {
            LooseBool::False => Ok(false),
            LooseBool::True => Ok(true),
            LooseBool::Undefined(_) => Err(UndefinedBoolError),
        }
    }
}

/// Error returned when attempting to convert a [`LooseBool::Undefined`] into a `bool`.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct UndefinedBoolError;

impl Display for UndefinedBoolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cannot convert `LooseBool::Undefined` into `bool`.")
    }
}

impl Error for UndefinedBoolError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loose_to_bool() {
        assert_eq!(LooseBool::<u8>::True.try_into(), Ok(true));
        assert_eq!(LooseBool::<u8>::False.try_into(), Ok(false));

        for i in 2..u8::MAX {
            let b: Result<bool, UndefinedBoolError> = LooseBool::Undefined(i).try_into();
            assert_eq!(b, Err(UndefinedBoolError), "Failed for i={i}");
        }
    }

    #[test]
    fn bool_to_loose() {
        assert_eq!(LooseBool::<i64>::from_bool(false), LooseBool::False);
        assert_eq!(LooseBool::<i64>::from_bool(true), LooseBool::True);
    }
}

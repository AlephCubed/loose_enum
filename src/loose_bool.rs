use crate::loose_enum;
use core::error::Error;
use core::fmt::{Display, Formatter};
use num_traits::{ConstOne, ConstZero, PrimInt};

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as `Unknown`.
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

    // Todo Orphan rule forbids `From` impl.
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
#[derive(Debug)]
pub struct UnknownBoolError;

impl Display for UnknownBoolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cannot convert `LooseBool::Unknown` into `bool`.")
    }
}

impl Error for UnknownBoolError {}

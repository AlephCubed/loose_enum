use crate::loose_enum;
use core::error::Error;
use core::fmt::{Display, Formatter};
use num_traits::{ConstOne, ConstZero, PrimInt};

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as `Unknown`.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    #[cfg_attr(
        feature = "bevy_reflect",
        derive(bevy_reflect::Reflect),
        reflect(Debug, Clone, PartialEq)
    )]
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
    fn u8_to_loose() {
        assert_eq!(LooseBool::from(0u8), LooseBool::False);
        assert_eq!(LooseBool::from(1u8), LooseBool::True);

        for i in 2..u8::MAX {
            assert_eq!(
                LooseBool::from(i),
                LooseBool::Unknown(i),
                "Failed for i={i}"
            );
        }
    }

    #[test]
    fn i8_to_loose() {
        assert_eq!(LooseBool::from(0i8), LooseBool::False);
        assert_eq!(LooseBool::from(1i8), LooseBool::True);

        for i in 2..i8::MAX {
            assert_eq!(
                LooseBool::from(i),
                LooseBool::Unknown(i),
                "Failed for i={i}"
            );
        }

        for i in i8::MIN..0 {
            assert_eq!(
                LooseBool::from(i),
                LooseBool::Unknown(i),
                "Failed for i={i}"
            );
        }
    }

    #[test]
    fn loose_to_u8() {
        assert_eq!(LooseBool::<u8>::False.to_repr(), 0);
        assert_eq!(LooseBool::<u8>::True.to_repr(), 1);

        for i in 2..u8::MAX {
            assert_eq!(LooseBool::<u8>::Unknown(i).to_repr(), i, "Failed for i={i}");
        }
    }

    #[test]
    fn loose_to_i8() {
        assert_eq!(LooseBool::<i8>::False.to_repr(), 0);
        assert_eq!(LooseBool::<i8>::True.to_repr(), 1);

        for i in 2..i8::MAX {
            assert_eq!(LooseBool::<i8>::Unknown(i).to_repr(), i, "Failed for i={i}");
        }

        for i in i8::MIN..0 {
            assert_eq!(LooseBool::<i8>::Unknown(i).to_repr(), i, "Failed for i={i}");
        }
    }

    #[test]
    fn loose_to_bool() {
        assert_eq!(LooseBool::<i32>::True.try_into(), Ok(true));
        assert_eq!(LooseBool::<i32>::False.try_into(), Ok(false));

        for i in 2..u8::MAX {
            let b: Result<bool, UnknownBoolError> = LooseBool::<i32>::Unknown(i as i32).try_into();
            assert_eq!(b, Err(UnknownBoolError), "Failed for i={i}");
        }
    }

    #[test]
    fn bool_to_loose() {
        assert_eq!(LooseBool::<u64>::from_bool(false), LooseBool::False);
        assert_eq!(LooseBool::<u64>::from_bool(true), LooseBool::True);
    }
}

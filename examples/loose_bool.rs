use core::error::Error;
use core::fmt::{Display, Formatter};
use loose_enum::loose_enum;

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as `Unknown`.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum LooseBool: i32 {
        #[default]
        False = 0,
        True = 1,
    }
}

impl LooseBool {
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }
}

impl From<bool> for LooseBool {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl TryFrom<LooseBool> for bool {
    type Error = UnknownBoolError;

    fn try_from(value: LooseBool) -> Result<Self, Self::Error> {
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
        assert_eq!(bool::try_from(LooseBool::True), Ok(true));
        assert_eq!(bool::try_from(LooseBool::False), Ok(false));

        for i in 2..256 {
            assert_eq!(
                bool::try_from(LooseBool::Unknown(i)),
                Err(UnknownBoolError),
                "Failed for i={i}"
            );
        }
    }

    #[test]
    fn bool_to_loose() {
        assert_eq!(LooseBool::from(false), LooseBool::False);
        assert_eq!(LooseBool::from(true), LooseBool::True);
    }
}

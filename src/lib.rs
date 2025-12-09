#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
#[cfg(feature = "serde")]
pub use serde_core as __serde;

/// Defines a repr enum that supports any value. If a value does not match any case, it will be parsed as `Undefined`.
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! loose_enum {
    // Special case for strings:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: String
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            /// Any value that doesn't match another case.
            Undefined(String),
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other.to_string()),
                }
            }
        }

        impl<'a> From<&'a str> for $name {
            fn from(value: &'a str) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other.to_string()),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value.to_string(), )+
                    $name::Undefined(val) => val,
                }
            }
        }
    };



    // All other types:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: $ty:ident
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            /// Any value that doesn't match another case.
            Undefined($ty),
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                }
            }
        }

        impl From<$name> for $ty {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value, )+
                    $name::Undefined(val) => val,
                }
            }
        }
    };



    // Generic:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<$ty:ident $( : $first_bound:tt $(+ $other_bounds:tt)* )?>
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name<$ty$(: $first_bound $(+ $other_bounds)+)?> {
            $(
                $(#[$meta])*
                $variant
            ),+,
            /// Any value that doesn't match another case.
            Undefined($ty),
        }

        impl<$ty$(: $first_bound $(+ $other_bounds)+)?> From<$ty> for $name<$ty> {
            fn from(value: $ty) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                }
            }
        }

        // Todo Orphan rule forbids `From` impl.
        impl<$ty$(: $first_bound $(+ $other_bounds)+)?> $name<$ty> {
            /// Converts the case into its representation.
            /// Orphan rule forbids `From` implementation, so we create our own method.
            pub fn to_repr(self) -> $ty {
                match self {
                    $( $name::$variant => $value, )+
                    $name::Undefined(val) => val,
                }
            }
        }
    };
}

/// Defines a repr enum that supports any value. If a value does not match any case, it will be parsed as `Undefined`.
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! loose_enum {
    // Special case for strings:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: String
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            /// Any value that doesn't match another case.
            Undefined(String),
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other.to_string()),
                }
            }
        }

        impl<'a> From<&'a str> for $name {
            fn from(value: &'a str) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other.to_string()),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value.to_string(), )+
                    $name::Undefined(val) => val,
                }
            }
        }

        impl<'de> $crate::__serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__serde::Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;
                Ok(match val.as_str() {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other.to_string()),
                })
            }
        }

        impl $crate::__serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__serde::Serializer,
            {
                match self {
                    $( $name::$variant => str::serialize($value, serializer), )+
                    $name::Undefined(val) => str::serialize(val, serializer),
                }
            }
        }
    };



    // All other types:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: $ty:ident
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            /// Any value that doesn't match another case.
            Undefined($ty),
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                }
            }
        }

        impl From<$name> for $ty {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value, )+
                    $name::Undefined(val) => val,
                }
            }
        }

        impl<'de> $crate::__serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__serde::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                })
            }
        }

        impl $crate::__serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__serde::Serializer,
            {
                match self {
                    $( $name::$variant => $ty::serialize(&$value, serializer), )+
                    $name::Undefined(val) => $ty::serialize(val, serializer),
                }
            }
        }
    };



    // Generic:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<$ty:ident $( : $first_bound:tt $(+ $other_bounds:tt)* )?>
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name<$ty$(: $first_bound $(+ $other_bounds)+)?> {
            $(
                $(#[$meta])*
                $variant
            ),+,
            /// Any value that doesn't match another case.
            Undefined($ty),
        }

        impl<$ty$(: $first_bound $(+ $other_bounds)+)?> From<$ty> for $name<$ty> {
            fn from(value: $ty) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                }
            }
        }

        // Todo Orphan rule forbids `From` impl.
        impl<$ty$(: $first_bound $(+ $other_bounds)+)?> $name<$ty> {
            /// Converts the case into its representation.
            /// Orphan rule forbids `From` implementation, so we create our own method.
            pub fn to_repr(self) -> $ty {
                match self {
                    $( $name::$variant => $value, )+
                    $name::Undefined(val) => val,
                }
            }
        }

        impl<'de, $ty$(: $first_bound $(+ $other_bounds)+)? + $crate::__serde::Deserialize<'de>> $crate::__serde::Deserialize<'de> for $name<$ty> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__serde::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                })
            }
        }

        impl<$ty$(: $first_bound $(+ $other_bounds)+)? + $crate::__serde::Serialize> $crate::__serde::Serialize for $name<$ty> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__serde::Serializer,
            {
                match self {
                    $( $name::$variant => $ty::serialize(&$value, serializer), )+
                    $name::Undefined(val) => $ty::serialize(val, serializer),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    loose_enum!(
        #[derive(Debug, Eq, PartialEq)]
        enum StringEnum: String {
            Foo = "Foo",
            Bar = "Bar",
            None = &String::default(),
        }
    );

    loose_enum! {
        #[derive(Debug, Eq, PartialEq)]
        pub enum IntEnum: u8 {
            Zero = 0,
        }
    }

    loose_enum![
        #[derive(Debug, PartialEq)]
        pub(super) enum FloatEnum: f32 {
            Default = f32::default(),
            Pi = 3.14,
        }
    ];

    #[test]
    #[cfg(feature = "std")]
    fn string_to_enum() {
        assert_eq!(StringEnum::from("Foo".to_string()), StringEnum::Foo);
        assert_eq!(StringEnum::from("Bar"), StringEnum::Bar);
        assert_eq!(StringEnum::from(""), StringEnum::None);

        assert_eq!(
            StringEnum::from("Other"),
            StringEnum::Undefined("Other".to_string())
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn enum_to_string() {
        assert_eq!(String::from(StringEnum::Foo), "Foo".to_string());
        assert_eq!(String::from(StringEnum::Bar), "Bar".to_string());
        assert_eq!(String::from(StringEnum::None), "".to_string());

        assert_eq!(
            String::from(StringEnum::Undefined("Other".to_string())),
            "Other".to_string()
        );
    }

    #[test]
    fn int_to_enum() {
        assert_eq!(IntEnum::from(0), IntEnum::Zero);

        assert_eq!(IntEnum::from(123), IntEnum::Undefined(123));
    }

    #[test]
    fn enum_to_int() {
        assert_eq!(u8::from(IntEnum::Zero), 0);

        assert_eq!(u8::from(IntEnum::Undefined(123)), 123);
    }

    #[test]
    fn float_to_enum() {
        assert_eq!(FloatEnum::from(0.0), FloatEnum::Default);
        assert_eq!(FloatEnum::from(3.14), FloatEnum::Pi);

        assert_eq!(FloatEnum::from(123.0), FloatEnum::Undefined(123.0));
    }

    #[test]
    fn enum_to_float() {
        assert_eq!(f32::from(FloatEnum::Default), 0.0);
        assert_eq!(f32::from(FloatEnum::Pi), 3.14);

        assert_eq!(f32::from(FloatEnum::Undefined(123.0)), 123.0);
    }
}

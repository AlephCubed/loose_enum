#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod __internal;

/// Defines a repr enum that supports any value. If a value does not match any case, it will be parsed as `Undefined`.
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! loose_enum {
    // Special case for strings:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: String {
            $($body:tt)*
        }
    ) => {
        $crate::__loose_enum_type! {
            $(#[$outer])*
            $vis enum $name: String {
                $($body)*
            }
        }

        $crate::__loose_enum_impl! {
            $(#[$outer])*
            $vis enum $name: String {
                $($body)*
            }
        }
    };



    // All other types:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: $ty:ident {
            $($body:tt)*
        }
    ) => {
        $crate::__loose_enum_type! {
            $(#[$outer])*
            $vis enum $name: $ty {
                $($body)*
            }
        }

        $crate::__loose_enum_impl! {
            $(#[$outer])*
            $vis enum $name: $ty {
                $($body)*
            }
        }
    };



    // Generic:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<$ty:ident $( : $first_bound:tt $(+ $other_bounds:tt)* )?>
        {
            $($body:tt)*
        }
    ) => {
        $crate::__loose_enum_type! {
            $(#[$outer])*
            $vis enum $name<$ty $( : $first_bound $(+ $other_bounds)* )?> {
                $($body)*
            }
        }

        $crate::__loose_enum_impl! {
            $(#[$outer])*
            $vis enum $name<$ty $( : $first_bound $(+ $other_bounds)* )?> {
                $($body)*
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
        $vis:vis enum $name:ident: String {
            $($body:tt)*
        }
    ) => {
        $crate::__loose_enum_type! {
            $(#[$outer])*
            $vis enum $name: String {
                $($body)*
            }
        }

        $crate::__loose_enum_impl! {
            $(#[$outer])*
            $vis enum $name: String {
                $($body)*
            }
        }

        $crate::__loose_enum_serde! {
            $(#[$outer])*
            $vis enum $name: String {
                $($body)*
            }
        }
    };



    // All other types:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: $ty:ident {
            $($body:tt)*
        }
    ) => {
        $crate::__loose_enum_type! {
            $(#[$outer])*
            $vis enum $name: $ty {
                $($body)*
            }
        }

        $crate::__loose_enum_impl! {
            $(#[$outer])*
            $vis enum $name: $ty {
                $($body)*
            }
        }

        $crate::__loose_enum_serde! {
            $(#[$outer])*
            $vis enum $name: $ty {
                $($body)*
            }
        }
    };



    // Generic:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<$ty:ident $( : $first_bound:tt $(+ $other_bounds:tt)* )?>
        {
            $($body:tt)*
        }
    ) => {
        $crate::__loose_enum_type! {
            $(#[$outer])*
            $vis enum $name<$ty $( : $first_bound $(+ $other_bounds)* )?> {
                $($body)*
            }
        }

        $crate::__loose_enum_impl! {
            $(#[$outer])*
            $vis enum $name<$ty $( : $first_bound $(+ $other_bounds)* )?> {
                $($body)*
            }
        }

        $crate::__loose_enum_serde! {
            $(#[$outer])*
            $vis enum $name<$ty $( : $first_bound $(+ $other_bounds)* )?> {
                $($body)*
            }
        }
    };
}

#[cfg(test)]
mod tests {
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

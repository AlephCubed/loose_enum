#[doc(hidden)]
#[macro_export]
macro_rules! __loose_enum_type {
    // All types (including String):
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: $ty:ident {
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
    };



    // Generic:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<$ty:ident $( : $first_bound:tt $(+ $other_bounds:tt)* )?> {
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __loose_enum_impl {
    // Special case for strings:
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident: String {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
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
        $vis:vis enum $name:ident<$ty:ident $( : $first_bound:tt $(+ $other_bounds:tt)* )?> {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
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

#[cfg(feature = "serde")]
pub use serde_core as serde;

#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! __loose_enum_serde {
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
        impl<'de> $crate::__internal::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__internal::serde::Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;
                Ok(match val.as_str() {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other.to_string()),
                })
            }
        }

        impl $crate::__internal::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__internal::serde::Serializer,
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
        impl<'de> $crate::__internal::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__internal::serde::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                })
            }
        }

        impl $crate::__internal::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__internal::serde::Serializer,
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
        impl<'de, $ty$(: $first_bound $(+ $other_bounds)+)? + $crate::__internal::serde::Deserialize<'de>> $crate::__internal::serde::Deserialize<'de> for $name<$ty> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::__internal::serde::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Undefined(other),
                })
            }
        }

        impl<$ty$(: $first_bound $(+ $other_bounds)+)? + $crate::__internal::serde::Serialize> $crate::__internal::serde::Serialize for $name<$ty> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__internal::serde::Serializer,
            {
                match self {
                    $( $name::$variant => $ty::serialize(&$value, serializer), )+
                    $name::Undefined(val) => $ty::serialize(val, serializer),
                }
            }
        }
    }
}

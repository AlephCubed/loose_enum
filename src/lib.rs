#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "loose_bool")]
mod loose_bool;

#[cfg(feature = "loose_bool")]
pub use loose_bool::LooseBool;

/// Defines a repr enum that supports any value. If a value does not match any case, it will be parsed as `Unknown`.
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
            Unknown(String),
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Unknown(other.to_string()),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value.to_string(), )+
                    $name::Unknown(val) => val,
                }
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde_core::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde_core::Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;
                Ok(match val.as_str() {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Unknown(other.to_string()),
                })
            }
        }

        #[cfg(feature = "serde")]
        impl serde_core::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde_core::Serializer,
            {
                match self {
                    $( $name::$variant => str::serialize($value, serializer), )+
                    $name::Unknown(val) => str::serialize(val, serializer),
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
            Unknown($ty),
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Unknown(other),
                }
            }
        }

        impl From<$name> for $ty {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value, )+
                    $name::Unknown(val) => val,
                }
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde_core::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde_core::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Unknown(other),
                })
            }
        }

        #[cfg(feature = "serde")]
        impl serde_core::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde_core::Serializer,
            {
                match self {
                    $( $name::$variant => $ty::serialize(&$value, serializer), )+
                    $name::Unknown(val) => $ty::serialize(val, serializer),
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
            Unknown($ty),
        }

        impl<$ty$(: $first_bound $(+ $other_bounds)+)?> From<$ty> for $name<$ty> {
            fn from(value: $ty) -> Self {
                match value {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Unknown(other),
                }
            }
        }

        // Todo Orphan rule forbids `From` impl.
        impl<$ty$(: $first_bound $(+ $other_bounds)+)?> $name<$ty> {
            pub fn to_repr(self) -> $ty {
                match self {
                    $( $name::$variant => $value, )+
                    $name::Unknown(val) => val,
                }
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, $ty$(: $first_bound $(+ $other_bounds)+)? + serde_core::Deserialize<'de>> serde_core::Deserialize<'de> for $name<$ty> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde_core::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( c if c == $value => $name::$variant, )+
                    other => $name::Unknown(other),
                })
            }
        }

        #[cfg(feature = "serde")]
        impl<$ty$(: $first_bound $(+ $other_bounds)+)? + serde_core::Serialize> serde_core::Serialize for $name<$ty> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde_core::Serializer,
            {
                match self {
                    $( $name::$variant => $ty::serialize(&$value, serializer), )+
                    $name::Unknown(val) => $ty::serialize(val, serializer),
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
        enum StringEnum: String {
            Apple = "Apple",
            Banana = "Banana",
            Orange = String::default(),
        }
    );

    loose_enum! {
        pub enum IntEnum: u8 {
            Zero = 0,
        }
    }

    loose_enum![
        pub(super) enum FloatEnum: f32 {
            Default = f32::default(),
            Pi = 3.14,
        }
    ];
}

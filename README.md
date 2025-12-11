# Loose Enum
[![Version](https://img.shields.io/crates/v/loose_enum)](https://crates.io/crates/loose_enum)
[![Docs](https://img.shields.io/docsrs/loose_enum)](https://docs.rs/loose_enum)
![License](https://img.shields.io/crates/l/loose_enum)

A macro for defining loose repr enums.

When parsing userdata, you often have known/supported cases; however, users don't always follow the rules.
One way to solve this is having a backup `Undefined` case that supports any value. This crate hopes to simplify this process.

### Example:
For example, an integer repr bool, with 0 being false and 1 being true would look something like this:
```rust
loose_enum::loose_enum! {
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum LooseBool: i32 {
        #[default]
        False = 0,
        True = 1,
    }
}
```
Which expands into the following:
```rust ignore
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LooseBool {
    #[default]
    False,
    True,
    /// Any value that doesn't match another case. 
    Undefined(i32),
}

impl From<i32> for LooseBool { /* ... */ }
impl From<LooseBool> for i32 { /* ... */ }

// If feature flag `serde` is enabled:
impl Serialize for LooseBool { /* ... */ }
impl Deserialize for LooseBool { /* ... */ }
```
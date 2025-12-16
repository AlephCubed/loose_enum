A macro for defining loose repr enums.

When parsing userdata, you often have known/supported cases; however, users don't always follow the rules.
One way to solve this is having a backup `Undefined` case that supports any value. This macro hopes to simplify this process.

### Example:
An integer repr bool, with 0 being false and 1 being true would look something like this:
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
Which expands into this type:
```rust
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LooseBool {
    #[default]
    False,
    True,
    /// Any value that doesn't match another case. 
    Undefined(i32),
}
```
The macro will also generate `From<i32>` and `Into<i32>` trait implementations. 
If the `serde` feature is enabled, `Serialize` and `Deserialize` will also be implemented.
## Special Cases
### String
A special case has been created for when `String` is the data type, which will implement `From<&str>` on top of the normal trait implementations.
### Generics
There is also **experimental** support for generic types.
```rust
use num_traits::{ConstZero, ConstOne};

loose_enum::loose_enum! {
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum LooseBool<T: PartialEq + ConstZero + ConstOne> {
        #[default]
        False = T::ZERO,
        True = T::ONE,
    }
}
```
Due to the orphan rule, `Into<T>` cannot be implemented. Instead, an `into_repr` method will be added.
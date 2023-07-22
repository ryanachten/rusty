# Rusty

## Method syntax

- Similar to functions, except they are defined within the context of a struct, enum, or trait
- Their first parameter is always `self` which represents the instance the method is being called on

### Defining methods

```rust
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect = Rectangle {
   width: 2,
   height: 3
};
let a = rect.area(); // 6
```

- we define the method in an `impl` implementation block for the Rectangle struct
  - everything in the `impl` block will be associated with the `Rectangle` type
- `&self` is short for `self: &Self`
  - In an `impl` block, `Self` refers to the type that the `impl` block is associated with (in this case, `Rectangle`)
  - is used here in this context instead of `rectangle: &Rectangle` (I believe this is equivalent to what we use in Golang)
- We can use this approach to create _getters_ (i.e. the `width()` method above)
  - can use this to keep the field private (private and public fields are discussed later)
- We can also create multiple `impl` blocks for the same type if we need to

### Where's the -> operator

- In C/C++ there are two approaches for calling methods:
  - Can use the `.` reference when calling on an instance directly
  - Or use `->` when calling on a pointer
    - We need to _deference_ the pointer first
  - Rust doesn't have this because it has a feature called _automatic referencing and dereferencing_

### Associated functions

- All functions inside of an `impl` block are _associated functions_
  - in the sense that they're associated with the `impl` type
- We can design associated functions which don't have `self` as the first parameter
  - these are no methods as they don't have access to an instance
    - function more like static methods in OOP
  - to call these we use `::`

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
let sq = Rectangle:square(3);
```

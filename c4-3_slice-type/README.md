# Rusty

## The Slice type

- _Slices_ let you reference a sequence of elements in a collection (rather than the whole collection)
  - it's a kind of reference, so doesn't have ownership

### String Slices

- A _string slice_ is a references to part of a `String`
- Can use Rust's range syntax to return a slice
  - looks like Python list slices

```rust
let s = String::from("hello world");
let hello = &s[0..5]; // hello
let world = &s[6..11]; // world
// can also do
let slice = &s[3..];
let slice = &s[..3];
let slice = &s[..]; // will return entire string as a slice
```

- Resulting string slice type return will be of type `&str`

### String slices as parameters

- Use `&str` as a function param instead of `&String`
  - allows us to pass `String` slice or references as arguments
  - this flexibility takes advantage of _deref coercions_ discussed later

### Other slices

- We can do the same with other arrays, such as integer arrays
  - instead an int array will have type like `&[i32]`

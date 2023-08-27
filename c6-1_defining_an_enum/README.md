# Rusty

## Enum values

```rust
enum IpAddrKind {
    V4,
    V6,
}
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- Variants of enums are namespaced under their identifier
  - we use a double colon to reference the value `::`

### Storing values

We can store values in an enum instead of in a struct:

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
```

- now `IpAddr::V4` is a function that returns an `IpAddr` type
- each variant can have a different constructor signature
- can also pass entire structs as part of the constructor

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

- The advantage of using an enum to group these is we can now use polymorphism to write a function which accepts the enum, regardless of which variant it is

### Enum methods

- We define methods for enums using `impl` blocks like we do for structs

```rust
impl IpAddr {
    fn call(&self) {
        // do something
    }
}

let home = IpAddr::V4(String::from("127.0.0.1"));
home.call();
```

### The `Option` enum and advantages over null values

the `Option` type encodes the very common scenario in which a value could be defined/undefined

- expressing this concept in terms of a type means that the compiler can check that we've handled all the scenarios

  - in turn, this can help prevent potential bugs

- Rust doesn't have a null type

  - null is a value that _doesn't have a value_
  - in languages with null, there is always two states; variables with values and those without
    - because of how pervasive null usage is it's extremely easy to encounter null exceptions
  - the concept of "not having a value" is still a useful one

- instead Rust has a `Option<T>` type:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

- this enum is included in the prelude
  - don't need to manually include it
  - also don't need to reference `Some` and `None` variants by `Option::`

```rust
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
```

- We need to extract the value from `Option<T>` before we do anything with it
  i.e. this won't work

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```

- the `Option<T>` type has a lot of [helpers](https://doc.rust-lang.org/std/option/enum.Option.html) to handle situations like this
- generally we need to ensure that we have `Some(T)` values before we can access the inner `T`
- need some other code to handle the `None` case
- we can use a `match` expression to produce a control flow which handles both scenarios

# Rusty

## Defining and Instantiating Structs

To define a struct, we use the `struct` keyword:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

We can then use the struct by creating an instance of it

- fields don't need to be declared in the same order they were defined in

```rust
let user = User {
    active: true,
    email: String::from("test@test.com"),
    username: String::from("test"),
    sign_in_count: 1
}
```

- Can also use dot notation to change a value:
- **Note** the entire instance must be mutable
  - Can't mark only certain fields as mutable

## Using the Field Init shorthand

When the parameter names and struct field names are exactly the same, we can use the _field init shorthand_ to make the code more concise:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of `username: username`
        email, // instead of `email: email`
        ...
    }
}
```

## Creating instances from other instances with Struct Update syntax

We often want to create a new instance with most of the values of another instance, but with some changes. Can achieve this using the _struct update syntax_:

```rust
let user2 = User {
    email: String::from("user2@test.com"),
    ..user1
}
```

- **Note** the struct update syntax uses `=` assignment
  - this means it _moves_ the data from the `user1` assignment to `user2`
  - meaning **we can no longer use `user1` as a whole** (because string values don't implement the `Copy` trait)
  - if we had only copied the `active` and `sign_in_count` fields then `user1` would be fine because their data types implement `Copy`

## Tuple structs

Rust supports structs that look similar to tuples.

- Has the added meaning that structs provide, but don't have field names; just data types (like tuples)
- Useful when you want to create a data type out of a tuple, but naming each field would be verbose or redundant, i.e.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

## Unit-like structs without fields

Can define structs without fields

- behave similar to an empty tuple _unit_ `()`
- can be useful when we need to implement a trait on some type but don't have any data you want to store on the type itself

```rust
struct AlwaysEqual; // every instance will always be equal
```

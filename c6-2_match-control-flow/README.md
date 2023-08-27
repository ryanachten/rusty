# Rusty

## The `match` control flow construct

- Rust has a control flow construct called `match`
  - can compare a value against a series of patterns and execute code based on the results
  - patterns can be made up of:
    - literal values
    - variable names
    - wildcards
    - etc

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

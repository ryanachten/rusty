# Rusty

## Functions

- Rust code uses snake-case for function and variable names
- Define a function using the `fn` keyword

### Parameters

- Must declare the type of each function parameter

### Statements and expressions

- **Statements**: are instructions that perform and action and do not return a value
- **Expressions**: are instructions that evaluate to a resultant value
- Some languages return the value of assignment (i.e. JavaScript)

  - i.e. can do this `x = y = 6`, where `x` and `y` will now equal 6
  - this is not the case in Rust
    - `y = 6` is a statement and does not have a return value
    - will result in an error, since there is nothing for `x` to bind to

- However, a block scope is an expression, so we can do the following

```rust
    let y = {
        let x = 3;
        x + 1 // does **not** have a semicolon
    };
    // y = 4
```

- Note that `x + 1` does not have a semicolon
  - adding one would make this a statement prevent a return value
  - _expressions do not have semicolons_

### Functions with return values

- Must declare the type of a function's return value after an `->`
- Can return early by explicitly `return`ing a value
  - most functions return a value implicitly

```rust
fn five() -> i32 {
    5 // note no semicolon - implicit return
}
let x = five(); // x = 5
```

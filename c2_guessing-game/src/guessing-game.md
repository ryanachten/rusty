# Rusty

## Guessing Game

### Imports

- Rust has a number of items it brings into the scope of every program, known as the _prelude_. To include types which aren't in the prelude, we need to import them explicitly.
- We can reference a library using the `use` statement
  - i.e. `use std::io` for input/output from the standard library

### Variables

- We can declare a variable using the `let` keyword, i.e. `let guess = "5"`
  - By default, this immutable
- We can declare a variable as being mutable via the `mut` keyword, i.e. `let mut guess = "5"`

- We can then update this variable, by passing it into the method argument via `.read_line(&mut guess)`
  - `&` is a reference, which allows multiple parts of the code to access one piece of data (seems similar to .NET's `ref` keyword)
  - References too are immutable by default, so we need to use `&mut` to make this a mutable reference (instead of `&guess`).

### Handling potential failure

- In addition to putting the user input into the string variable, `.read_line` also returns as `Result` value
  - `Result` is an enum containing different possible states (each state being a _variant_) - `Ok` and `Err`
- `Result` objects also have an `.expect(msg)` method
  - If the result is an `Err`, then `.expect` will cause the program to crash and display the message we pass into it
  - If we don't call `.expect`, then we'll get a warning at compile time, indicating that a potential error isn't being handled - awesome!
- We can explicitly handle the different variants using a `match` expression
  - the `Err(_)` will match all error values, regardless of what information they have in them

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue
}
```

### Dependencies

- To add a dependency to the Cargo project, we can update the `.toml` file `[dependencies]` section to include a package, i.e. `rand = "0.8.5"`
- We then rebuild the project to download the dependency from the Creates.io registry
- When we want to update dependencies, we can run `cargo update` and Cargo will figure out the latest versions which fit our specifications.
  - This will only do minor updates, to bump major versions, we need to modify the `.toml` file explicitly

### Range

- We can specify a range expression using `1..=100` which is inclusive of lower and upper bounds.
- We can build docs relating to our project via `cargo doc --open`. This will provide information about not only our own code, but also crate dependencies

### Comparisons

- `Ordering` is another enum w/ variants `Less`, `Greater` and `Equal`
- We can use a `match` expression to call a function based on which variant is returned in the comparison
  - A `match` expression is made up of _arms_
  - An _arm_ consists of a _pattern_ to match against, which if fulfilled, will execute the callback accordingly

### Casting

- We can convert the `guess` string variable to a int via by parsing it:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- We don't need to create a separate variable to hold the result due to Rust's _shadowing_ (discussed further later)
  - Shadowing is often used when converting variable types

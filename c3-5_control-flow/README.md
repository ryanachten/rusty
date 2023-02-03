# Rusty

## `if` expressions

- Don't need to wrap conditions in brackets, i.e.

```rust
let number = 3;
if number < 5 {

}
```

- blocks of code associated with an if condition are sometimes called _arms_
- the condition inside of the if condition **must be a bool**
  - i.e. can't do `if number {` like you can in JS

### Using `if` in a `let` statement

- We can use an `if` statement as part of a let assignment
  - i.e. `let number = if condition { 5 } else { 6 };`
- as long as different arms of the if statement evaluate to the same data type
  - can't do `let number = if condition { 5 } else { "six" };`

## Repetition with loops

- Rust has three kinds of loops
  - `loop`, `while`, `for`

### `loop`

- will loop forever until we explicitly tell it to stop

```rust
loop {
    println!("looping!");

    if some_condition {
        break; // will loop forever unless this condition is met
    }
}
```

#### returning values from loops

- one if the uses of a `loop` is to retry an operation you know might fail
  - i.e. check if a thread has finished its job
- we can `break` with a return value followed by a semicolon

```rust
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2; // note the semicolon
    }
};
// result = 20
```

#### labels to disambiguate between loops

- loop labels can be used to explicitly reference a loop we want to `break` or `continue`
- loop labels must begin with a single quote i.e. `'some_label`
- useful when we're got nested loops at play

```rust
'counting_up: loop {
    let mut remaining = 10;
    loop {
        if remaining == 9 {
            break; // will break the unlabelled inner loop
        }
        if count == 2 {
            break 'counting_up; // will break outer loop
        }
        remaining -= 1;
    }
}
```

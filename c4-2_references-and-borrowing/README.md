# Rusty

## References and borrowing

- A _reference_ is like a pointer
  - in the sense it's an address we can use to access stored data
  - but unlike a pointer, a reference is guaranteed to point to a valid value for the life of the reference

```rust
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // We can still use s1, since the calculate_length call hasn't taken ownership

fn calculate_length(s: &String) -> usize {
    s.len()
    // because s is just a reference, the value is not dropped at the end of this scope
}
```

- We declare the use of a reference using `&`
  - in the function param, we use `&String` instead of `String`
  - this allows us to pass a reference to value, without the function taking ownership of the value
- the opposite of referencing is _dereferencing_ using the `*` operator

  - will discuss this later

- The action of creating a reference is called _borrowing_
  - like borrowing someone's stuff, we can't modify it by default

```rust
    let s = String::from("hello");
    change(&s);

fn change(some_string: &String) { //
    some_string.push_str(", world"); // this would throw an error
}
```

### Mutable references

- to allow us to modify borrowed references, we can use the `mut` keyword
  - this makes it very clear that the function will mutate the value it borrows

```rust
    let mut s = String::from("hello");
    change(&mut s);

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- **Note** mutable references have one big restriction:
  - you can only have _one mutable reference_ to a value
  - attempts to create two mutable references will fail

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // this reference will fail
```

- this restriction helps prevent _data races_

  - a data race is similar to a race condition
  - occurs when the following behaviours occur:
    1. two or more pointers access the same data at the same time
    2. at least one of the pointers is being used to write data
    3. there's no mechanism to synchronize access to the data
  - can be difficult to diagnose these issues
    - Rust avoids this by preventing them at compile time

- We can work around this by moving a reference into a different scope
  - since these will not longer be _simultaneous_ references

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s;
```

- **Note** we also cannot have a mutable reference when we have an _immutable reference_
  - In short you can have:
    - _**A single** mutable reference_
    - OR _**Multiple** immutable references_

```rust
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{} and {}", r1, r2); // we can fix this by moving this above the r3 declaration, cause then r1 and r2 will be out of scope allowing a mutable reference
```

### Dangling references

- A _dangling pointer_ is a pointer to a location in memory that may have been given to someone else
  - done by freeing memory but preserving the pointer
  - if you have a reference, the compiler will ensure that _the data will not go out of scope before the reference does_

```rust
let reference_to_nothing = dangle();

fn dangle() -> &String {
    let s = String::from("hello");
    &s // here's our dangling reference: we return a reference, but the s value goes out of scope at the end of this function
}
```

- We can address the above issue by simply returning the value `s` instead of the reference `&s`

# Rusty

## Variables and mutability

- If we were to try and reassign a variable w/o the `mut` keyword, then the compiler would throw an immutability error, i.e.

```rust
let x = 5;
x = 6; // throws: cannot assign twice to immutable variable `x`
```

- the Rust compiler guarantees that if we state a variable shouldn't change, then it won't change

### Constants

- Constants are bound to a name and _not_ allowed to change
  - even through use of `mut`
- Use `const` instead of `let` to declare constants
- Type _must_ be annotated
- Can be declared in any scope (including global scope)
- Must be set to a constant expression (not one determined at runtime)
- i.e. `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

### Shadowing

- You can declare a new variable with the same name as an existing variable
  - the existing variable will be _shadowed_ by the new one
  - meaning, the compiler will see the second variable when referencing the variable name
- Shadowing is scope-sensitive, so a shadow established in one scope is accessible only to that scope
- Shadowing is different to use of `mut` because we can create transformations on a variable, and have the result be immutable after
- Effectively just creates a new variable
  - Because of this, we can also change a variable's type via shadowing (which we can't do with `mut` reassignment)

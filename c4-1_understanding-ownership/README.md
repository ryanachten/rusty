# Rusty

## What is ownership

- _Ownership_ is a set of rules which govern how Rust manages memory
- Some languages have garbage collection that looks for no longer used memory at runtime
- Other languages require programmers to explicitly allocate and free memory
- Rust uses a third approach - memory is managed through a system of ownership
  - using a set of rules that the compiler checks
  - this does not slow down the program at runtime

### The Stack and the Heap

- Many programming languages don't require thinking about the stack and heap
  - system programming languages like Rust do
    - a value being on the stack or heap impacts how the language behaves
- Parts of ownership relate to the stack and heap

**Stack**

- Stack uses LIFO
- All data stored on the stack must have _a known and fixed size_
  - data of unknown size at compile time need to go on the heap

**Heap**

- Is less organised
- When placing on the heap, we request a certain amount of space
  - memory allocator finds a spot that is big enough
    - marks the spot as in use
    - returns a _pointer_
  - process is called _allocating to the heap_
- because the pointer is known and of fixed size, we can store this pointer on the stack

  - but when retrieving the data, we still need to follow the pointer back to the heap

- pushing to the stack is _faster_ than allocating on the heap because we don't need to find a place to store data
- accessing data from the heap is also _slower_ than retrieving from the stack because we need to follow the pointer

### Ownership rules

- Each value in Rust has a _owner_
- There can only be _one owner_ at a time
- When the owner goes out of scope, the value will be dropped

### The `String` type

- the data types we've discussed so far are of a fixed size
  - can therefore be stored on the stack
- the `String` type is not of fixed length and is therefore stored on the heap

- String literals (where the string is hardcoded) aren't suitable for every situation (has `str` type I think)

  - they're immutable
  - sometimes not every string value can be known (i.e. `StringBuilder` scenarios)

- A second type `String` manages data on the heap
  - able to store an amount of text which is unknown at compile time
  - can convert from string literal using `let s = String::from("hello")`

### Memory and allocation

- with a string literal, we know the contents, so the text is hardcoded directly into the final executable
- we can't do that with the `String`

  - can't put a block of memory into binary for text of an unknown size

- `String::from()` does the memory request to the memory allocator
  - this is consistent with other programming languages
- we also need to return the memory when we're done with the value
  - in garbage collected languages, the GC will do this for us
  - in Rust, once the scope of the value has finished, the memory will be freed
    - under the hood, Rust will call the `drop` function to deallocate the memory
    - this is the same pattern as the _Resource Acquisition Is Initialization (RAII)_ pattern in C++
  - this pattern has an impact on how Rust code is written and can cause unexpected behaviour

### Variables and data interacting with move

- Given the following example

```rust
let x = 5;
let y = x;
```

- We have two values bound to 5 on the stack

- Given the following example

```rust
let s1 = String::from("hello");
let s2 = s1;
```

- We have a `s1` bound to a pointer in heap and `s2` bound to the _same_ pointer
  - the problem now is, if `s1` and `s2` both go out of memory, it will result in a _double free error_
    - freeing memory twice can lead to memory corruption and security vulnerabilities
  - therefore after `let s2 = s1;`, Rust doesn't consider `s1` valid any more
  - because `s1` is invalidated, this isn't a _shallow copy_ of data, but rather called a _move_
    - where `s1` has moved into `s2`

### Variables and data interacting with clone

- If we do want to _deep copy_ a value on the heap, we can use a common method called `clone`
- this will copy the actual data on the heap
  - this can be expensive

### Stack-Only Data: Copy

- Rust has a special annotation called `Copy`
  - this can be applied to types stored on the stack
  - when applied, types are no longer _moved_ but are instead copied
  - We can't apply this to types that implement the `Drop` trait (i.e. heap data types)
  - this is implemented by the following types:
    - integer types
    - Boolean types
    - floating point types
    - Tuples (as long as these contain types implementing `Copy`)
  - this is why, when we assign integer values, then we can still access the original value:
  ```rust
    let mut x = 5;
    let mut y = x;
    x += 1; // couldn't do this w/o the `Clone` trait
    y += 2;
  ```

### Ownership and functions

- Passing a value to a function is similar to assigning a value to variable
  - it will either _move_ or _copy_, just like an assignment

```rust
    let s = String::from("hello");
    takes_ownership(s);
    // s's value moves into the function
    // we can no longer use s in this scope
```

- to workaround this, we could return a value from `takes_ownership` and assign it to a new variable
- we can simplify this process through the use of _references_

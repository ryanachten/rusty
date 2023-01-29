# Rusty

## Data types

- We'll look at two data type subsets; _scalar_ and _compound_
- Rust is a _statically_ typed language
  - it must know the types of all variables at compile time
- The compiler can usually infer the type of a variable based on its value and use
  - However, when many types are possible (such as when parsing variables), we need to add a type annotation

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

- Omitting the `: u32` type assignment will result in a compilation error

### Scalar types

- Scalar types represent a _single value_
- Rust has 4 primary scalar types:
  - integers
  - floating-point
  - Booleans
  - characters

#### Integer types

##### Signed bits

- Can be signed, prefixed with an `i`, i.e. `i32` for signed 32 bit
  - can be positive or negative
  - stored using two's compliment
- Or unsigned, prefixed with a `u`, i.e. `u32` for unsigned 32 bit
  - will only ever be positive

##### Integer lengths

- There are different lengths based on how much space we need to allocate to a number

  - 8-bit, `i8` (-128 -> 127), `u8` (0 -> 255),
  - 16-bit, `i16`, `u16`
  - 32-bit, `i32`, `u32`
  - 64-bit, `i64`, `u64`
  - 128-bit, `i128`, `u128`
  - arch, `isize`, `usize`
    - will use 64 or 32 bit depending on the machine architecture the program is running on
    - the main use for this is when indexing a collection

- If you're unsure what to use, Rust defaults are a good place to start

  - for integers, this is `i32`

- Using the wrong length can result in **integer overflow**
  - in debug mode, this will result in a _panic_ and exit with error
  - is compiled in release mode, Rust will perform two's compliment wrapping, effectively rounding the number
    - can result in unexpected values
  - We can handle these scenarios by using:
    - `wrapping_*` methods to wrap numbers
    - `checked_*` to check for overflow
    - `saturate_*` to define a max and min value range

##### Integer formats

- We can write integers in the following formats:
  - Decimal: `1_000`
    - `_` can be used to make numbers easier to read
  - Hex: `0xff`
  - Octal: `0off`
  - Binary: `0b1111_0000`
  - Byte: `b'A'`
    - can only be used in `u8`

#### Floating-point types

- Can have either `f32` or `f64` floating point numbers
  - Rust default is `f64`
  - `f64` is roughly the same speed on modern CPUs with higher precision

### Compound types

- _Compound types_ can group multiple values in one type
- Rust has two primitive compound types; _tuples_ and _arrays_

#### Tuple

- Tuple is a general way of grouping different values and types together
- Have a fixed length; once declared, can't change size

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- We can then destructure tuples via:

```rust
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
```

- We can directly access a tuple element by using dot notation followed by the index we want to access

```rust
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
```

- A tuple without any values is called a _unit_
  - used to represent an empty return type

#### Array

- Elements must be of the same type
- Are useful when we want data allocated on the _stack_ rather than the _heap_
- Or when we will have numbers of a fixed length

- A _vector_ is a similar collection type provided by the standard library

  - but a vector _can_ change is size
  - if you're unsure which to use, you'll probably want a vector

- We can declare the type of an array by the type of the elements followed by the array length

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- Cam initialise an array to fill with a single number via the type:

```rust
let a = [3; 5]; // will create the following array [3,3,3,3,3]
```

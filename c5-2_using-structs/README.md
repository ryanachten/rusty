# Rusty

## Adding useful functionality with Derived Traits

If we try to print a struct using the `println` macro, it won't work by default

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("rect1 is {}", rect1); // will throw an error `Rectangle` doesn't implement `std::fmt::Display`
```

- We can change the print statement to use `println!("rect1 is {:?}", rect1);`
  - this will print the struct using a Debug output format
- To use this, we need to provide a Debug implementation for our struct
  - We can opt into this by decorating our struct with `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- `{:?}` prints all the fields inline which isn't great for large structs

  - to improve this, we can use `{:#?}` instead: `println!("rect1 is {:#?}", rect1);`
  - this will print the fields vertically

- Instead of using `println!` we can also use the `dbg!` macro
  - prints to the error console stream instead of standard output

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! returns ownership so can inline in computed properties
        height: 50,
    };

    dbg!(&rect1); // need to pass in a reference here so dbg! doesn't take ownership of the instance
}
```

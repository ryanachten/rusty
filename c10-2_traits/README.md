# Rusty

## Traits

- A _trait_ defines functionality a particular type has and can share with other types
- can use traits to define shared behaviour in an abstract way
- _trait bounds_ are used to specify a generic types that have certain behaviour

**Note** traits are similar to _interfaces_ in other languages, but with some differences

### Defining a trait

- Trait definitions are a way to group method signatures together to define a set of behaviours

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

- the compiler will enforce that any type with the `Summary` trait will have the method `summarize` exactly

### Implementing a trait on a type

- Once we've defined the trait, we can implement it for different structs
- similar to defining normal methods for structs, except we use the `for` keyword to indicating what trait we want to implement for a struct

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

**Note** when using traits, either the trait or the type it is being implemented on must be local to our crate

- can't implement external traits on external types
- this rule is part of a property called _coherence_, specifically the _orphan rule_
  - where the parent type is not present
  - rule ensures that other people's code can't break your code (and vice versa)
  - otherwise two crates could implement the same trait, and Rust wouldn't know which to use

### Default implementations

- Sometimes its useful to have a default implementation instead of requiring all implementations for all methods all the time
  - kinda like abstract classes in OOP
- these can be kept or overridden if requirements differ

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

- default implementations can call other methods in the same trait
  - even other methods w/o a default implementation

### Traits as parameters

- We can use traits to define functions that accept many types
  - i.e. polymorphism
- To do this, we can use the `impl Trait` syntax to specify that the parameter must have implemented a given trait
  - we can then call any of the trait methods in the method

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait bound syntax

- The `impl Trait` syntax works for straightforward cases, but it's actually syntax sugar for a longer form called _trait bound_ syntax

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- the bound syntax can be used for more complex cases
- i.e. `pub fn notify<T: Summary>(item1: &T, item2: &T)` will ensure `T` is the same concrete type (can't do this with `impl Trait`)

### Specifying multiple trait bounds with the `+` syntax

- We can specify multiple trait bounds where an item must have implemented all of the specified traits to be able to be passed as the parameter
  - i.e. using the impl trait syntax: `pub fn notify(item: &(impl Summary + Display)) {`
  - i.e. using the trait bound syntax: `pub fn notify<T: Summary + Display>(item: &T) {`

### Clearer trait bonds with `where` clauses

- Having numerous generic parameters with trait bounds can create function signatures which are hard to read
- We can simplify this by using a `where` clause

i.e. this
`fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {`

becomes:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

### Returning types that implement traits

- We can also use the `impl Trait` syntax to indicate that the return type of a method implements a given trait
  i.e.

```rust
fn returns_summarizable() -> impl Summary {
  // return a type implementing Summary
}
```

- being able to specify the return type by only the trait it implements is especially useful in the context of closures and iterators
  - these create types that only the compiler knows or types which are very long to specify
  - using the `impl Trait` syntax, we can refer to these much more concisely
- we can only use the `impl Trait` syntax if we're returning a single type
  - a workaround for this is discussed in Chapter 17

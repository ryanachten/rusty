# Rusty

## Generic data types

### Performance of code using generics

- Using generics in Rust, won't make your program run any slower than with concrete types
- Rust achieves this using _monomorphization_
  - process of turning generic code into specific code by filling in concrete types at compile time
  - compiler looks at all the types where generic code is used and generates concrete code for these instances
- This is a form of _static dispatch_ where polymorphism is fully resolved at compile time
  - Compared to _dynamic dispatch_ which is resolved at runtime

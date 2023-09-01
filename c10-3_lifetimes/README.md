# Rusty

## Validating references with lifetimes

- instead of verifying that a types has the behaviour we want
- _lifetimes_ ensure that references are valid as long as we need them to be

- every reference in Rust has a lifetime

  - scope for which a reference is valid
  - most of the time, these are implicit and inferred
  - annotating lifetimes is not even a concept in most programming languages

- similar to types, where we only annotate types when multiple types are possible
  - we only annotate lifetimes when lifetimes of references could be related in different ways
  - Rust requires us to annotate relationships using generic lifetime parameters to ensure the references used at runtime will be valid

### Lifetime annotation syntax

- Lifetime annotations don't change how long references live
  - they just describe the relationships of lifetimes of multiple references
- Functions can accept generic lifetime parameters for references
  - similar to how we declare generic time parameters
- Lifetime annotations are prefixed with a `'` i.e. `'a`
  - by convention, the first lifetime is annotated with `'a`
  - is placed after the `&` of a reference, i.e. `&'a i32`

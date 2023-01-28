# Rusty

## Cargo

Cargo is Rust's build system and package manager. Cargo is typically used to manage Rust projects as it handles a lot of tasks for us. We can initialise a new Cargo project via `cargo new project_name`.

Cargo uses a `.toml` file to define our project configuration and dependencies.

Cargo expects the source files to live inside the `/src` directory.

To build a Cargo project, we run `cargo build` inside the project directory. This will produce the following:

- `cargo.lock` - keeps track of exact dependency versions (like `package.json`)
- `target/*` - binary and debug files
  We can build our project for release using the `--release` flag

We can then run the project via `cargo run`. We don't actually need to run `cargo build` before running this.

We can run `cargo check` to ensure the code compiles w/o building the executables. This is faster the doing a proper build, so useful while developing to ensure we haven't introduced breaking changes.

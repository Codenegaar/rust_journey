# Cargo Playground
In this example project, I want to test features of cargo and see how a project
can be organized.

The concepts practiced in this project, and the associated resources are provided below.

## Workspaces
The project is actually a cargo workspace. Refer to the root `Cargo.toml` file to learn more.

## Libraries
A library is created, named `codar`.

`cargo new --lib codar`

This library is added to the root `Cargo.toml` file as a member of the workspace as well.

## Modules
A module can be created either in a file named after the module (e.g `math.rs`) 
or in a directory (e.g `math`) with a root source file, named `mod.rs`.

[Learn more about modules, visibility, etc](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

## Testing
### Unit tests
Unit tests for the module `math` are defined inside a sub-module called `tests` (as a convention).

### Integration tests
Integration tests are defined in a file inside `tests` directory.

### Running tests
Simply run the following: `cargo test`. Note that documentation tests are executed as well.

[Read more about testing in Rust](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

## Documentation
Documentation is done by using special comment indicators, `///`.
You can find these documentations in `src/math/simple.rs` and `src/math/advanced.rs`.

To generate documentations, run `cargo doc` and look at `target/doc/codar/index.html`.

[Learn more about documentation](https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html)
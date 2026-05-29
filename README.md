# rinha-compilers

> “Good compilers take years to mature.”
>
> Thankfully, this implementation arrived only 3 years after the original
'Rinha de Compilers'.

So, formally, this is my implementation for the 'Rinha de Compilers' using Rust
with a Tree-Walking Interpreter architecture.

This project was built to explore:
* parsing and AST construction;
* expression evaluation;
* scopes handling;
* functions and closures;
* runtime/value representation in Rust;
* future optimizations and some experimentation.

## Running
### Requirements
* Rust and Cargo version 1.93.1 (the one I used during implementation)

### Build and Run
```bash
cargo build --release
cargo run --release <path_to_program_ast.json>
```

## Contributing
Feel free to open issues or submit pull requests if you'd like to contribute to this project.

## License
This project is licensed under the [MIT License](LICENSE).

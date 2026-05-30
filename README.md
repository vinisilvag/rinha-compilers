# rinha-compilers

> “Good compilers take years to mature.”
>
> Thankfully, this implementation arrived only 3 years after the original
'Rinha de Compilers'.

So, formally, this is my implementation for the [Rinha de Compilers](https://github.com/aripiprazole/rinha-de-compiler)
using Rust with a Tree-Walking Interpreter architecture.

This project was built to explore:
* parsing and AST construction;
* expression evaluation;
* scopes handling;
* functions and closures;
* runtime/value representation in Rust;
* future optimizations and some experimentation.

## Running
### Requirements
* Rust and Cargo version 1.96 (the one I used during implementation)

### Build and Run
```bash
cargo build --release
cargo run --release <path_to_program_ast.json>
```

### Docker
```bash
# Build Docker image
docker build -t rinha-compilers .

# Run
docker run --rm -it \
  -v <path_to_some_input_file.json>:/var/rinha/source.rinha.json \
  rinha-compilers
```

## Roadmap
* Better specified errors (with the `location` information on the AST)
* Some optimizations (do some profilling with `flamegraph` first):
  * Unnecessary `.clone()` (use `Rc<..>`)
  * Ensure `env` is getting passed through reference everywhere
  * `str` instead of `String`
  * Memoization/caching

## Contributing
Feel free to open issues or submit pull requests if you'd like to contribute to this project.

## License
This project is licensed under the [MIT License](LICENSE).

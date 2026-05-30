FROM rust:1.96-slim AS build

RUN cargo new --bin rinha-compilers
WORKDIR /rinha-compilers

COPY Cargo.toml /rinha-compilers/
COPY Cargo.lock /rinha-compilers/
RUN cargo build --release

COPY src /rinha-compilers/src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=build /rinha-compilers/target/release/rinha-compilers /rinha-compilers

CMD ["/rinha-compilers", "/var/rinha/source.rinha.json"]

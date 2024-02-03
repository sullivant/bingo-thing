FROM rust:1.75.0-slim as builder

# 1. Create a new empty shell project
WORKDIR /usr/src/
RUN USER=root cargo new --bin service-ball
WORKDIR /usr/src/service-ball

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/service_ball*
RUN cargo install --path .

# # Final image prep for a smaller size
FROM rust:1.75.0-slim as release
COPY --from=builder /usr/local/cargo/bin/service-ball .
USER 1000
EXPOSE 8000

CMD ["./service-ball"]
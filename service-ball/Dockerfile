FROM rust:1.72.0 as builder

# 1. Create a new empty shell project
RUN USER=root cargo new --bin service-ball
WORKDIR /service-ball

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
RUN cargo build -vv --release

# # Final image prep for a smaller size
# FROM debian:buster-slim
# COPY --from=builder /service-ball/target/release/service-ball .

CMD ["./target/release/service-ball"]
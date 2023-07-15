# Build image
FROM rustlang/rust:nightly as builder

# Run dummy build to build and cache dependencies that only depends on Cargo.toml and Cargo.lock
WORKDIR /usr/src
RUN USER=root cargo new rust-warp-study
COPY Cargo.toml Cargo.lock /usr/src/rust-warp-study/
WORKDIR /usr/src/rust-warp-study
RUN cargo build --release

# Run actual build
COPY ./src ./src
RUN cargo build --release

# Run image
FROM debian:bullseye-slim
RUN apt-get update
RUN apt-get -y install libpq-dev
RUN apt-get -y install libssl1.1
RUN apt-get install -y --no-install-recommends ca-certificates


# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder  /usr/src/rust-warp-study/target/release/rust-warp-study /usr/local/bin/rust-warp-study

WORKDIR /usr/rust-warp-study
COPY ./.env ./.env
COPY ./static ./static
CMD ["rust-warp-study"]
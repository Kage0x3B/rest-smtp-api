FROM rust:1.81-slim-bullseye AS build

# Start from new project and copy only Cargo.toml to build and cache dependencies
RUN USER=root cargo new --bin rest-smtp-api
WORKDIR /rest-smtp-api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Remove build artifacts from initial hello world project used to cache deps
RUN rm ./target/release/deps/rest_smtp_api*
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /rest-smtp-api

RUN apt-get update \
  && apt-get install -y wget \
  && apt-get clean

COPY --from=build /rest-smtp-api/target/release/rest-smtp-api .

CMD ["./rest-smtp-api"]
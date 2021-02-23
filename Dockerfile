# syntax = docker/dockerfile:1.2

FROM docker.io/rustlang/rust:nightly-buster as builder

WORKDIR /build

RUN --mount=type=cache,target=/var/cache/apt \
  apt-get update && \
  apt-get install -yqq --no-install-recommends \
  clang \
  lld \
  libasound2-dev \
  libudev-dev

COPY . .

RUN --mount=type=cache,target=~/.cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release -p senior_game_server && \
    cargo install --path /build/senior_game_server --bin senior_game_server --verbose

FROM docker.io/frolvlad/alpine-glibc:alpine-3.13

RUN apk add --no-cache libstdc++

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/senior_game_server /app
COPY /senior_game_server/assets /app/assets

CMD ["/app/senior_game_server"]

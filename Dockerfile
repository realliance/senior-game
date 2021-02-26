FROM docker.io/rustlang/rust:nightly-buster as builder
WORKDIR /build
ENV CARGO_HOME /build/cargo

RUN --mount=type=cache,target=/var/cache/apt \
  apt-get update && \
  apt-get install -yqq --no-install-recommends \
  clang lld libasound2-dev libudev-dev

COPY . .

RUN --mount=type=cache,target=/build/cargo \
    --mount=type=cache,target=/build/target \
    cargo install --locked --root install \
    --path senior_game_server --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc-debian10
ARG RELEASE
WORKDIR /app
ENV RELEASE $RELEASE

COPY --from=builder /build/install/bin/senior_game_server /app
COPY senior_game_server/assets /app/assets

EXPOSE 12350
CMD ["/app/senior_game_server"]

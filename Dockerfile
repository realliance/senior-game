FROM docker.io/rustlang/rust:nightly-buster@sha256:d24db080a4d95cb7c1ee2156e3edd0e3e1c8b2d8898443ffd8a62f8bb7b88392 as builder
WORKDIR /build
ENV CARGO_HOME /build/cargo

RUN --mount=type=cache,target=/var/cache/apt \
  apt-get update -y && \
  apt-get install -y --no-install-recommends \
  clang lld libasound2-dev libudev-dev cmake

COPY . .

RUN --mount=type=cache,target=/build/cargo \
    --mount=type=cache,target=/build/target \
    cargo install --locked --root install \
    --path senior_game_server --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc-debian10@sha256:5b477f148457a90597954c7d167f9f69f75b3f5706225047cd4cbd969ce7a653
ARG RELEASE
WORKDIR /app
ENV RELEASE $RELEASE

COPY --from=builder /build/install/bin/senior_game_server /app
COPY senior_game_server/assets /app/assets

EXPOSE 12350
CMD ["/app/senior_game_server"]

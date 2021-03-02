FROM docker.io/rustlang/rust:nightly-buster@sha256:39de5fa42f3ee841ea47c1872e8b8b9813d83b246b0525e20245cc43aaae85da as builder
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

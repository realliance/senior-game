FROM docker.io/rustlang/rust:nightly-buster@sha256:6181b1d07869e88f5d17e681ab34a89e6b38d324c5b27cb92672c7b277c66812 as builder
WORKDIR /build
ENV CARGO_HOME /build/cargo

RUN --mount=type=cache,target=/var/cache/apt \
  apt-get update -y && \
  apt-get install -y --no-install-recommends \
  clang lld libasound2-dev libudev-dev libxcb-xfixes0-dev libxcb-shape0-dev cmake

COPY . .

RUN --mount=type=cache,target=/build/cargo \
    --mount=type=cache,target=/build/target \
    cargo install --locked --root install \
    --path senior_game_server --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc-debian10@sha256:98b6961beae0c8f65a913eaccb1e43a227a1ed6e2dbe61150963a4952d1c4635
ARG RELEASE
WORKDIR /app
ENV RELEASE $RELEASE

COPY --from=builder /build/install/bin/senior_game_server /app
COPY senior_game_server/assets /app/assets

EXPOSE 12350
CMD ["/app/senior_game_server"]

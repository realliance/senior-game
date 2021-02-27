#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install \
--path senior_game_client --target x86_64-unknown-linux-gnu

mkdir -p linux
cp install/bin/senior_game_client linux/WizardConnect3
cp -r senior_game_client/assets linux

cd linux
mkdir -p ../clients/$VERSION
zip -r ../clients/$VERSION/linux.zip .

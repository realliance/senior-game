#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install \
--path senior_game_client --target x86_64-unknown-linux-gnu

mkdir -p linux
cp install/bin/senior_game_client linux/WizardConnect3
cp -r senior_game_client/assets linux

cd linux
zip -r ../clients//linux-$VERSION.zip .

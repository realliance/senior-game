#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install \
--path senior_game_client --target x86_64-pc-windows-gnu

mkdir -p windows
cp install/bin/senior_game_client.exe windows/WizardConnect3.exe
cp -r senior_game_client/assets windows
cp /usr/x86_64-w64-mingw32/bin/{libstdc++-6.dll,libgcc_s_seh-1.dll,libwinpthread-1.dll} windows

cd windows
mkdir -p ../clients/$VERSION
zip -r ../clients/$VERSION/windows.zip .

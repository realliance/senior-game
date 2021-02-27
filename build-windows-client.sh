#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install \
--path senior_game_client --target x86_64-pc-windows-gnu

mkdir -p windows
cp install/bin/senior_game_client.exe windows/WizardConnect3.exe
cp -r senior_game_client/assets windows
cp /usr/lib/gcc/x86_64-w64-mingw32/8.3-posix/libstdc++-6.dll windows
cp /usr/lib/gcc/x86_64-w64-mingw32/8.3-posix/libgcc_s_seh-1.dll windows
cp /usr/x86_64-w64-mingw32/lib/libwinpthread-1.dll windows

cd windows
mkdir -p ../clients/$VERSION
zip -r ../clients/$VERSION/windows.zip .

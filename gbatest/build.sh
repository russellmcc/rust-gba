#!/bin/bash
projname=gbatest
dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

set -e
export RUST_TARGET_PATH=$dir

for i in "$@"
do
case $i in
    --release)
    release=true
    ;;
    *)
            # unknown option
    ;;
esac
done

if [ "$release" = true ]; then
    out_dir=$dir/target/gameboy-advance/release
else
    out_dir=$dir/target/gameboy-advance/debug
fi

xargo build --target gameboy-advance "$@"
gba-make-cartridge $out_dir/$projname -o $out_dir/"$projname".gba
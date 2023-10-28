#!/usr/bin/env bash
# adapted from: https://github.com/rustwasm/wasm-pack/issues/313#issuecomment-441163923
set -e

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

# Clean previous packages
if [ -d "pkg" ]; then
    rm -rf pkg
fi

if [ -d "pkg-node" ]; then
    rm -rf pkg-node
fi

# Build for both targets
wasm-pack build -t nodejs -d pkg-node
wasm-pack build -t web -d pkg

# Get the package name
ROOT_DIR=$(dirname "$0")
PKG_DIR=$ROOT_DIR/pkg
PKG_DIR_NODE=$ROOT_DIR/pkg-node
PKG_NAME=$(jq -r .name $PKG_DIR/package.json | sed 's/\-/_/g')

rm -rf $PKG_DIR/node $PKG_DIR/web

# Merge nodejs & browser packages
mkdir $PKG_DIR/node $PKG_DIR/web
mv $PKG_DIR/$PKG_NAME* $PKG_DIR/web
mv $PKG_DIR_NODE/$PKG_NAME* $PKG_DIR/node

# Add node, web folders to files
jq ".files = [\"node\", \"web\"]" $PKG_DIR/package.json \
| jq ".main = \"node/${PKG_NAME}.js\"" \
| jq ".browser = \"web/${PKG_NAME}.js\"" \
| jq "del(.module)" | jq "del(.types)" > $PKG_DIR/temp.json

mv $PKG_DIR/temp.json $PKG_DIR/package.json

rm -rf $PKG_DIR_NODE
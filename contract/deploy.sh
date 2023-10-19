#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
# use flag --force to always create new dev acoount to deploy to
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/the_cyber_caffe.wasm

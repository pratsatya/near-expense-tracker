# !/usr/bin/env bash

echo --------------------------------------------
echo --------------------------------------------
echo "Running test scripts"
echo --------------------------------------------
echo --------------------------------------------

# exit on first error after this point to avoid redeploying with successful build
set -e

# yarn install
# echo --------------------------------------------
echo
echo "install dependencies"
echo
yarn install --check-files

# yarn test
echo --------------------------------------------
echo
echo "build the contract (test build)"
echo
yarn test

exit 0


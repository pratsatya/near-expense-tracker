# !/usr/bin/env bash

echo --------------------------------------------
echo --------------------------------------------
echo "Deploy contract"
echo --------------------------------------------
echo --------------------------------------------

# delete files from neardev, if exists
echo
echo "cleaning up the /neardev folder"
echo
rm -rf ../neardev/

# exit on first error after this point to avoid redeploying with successful build
set -e

# yarn deploy
echo
echo "build the contract and dev-deploy"
echo
yarn deploy

# unset existing env var, if present
unset CONTRACT_ADD
echo $CONTRACT_ADD

# prompt user to set env variables
echo --------------------------------------------
echo "your contract_id is: "
cat ../neardev/dev-account.env
echo
echo
echo "run the following commands"
echo 'export CONTRACT_ADD=__enter_contract_id_here__'
echo

exit 0

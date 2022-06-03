# ⋈ NEAR Expense Tracker v1.0

This repository is a blockchain implementation of an expense tracker.

The current version can be easliy deployed on the testnet and implements the basic functionalities of how a trip expense tracker should work.

v2.0, when launched will contain advanced expense management methods, frontend integration, expense settlement, and push to mainnet.

See the [Idea whiteboard](NCD-Demo-Idea.pdf)
<br /><br />

## Usage
---
### Prerequisites
* Current version of [node.js](https://nodejs.org/)
* Make sure [Rust](https://www.rust-lang.org/) is installed for smart contract

### Getting started
1. Clone repo to a local folder
2. run `yarn`
3. run `yarn test`

### Top-level `yarn` commands
* run `yarn` to install all project dependencies
* run `yarn build` to quickly verify build status
* run `yarn deploy` to build, and dev-deploy
* run `yarn test` to run all tests
<br /><br />

## Project Flow and Methods
---
Below is a list of methods to interact with the smart contract.
* Setup
```
#install dependencies
yarn

#run tests
yarn test

#dev-deploy
yarn deploy
```

* Environment and Contract Init
```
#export account-id from dev-account.env
export CONTRACT_ADD=

echo $CONTRACT_ADD

#init contract
near call $CONTRACT_ADD new --accountId $CONTRACT_ADD
```

* Example Commands
```
#add a trip
near call $CONTRACT_ADD add_trip '{"trip_metadata":{"trip_name":"trip test1"}}' --accountId $CONTRACT_ADD --deposit 1

#add trip members
##trip should exist
near call $CONTRACT_ADD add_trip_members '{"trip_id":"1","new_members":["a.testnet","b.testnet"]}' --accountId $CONTRACT_ADD --deposit 1

#add an expense in a trip
#trip should exist
#lender and ower must be members in the trip
#lender cannot be same as ower
#amount in yocto
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 1","ower_id":"a.testnet","lender_id":"b.testnet","loan_amount":10000000000000000000000}' --accountId $CONTRACT_ADD --deposit 1

#update an expense
#trip should exist
#expense id should exist
#lender and ower must be members in the trip
#lender cannot be same as ower
#only the expense lender can update
near call $CONTRACT_ADD update_trip_expense '{"trip_id":"1","expense_id":"1","ower_id":"a.testnet","lender_id":"b","loan_amount":50000000000000000000000}' --accountId b.testnet --deposit 1

#delete an expense
#trip should exist
#expense id should exist
#only the expense lender can update
near call $CONTRACT_ADD delete_trip_expense '{"trip_id":"1","expense_id":"1"}' --accountId b.testnet --deposit 1

#get all expenses summary in a trip for an account id
#trip should exist
#trip should have atleast an expense
#account id must be a member of the trip
near call $CONTRACT_ADD get_expense_summary_by_trip_id_account_id '{"trip_id":"1","account_id":"a.testnet"}' --accountId $CONTRACT_ADD --deposit 1



# view a trip metadata
near view $CONTRACT_ADD view_trip_metadata_by_trip_id '{"trip_id":"3"}'

# view all trips an account id is in
near view $CONTRACT_ADD view_trip_id_by_account_id '{"account_id":"a.testnet"}'

# view all expense ids in a trip
near view $CONTRACT_ADD view_trip_expense_ids_by_trip_id '{"trip_id":"1"}'

# view an expense detail in a trip
near view $CONTRACT_ADD view_trip_expense_by_expense_id '{"trip_id":"1","expense_id":"1"}'
```
<br /><br />

## Get in touch
---
Please drop a [mail](mailto:prathamesh.satya@gmail.com) in case you have any suggestions/feedback, or are interested in further collaborative development.

Or [DM](https://twitter.com/pratsatya) on Twitter.






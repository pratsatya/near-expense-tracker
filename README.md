# ⋈ NEAR Expense Tracker v1.0

This repository is a blockchain implementation of an expense tracker.

The current version can be easliy deployed on the testnet and implements the basic functionalities of how a trip expense tracker should work.

v2.0, when launched will contain advanced expense management methods, frontend integration, expense settlement, and push to mainnet.

See the [Idea whiteboard](NCD-Demo-Idea.pdf).
<br /><br />

## Usage
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

Go to [scripts](scripts/) to setup and interact with the contract

Watch the [loom]() to know more.
<br /><br />

## Project Basics and Methods
### Project basics
The project has the following features:
* A user needs to create a trip before adding any expense
* Once a trip is created, the trip members can add expenses in the trip
* Only trip members can be a part of an expense
* Trip members can add new members in the trip
* An expense can only be updated/deleted by the lender
* Expense amount should be in NEAR (eg. 1 in params means 1 NEAR)
* Anyone can view any view methods in the contract
<br /><br />

### The file system
Please note that boilerplate project configuration files have been ommitted from the following lists for simplicity.
```
near-expense-tracker
├── contract                       <-- contract
│   ├── src
│   │   ├── lib.rs                                  <-- contract code
│   │── target                                      <-- outputs generated when yarn build
│   │── Cargo.toml                                  <-- cargo config file
│── neardev                        <-- env file with dev-account id when yarn deploy
│── out                            <-- output wasm deployed on blockchain
│── scripts                        <-- helper scripts to instantly run the project
│   │── 1_tests.sh                                  <-- script to run contract tests
│   │── 2_contract_deploy.sh                        <-- script to build and deploy
│   │── 3_contract_init.sh                          <-- script to init contract
│   │── 4_demo_commands.sh                          <-- script to run all contract methods
│── Getting-Started.loom           <-- loom video demo
│── NCD_Demo-Idea.pdf              <-- idea whiteboard pdf
│── package.json                   <-- json config file for yarn commands
│── README.md                       
```
<br /><br />

### Project methods
Below is a list of methods to interact with the smart contract.
* Setup
```Command
#install dependencies
yarn

#run tests
yarn test

#dev-deploy
yarn deploy
```

* Environment and Contract Init
```Command
#export account-id from dev-account.env
export CONTRACT_ADD=

echo $CONTRACT_ADD

#init contract
near call $CONTRACT_ADD new --accountId $CONTRACT_ADD
```

* Example Commands
```Command
#add a trip
near call $CONTRACT_ADD add_trip '{"trip_metadata":{"trip_name":"trip test1"}}' --accountId $CONTRACT_ADD --deposit 1

#add trip members
##trip should exist
near call $CONTRACT_ADD add_trip_members '{"trip_id":"1","new_members":["a.testnet","b.testnet"]}' --accountId $CONTRACT_ADD --deposit 1

#add an expense in a trip
#trip should exist
#lender and ower must be members in the trip
#lender cannot be same as ower
#amount in NEAR
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 1","ower_id":"a.testnet","lender_id":"b.testnet","loan_amount":10}' --accountId $CONTRACT_ADD --deposit 1

#update an expense
#trip should exist
#expense id should exist
#lender and ower must be members in the trip
#lender cannot be same as ower
#only the expense lender can update
near call $CONTRACT_ADD update_trip_expense '{"trip_id":"1","expense_id":"1","ower_id":"a.testnet","lender_id":"b","loan_amount":50}' --accountId b.testnet --deposit 1

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
<br />

## Running Shell Scripts
The scripts in [this](scripts/) folder support a simple demonstration of the contract. 

1. Run the script: [1_tests.sh](scripts/1_tests.sh) to install npm dependencies and run the test build
2. Run the script: [2_contract_deploy.sh](scripts/2_contract_deploy.sh) to build wasm and dev deploy
3. Run the command: `export CONTRACT_ADD=__enter_contract_id_here__` . The contract id can be copied from the terminal
4. Run the script: [3_contract_init.sh](scripts/3_contract_init.sh) to initialize the contract
5. Replace all the placeholder dev account ids in the script [4_demo_commands.sh](scripts/4_demo_commands.sh) with CONTRACT_ADD value
6. Run the script: [4_demo_commands.sh](scripts/4_demo_commands.sh) to see a demo implementation the contract methods
<br /><br />

## Get In Touch
Please drop a [mail](mailto:prathamesh.satya@gmail.com) in case you have any suggestions/feedback, or are interested in further collaborative development.

Or [DM](https://twitter.com/pratsatya) on Twitter.






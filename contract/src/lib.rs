//! This contract implements an expense tracker on the NEAR blockchain

// use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, serde_json::json, PanicOnDefault, env, BorshStorageKey, Balance, Promise};
use near_sdk::AccountId;

pub type TripId = String;
pub type TripIds = Vec<TripId>;
pub type ExpenseId = String;
pub type TripExpenses = UnorderedMap<ExpenseId,Expense>;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TripMetadata {
    trip_id: Option<TripId>,
    trip_name: Option<String>,
    trip_members: Option<Vec<AccountId>>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Expense {
    expense_id: Option<ExpenseId>,
    expense_name: Option<String>,
    ower_id: AccountId,
    lender_id: AccountId,
    loan_amount: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExpenseStats {
    trip_id: Option<TripId>,
    trip_name: Option<String>,
    expense_acc_key: Vec<AccountId>,
    expense_amt_value: Vec<i128>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    TripIdsByAccountId,
    TripMetadataByTripId,
    TripExpensesByTripId,
    TripExpenseByExpenseId {key_expense_by_expense_id: u32},
    TripExpenseByAccountId {key_expense_by_account_id: u32},
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct ExpenseTracker {
    trip_id_by_account_id: LookupMap<AccountId,TripIds>,
    trip_metadata_by_trip_id: UnorderedMap<TripId,TripMetadata>,
    trip_expenses_by_trip_id: UnorderedMap<TripId,TripExpenses>,
    storagekey_counter: u32,
}

#[near_bindgen]
impl ExpenseTracker {

    // constructor methods *****************

    // init contract
    #[init]
    pub fn new(
    ) -> Self{
        assert!(!env::state_exists(), "Already initialized");
        Self{
            trip_id_by_account_id: LookupMap::new(StorageKey::TripIdsByAccountId),
            trip_metadata_by_trip_id: UnorderedMap::new(StorageKey::TripMetadataByTripId),
            trip_expenses_by_trip_id: UnorderedMap::new(StorageKey::TripExpensesByTripId),
            storagekey_counter: 0,
        }
    }



    // call methods *****************

    // add a trip
    #[payable]
    pub fn add_trip(
        &mut self,
        trip_metadata: TripMetadata,
    ) -> TripMetadata {
        let initial_storage_usage = env::storage_usage();
        let owner_id = env::predecessor_account_id();

        //handle trip name
        let trip_name = trip_metadata.trip_name.clone();
        assert!(trip_name.is_some(), "trip title is required");

         //handle trip members
         let trip_members:Option<Vec<AccountId>>;
        //  if members are provided by user
         if trip_metadata.trip_members.is_some() {
            let mut trip_members_list:Vec<AccountId> = trip_metadata.trip_members.clone().unwrap();
            // add owner also if not present
            if trip_members_list.contains(&owner_id){
                trip_members = Some(trip_members_list);
            } else{
                trip_members_list.push(owner_id);
                trip_members = Some(trip_members_list);
            }
         } else {
            //  add only owner as member
            trip_members = Some(vec![owner_id]);
         }

        //increment trip id
        let trip_id:TripId = format!("{}", (self.trip_metadata_by_trip_id.len() + 1));
        //check trip id is unique
        assert!(
			self.trip_metadata_by_trip_id.get(&trip_id).is_none(),
			"trip_id alredy exits"
		);
        
        //insert trip metadata into contract
        self.trip_metadata_by_trip_id.insert(
			&trip_id,
			&TripMetadata {
                trip_id: Some(trip_id.clone()),
				trip_name: trip_name.clone(),
				trip_members: trip_members.clone(),
			},
		);

        //update trip_id_by_owner collection in contract
        let members_vec = trip_members.clone().unwrap();
        // iterate on member ids
        for i in &members_vec {
            let member_id: &AccountId = i;
            // read trip ids collection
                let trip_ids =  &mut self.trip_id_by_account_id.get(&member_id).unwrap_or_else(|| {
                    vec![] 
                });
                // add current trip id to account vector
                let current_trip_id = trip_id.clone();
                trip_ids.push(current_trip_id);
                // add trip id vector to account
                self.trip_id_by_account_id.insert(&member_id,&trip_ids);
                }

        //log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "add_trip",
					"params": {
                        "trip_id": trip_id,
						"trip_name": trip_name,
						"trip_members": trip_members,
					}
				})
			)
			.as_ref(),
		);

        //refund after storage costs
        refund_deposit(env::storage_usage() - initial_storage_usage);

        // return val
        TripMetadata{
            trip_id: Some(trip_id.clone()),
			trip_name: trip_name.clone(),
			trip_members: trip_members.clone(),
        }
    }


    // add trip members
    #[payable]
    pub fn add_trip_members(
        &mut self,
        trip_id: TripId,
        new_members: Option<Vec<AccountId>>,
    ) -> TripMetadata {
        let initial_storage_usage = env::storage_usage();
        let owner_id = env::predecessor_account_id();

        //check trip id exists
        let mut trip_metadata = self
			.trip_metadata_by_trip_id
			.get(&trip_id)
			.expect("trip_id doesn't exist");

        // check caller present is a trip member
        assert!(self.trip_id_by_account_id.get(&owner_id).expect("caller id is added in no trips").contains(&trip_id), "caller id not an existing trip member");

        ////handle new trip members
         if trip_metadata.trip_members.is_some() {
             // check if trip has some members already
            let mut trip_members_list:Vec<AccountId> = trip_metadata.trip_members.clone().expect("trip has no members");
            let old_members_vec = trip_members_list.clone();
            // check caller has provided new members to add
            let new_members_vec = new_members.clone().expect("no member ids provided by caller");


            // add new members into existing member list
            let trip_members:Option<Vec<AccountId>>;
            for i in new_members_vec.clone(){
                let member_id: AccountId = i;
                if trip_members_list.contains(&member_id){
                    continue;
                } else {
                trip_members_list.push(member_id);
                }
            }
            trip_members = Some(trip_members_list);

            // update trip metadata
            trip_metadata = TripMetadata{
                trip_id: trip_metadata.trip_id.clone(),
                trip_name: trip_metadata.trip_name.clone(),
                trip_members: trip_members.clone(),
            };

            //insert trip metadata into contract
            self.trip_metadata_by_trip_id.insert(
                &trip_id,
                &trip_metadata,
            );

            // //insert trip metadata into contract
            // self.trip_metadata_by_trip_id.insert(
            //     &trip_id,
            //     &TripMetadata {
            //         trip_id: trip_metadata.trip_id.clone(),
            //         trip_name: trip_metadata.trip_name.clone(),
            //         trip_members: trip_members.clone(),
            //     },
            // );

            // update trip_id_by_owner collection in contract
            let trip_members_vec = trip_members.clone().unwrap();
            for j in &trip_members_vec {
                let member_id: &AccountId = j;
                // get current trip ids collection for each account
                let trip_ids =  &mut self.trip_id_by_account_id.get(&member_id).unwrap_or_else(|| {
                        vec![] 
                    });
                // check if current trip is in the vector
                if trip_ids.contains(&trip_id){
                    continue;
                } else {
                    // add current trip id to account vector
                    let current_trip_id = trip_id.clone();
                    trip_ids.push(current_trip_id);
                    // add trip id vector to account
                    self.trip_id_by_account_id.insert(&member_id,&trip_ids);
                }
            }

            // log
            env::log_str(
                format!(
                    "{}",
                    json!({
                        "method type": "add_trip_members",
                        "params": {
                            "trip_id": trip_metadata.trip_id,
                            "trip_name": trip_metadata.trip_name,
                            "old_members": old_members_vec,
                            "new_members": new_members_vec,
                            "all_members": trip_metadata.trip_members,
                        }
                    })
                )
                .as_ref(),
            );
        }  

         //refund after storage costs
         refund_deposit(env::storage_usage() - initial_storage_usage);

         // return val
         trip_metadata
    }


    // add trip expense
    #[payable]
    pub fn add_trip_expense(
        &mut self,
        trip_id: TripId,
        expense_name: Option<String>,
        ower_id:AccountId,
        lender_id:AccountId,
        loan_amount:u128,
    ) -> Expense  {
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        //handle expense name
        assert!(expense_name.is_some(), "expense title is required");

        //check trip id exists
        assert!(self.trip_metadata_by_trip_id.get(&trip_id).is_some(),"trip_id doesn't exist");

        // check caller present is a trip member
        assert!(self.trip_id_by_account_id.get(&caller_id).expect("caller id is added in no trips").contains(&trip_id), "caller id not an existing trip member");

        // check lender id is a trip member
        assert!(self.trip_id_by_account_id.get(&lender_id).expect("lender id is added in no trips").contains(&trip_id), "lender not an existing trip member");

        // check ower id is a trip member
        assert!(self.trip_id_by_account_id.get(&ower_id).expect("ower id is added in no trips").contains(&trip_id), "ower not an existing trip member");

        // check lender is not ower
        assert_ne!(&lender_id,&ower_id,"lender and ower cannot be same");

        // generate storagekey if needed
        let mut storagekey = 0;
        if self.trip_expenses_by_trip_id.get(&trip_id).is_none(){
            storagekey = self.storagekey_counter + 1;
            self.storagekey_counter = storagekey.clone();
        }
        
        // get expense map for trip
        let mut expense_id_map = self.trip_expenses_by_trip_id.get(&trip_id).unwrap_or_else(|| {
            UnorderedMap::new(StorageKey::TripExpenseByExpenseId { key_expense_by_expense_id: storagekey })
        });
        //increment expense id
        let expense_id:ExpenseId = format!("{}", (expense_id_map.len() + 1));
        //check expense id is unique
        assert!(
			self.trip_expenses_by_trip_id.get(&trip_id).unwrap_or_else(|| UnorderedMap::new(StorageKey::TripExpenseByExpenseId { key_expense_by_expense_id: storagekey })).get(&expense_id).is_none(),
			"expense_id alredy exits"
		);

        //insert trip expense into unordered map
        expense_id_map.insert(
            &expense_id,
            &Expense {
                expense_id: Some(expense_id.clone()),
                expense_name: expense_name.clone(),
                ower_id: ower_id.clone(),
                lender_id: lender_id.clone(),
                loan_amount: loan_amount.clone(),
            },
        );

        //insert trip expense into contract
        self.trip_expenses_by_trip_id.insert(
            &trip_id,
            &expense_id_map,
        );


        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "add_trip_expense",
					"params": {
                        "trip_id": &trip_id,
                        "expense_id": &expense_id,
                        "expense_name": &expense_name,
                        "ower_id": ower_id,
                        "lender_id": lender_id,
                        "loan_amount": loan_amount.to_string(),
                        "storage_key": storagekey,
					}
				})
			)
			.as_ref(),
		);

        //refund after storage costs
        refund_deposit(env::storage_usage() - initial_storage_usage);

        // return val
        self.trip_expenses_by_trip_id.get(&trip_id).unwrap().get(&expense_id).unwrap()
    }


    // update trip expense
    #[payable]
    pub fn update_trip_expense(
        &mut self,
        trip_id: TripId,
        expense_id: ExpenseId,
        expense_name: Option<String>,
        ower_id:AccountId,
        lender_id:AccountId,
        loan_amount:u128,
    ) -> Expense {
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::predecessor_account_id();

        //check trip id exists
        assert!(self.trip_metadata_by_trip_id.get(&trip_id).is_some(),"trip_id doesn't exist");

        // check caller id is a trip member
        assert!(self.trip_id_by_account_id.get(&caller_id).expect("caller id is added in no trips").contains(&trip_id), "caller is not an existing trip member");

        // check lender id is a trip member
        assert!(self.trip_id_by_account_id.get(&lender_id).expect("lender id is added in no trips").contains(&trip_id), "lender not an existing trip member");

        // check ower id is a trip member
        assert!(self.trip_id_by_account_id.get(&ower_id).expect("ower id is added in no trips").contains(&trip_id), "ower not an existing trip member");

        // check lender is not ower
        assert_ne!(&lender_id,&ower_id,"lender and ower cannot be same");

        //check expense map exists, get
        let mut expense_map = self
			.trip_expenses_by_trip_id
			.get(&trip_id)
			.expect("trip doesn't have any expenses");
        
        // get expense from expense id
        assert!(expense_map.get(&expense_id).is_some(),"expense_id doesn't exist in trip");

        // check caller is also lender
        assert_eq!(caller_id,expense_map.get(&expense_id).unwrap().lender_id, "cannot edit expense since caller is not current lender");

        //handle expense name
        let final_expense_name = if expense_name.is_some(){
            expense_name
        } else {
            expense_map.get(&expense_id).unwrap().expense_name
        };

        // update expense
        expense_map.insert(
            &expense_id,
            &Expense {
                expense_id: Some(expense_id.clone()),
                expense_name: final_expense_name.clone(),
                ower_id: ower_id.clone(),
                lender_id: lender_id.clone(),
                loan_amount: loan_amount.clone(),
            },
        );

        //insert trip expense into contract
        self.trip_expenses_by_trip_id.insert(
            &trip_id,
            &expense_map,
        );


        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "update_trip_expense",
					"params": {
                        "trip_id": &trip_id,
                        "expense_id": &expense_id,
                        "expense_name": &final_expense_name,
                        "ower_id": ower_id,
                        "lender_id": lender_id,
                        "loan_amount": loan_amount.to_string(),
					}
				})
			)
			.as_ref(),
		);

        //refund after storage costs
        refund_deposit(env::storage_usage() - initial_storage_usage);

        // return val
        self.trip_expenses_by_trip_id.get(&trip_id).unwrap().get(&expense_id).unwrap()
    }


    // delete trip expense
    #[payable]
    pub fn delete_trip_expense(
        &mut self,
        trip_id: TripId,
        expense_id: ExpenseId,
    ) -> bool {
        let caller_id = env::predecessor_account_id();

        //check trip id exists
        assert!(self.trip_metadata_by_trip_id.get(&trip_id).is_some(),"trip_id doesn't exist");

        // check caller present is a trip member
        assert!(self.trip_id_by_account_id.get(&caller_id).expect("caller id is added in no trips").contains(&trip_id), "caller id not an existing trip member");

        //check expense map exists, get
        let mut expense_map = self
			.trip_expenses_by_trip_id
			.get(&trip_id)
			.expect("trip doesn't have any expenses");

        
        // get expense from expense id
        assert!(expense_map.get(&expense_id).is_some(),"expense_id doesn't exist in trip");

        // check caller is also lender
        assert_eq!(caller_id,expense_map.get(&expense_id).unwrap().lender_id, "cannot delete expense since caller is not lender");

        // delete expense
        expense_map.remove(&expense_id);

        //insert trip expense into contract
        self.trip_expenses_by_trip_id.insert(
            &trip_id,
            &expense_map,
        );


        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "delete_trip_expense",
					"params": {
                        "trip_id": &trip_id,
                        "expense_id": &expense_id,
					}
				})
			)
			.as_ref(),
		);

        true
    }


    // get expense stats by trip id
    #[payable]
    pub fn get_expense_summary_by_trip_id_account_id(
        &mut self,
        trip_id: TripId,
        account_id: AccountId,
    ) -> ExpenseStats {
        let initial_storage_usage = env::storage_usage();

        //check trip id exists
        let trip_metadata = self
			.trip_metadata_by_trip_id
			.get(&trip_id)
			.expect("trip_id doesn't exist");

        // check account_id is a trip member
        assert!(self.trip_id_by_account_id.get(&account_id).expect("account id is added in no trips").contains(&trip_id), "account id not an existing trip member");

        //check expense map exists
        let expense_map = self
			.trip_expenses_by_trip_id
			.get(&trip_id)
			.expect("trip doesn't have any expenses");

        // get members list in trip
        let trip_members_list = trip_metadata.trip_members.unwrap();

        // generate storagekey if needed
        let storagekey = self.storagekey_counter + 1;
        self.storagekey_counter = storagekey.clone();

        // create unordered map of all ids w.r.t account id
        let mut map_of_accounts:UnorderedMap<AccountId,Vec<i128>> = UnorderedMap::new(StorageKey::TripExpenseByAccountId { key_expense_by_account_id: storagekey });
        // iterate
        if trip_members_list.len() > 1 {
            // iterate on other account ids
            for i in trip_members_list.clone(){
                let member_id = i;
                if member_id == account_id{
                    // continue;
                } else {
                    // initialize vector
                    let expense_vec:Vec<i128> = vec![0];
                    // add to map
                    map_of_accounts.insert(
                        &member_id,
                        &expense_vec,
                    );
                }
            }
        } else {
            panic!("no other account present in trip");
        }

        // iterate over expenses and store in unordered map
            for  (_k, v) in expense_map.iter() {
                // if account is lender
                if v.lender_id == account_id {
                    // get other id from map of accounts
                    let mut vec_from_map_of_accounts = map_of_accounts.get(&v.ower_id).unwrap();
                    // get amount from expense map
                    let amt = v.loan_amount as i128;
                    // push into vec
                    vec_from_map_of_accounts.push(amt);
                    // push into map
                    map_of_accounts.insert(
                        &v.ower_id,
                        &vec_from_map_of_accounts,
                    );
                } else if v.ower_id == account_id {
                    // get other id from map of accounts
                    let mut vec_from_map_of_accounts = map_of_accounts.get(&v.lender_id).unwrap();
                    // get amount from expense map
                    let amt = (v.loan_amount as i128) * -1 ;
                    // push into vec
                    vec_from_map_of_accounts.push(amt);
                    // push into map
                    map_of_accounts.insert(
                        &v.lender_id,
                        &vec_from_map_of_accounts,
                    );
                } else {
                continue;
            };
        }

        // get final struct from map of accounts
        let mut output = ExpenseStats {
            trip_id: Some(trip_id.clone()),
            trip_name: trip_metadata.trip_name,
            expense_acc_key:Vec::new(),
            expense_amt_value:Vec::new(),
        }; 
        // save values in output struct
        for (k, v) in map_of_accounts.iter() { 
            // net amount for account id key
            let net_amt:i128 = v.iter().sum();
            // add to vectors
            output.expense_acc_key.push(k);
            output.expense_amt_value.push(net_amt);
        };    


        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "get_expense_stats_by_trip_id",
					"params": {
                        "trip_id": trip_id,
                        "trip_name": &output.trip_name,
                        // "expense_acc_key": &output.expense_acc_key,
                        // "expense_amt_value": &output.expense_amt_value,
                        // "final storage": env::storage_usage(),
                        // "initial storage": initial_storage_usage,
					}
				})
			)
			.as_ref(),
		);

        //refund after storage costs
        refund_deposit(env::storage_usage() - initial_storage_usage);

        output
    }



    // view methods *****************

    // view trip metadata
    pub fn view_trip_metadata_by_trip_id(
        &self,
        trip_id: TripId,
    ) -> TripMetadata {
        //check trip id exists
        let trip_metadata = self
			.trip_metadata_by_trip_id
			.get(&trip_id)
			.expect("trip_id doesn't exist");

        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "view_trip_metadata_by_trip_id",
					"params": {
                        "trip_metadata": trip_metadata,
					}
				})
			)
			.as_ref(),
		);

        trip_metadata
    }


    // view trip ids by account id
    pub fn view_trip_id_by_account_id(
        &self,
        account_id: AccountId,
    ) -> TripIds {
        //check account id exists
        let trip_ids = self
			.trip_id_by_account_id
			.get(&account_id)
			.expect("account_id doesn't have any trips");

        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "view_trip_id_by_account_id",
					"params": {
                        "trip_ids": trip_ids,
					}
				})
			)
			.as_ref(),
		);

        trip_ids
    }


    // view trip expense ids by trip id
    pub fn view_trip_expense_ids_by_trip_id(
        &self,
        trip_id: TripId,
    ) -> Vec<ExpenseId> {
        //check trip id exists
        let trip_metadata = self
			.trip_metadata_by_trip_id
			.get(&trip_id)
			.expect("trip_id doesn't exist");

        //check expense map exists
        let expense_map = self
			.trip_expenses_by_trip_id
			.get(&trip_id)
			.expect("trip doesn't have any expenses");

        // create vector of expense ids
        let mut expense_id_vec: Vec<ExpenseId> = vec![];
            for  (k, _v) in expense_map.iter() {
                // println!("key={}, value={}", k, v);
                expense_id_vec.push(k.clone());
            }

        // sort expense_id_vec
        expense_id_vec.sort_by_key(|a| a.parse::<u32>().unwrap());

        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "view_trip_expense_ids_by_trip_id",
					"params": {
                        "trip_id": trip_id,
                        "trip_name": trip_metadata.trip_name,
                        "expense_ids": expense_id_vec,
					}
				})
			)
			.as_ref(),
		);

        expense_id_vec
    }


    // view trip expense by expense id
    pub fn view_trip_expense_by_expense_id(
        &self,
        trip_id: TripId,
        expense_id: ExpenseId,
    ) -> Expense {
        //check trip id exists
        let trip_metadata = self
			.trip_metadata_by_trip_id
			.get(&trip_id)
			.expect("trip_id doesn't exist");

        //check expense map exists
        let expense_map = self
			.trip_expenses_by_trip_id
			.get(&trip_id)
			.expect("trip doesn't have any expenses");


        // get expense from expense id
        let expense = expense_map.get(&expense_id).expect("expense_id doesn't exist in trip");


        // log
        env::log_str(
			format!(
				"{}",
				json!({
					"method type": "view_trip_expense_by_expense_id",
					"params": {
                        "trip_id": trip_id,
                        "trip_name": trip_metadata.trip_name,
                        "expense_id": expense.expense_id,
                        "expense_name": expense.expense_name,
                        "ower_id": expense.ower_id,
                        "lender_id": expense.lender_id,
                        "loan_amount": expense.loan_amount.to_string(),
					}
				})
			)
			.as_ref(),
		);

        expense
    }

}


pub fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
    // log!("refund_deposit amount {}", refund);
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}











/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts,VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id:ValidAccountId) -> VMContextBuilder {
        // initialize context
        let mut context_builder = VMContextBuilder::new();

        // VMContext { 
        //     current_account_id: (), 
        //     signer_account_id: (), 
        //     signer_account_pk: (), 
        //     predecessor_account_id: (), 
        //     input: (), 
        //     block_index: (), 
        //     block_timestamp: (), 
        //     epoch_height: (), 
        //     account_balance: (), 
        //     account_locked_balance: (), 
        //     storage_usage: (), 
        //     attached_deposit: (), 
        //     prepaid_gas: (), 
        //     random_seed: (), 
        //     view_config: (), 
        //     output_data_receivers: () 
        // }

        // setup with values
        context_builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);

        // return
        context_builder
    }

    #[test]
    // check new method runs correctly
    fn test_method_new() {
        // get context
        let mut context = get_context(accounts(1));
        // set testing env
        testing_env!(context); 
        let contract = ExpenseTracker::new();

        // c
        assert_eq!(env::current_account_id().to_string(), accounts(1).to_string());
    //     assert_eq!(1, contract.get_num());
    }





//     #[test]
//     fn decrement() {
//         let mut contract = Counter { val: 0 };
//         contract.decrement();
//         assert_eq!(-1, contract.get_num());
//     }

//     #[test]
//     fn increment_and_reset() {
//         let mut contract = Counter { val: 0 };
//         contract.increment();
//         contract.reset();
//         assert_eq!(0, contract.get_num());
//     }

//     #[test]
//     #[should_panic]
//     fn panics_on_overflow() {
//         let mut contract = Counter { val: 127 };
//         contract.increment();
//     }

//     #[test]
//     #[should_panic]
//     fn panics_on_underflow() {
//         let mut contract = Counter { val: -128 };
//         contract.decrement();
//     }
}

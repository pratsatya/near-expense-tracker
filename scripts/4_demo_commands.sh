echo --------------------------------------------
echo --------------------------------------------
echo "Step by step demo commands to interact with smart contract"

echo "!!!REMEMBER TO REPLACE dev-1654439673483-67675783849542 IN SCRIPT 4 WITH ENVIRONMENT VARIABLE'S VALUE IN COMMANDS 5 , 7 , 8 , 9 , 12 , 14 !!!"
echo --------------------------------------------
echo --------------------------------------------

[ -z "$CONTRACT_ADD" ] && echo "Missing \$CONTRACT_ADD environment variable"

set -e

echo
echo "contract id : " 
echo $CONTRACT_ADD

#1 add a trip// with other members
echo
echo --------------------------------------------
echo "add a trip// with other members"
echo --------------------------------------------
near call $CONTRACT_ADD add_trip '{"trip_metadata":{"trip_name":"trip test1","trip_members":["a.testnet","b.testnet"]}}' --accountId $CONTRACT_ADD --deposit 1

#2 view above trip's metadata
echo
echo --------------------------------------------
echo "view above trip's metadata"
echo --------------------------------------------
near view $CONTRACT_ADD view_trip_metadata_by_trip_id '{"trip_id":"1"}'

#3 add a trip// with no other member
echo
echo --------------------------------------------
echo "add a trip// with no other member"
echo --------------------------------------------
near call $CONTRACT_ADD add_trip '{"trip_metadata":{"trip_name":"trip test2"}}' --accountId $CONTRACT_ADD --deposit 1

#4 view above trip's metadata
echo
echo --------------------------------------------
echo "view above trip's metadata"
echo --------------------------------------------
near view $CONTRACT_ADD view_trip_metadata_by_trip_id '{"trip_id":"2"}'


#5 view all trips an account is present in
echo
echo --------------------------------------------
echo "view all trips an account is present in"
echo --------------------------------------------
near view $CONTRACT_ADD view_trip_id_by_account_id '{"account_id":"dev-1654439673483-67675783849542"}'

#6 add an expense in trip id 1
echo
echo --------------------------------------------
echo "add expense 1 in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 1","ower_id":"a.testnet","lender_id":"b.testnet","loan_amount":1}' --accountId $CONTRACT_ADD --deposit 1
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 1","ower_id":"a.testnet","lender_id":"b.testnet","loan_amount":10}' --accountId $CONTRACT_ADD --deposit 1
#7 add an expense in trip id 1
echo
echo --------------------------------------------
echo "add expense 2 in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 2","ower_id":"a.testnet","lender_id":"dev-1654439673483-67675783849542","loan_amount":2}' --accountId $CONTRACT_ADD --deposit 1

#8 add an expense in trip id 1
echo
echo --------------------------------------------
echo "add expense 3 in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 3","ower_id":"b.testnet","lender_id":"dev-1654439673483-67675783849542","loan_amount":5}' --accountId $CONTRACT_ADD --deposit 1

#9 add an expense in trip id 1
echo
echo --------------------------------------------
echo "add expense 4 in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD add_trip_expense '{"trip_id":"1","expense_name":"expense 4","ower_id":"dev-1654439673483-67675783849542","lender_id":"a.testnet","loan_amount":8}' --accountId $CONTRACT_ADD --deposit 1

#10 view expense 1 in trip id 1
echo
echo --------------------------------------------
echo "view expense 1 in trip id 1"
echo --------------------------------------------
near view $CONTRACT_ADD view_trip_expense_by_expense_id '{"trip_id":"1","expense_id":"1"}'

#11 view all expense ids in trip id 1
echo
echo --------------------------------------------
echo "view all expense ids in trip id 1"
echo --------------------------------------------
near view $CONTRACT_ADD view_trip_expense_ids_by_trip_id '{"trip_id":"1"}'

#12 update expense 2 in trip id 1
echo
echo --------------------------------------------
echo "update expense 2 in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD update_trip_expense '{"trip_id":"1","expense_id":"2","ower_id":"a.testnet","lender_id":"dev-1654439673483-67675783849542","loan_amount":4}' --accountId $CONTRACT_ADD --deposit 1

#13 delete expense 2 in trip id 1
echo
echo --------------------------------------------
echo "delete expense 2 in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD delete_trip_expense '{"trip_id":"1","expense_id":"2"}' --accountId $CONTRACT_ADD --deposit 1

#14 view expenses summary for account id in trip id 1
echo
echo --------------------------------------------
echo "view expenses summary for account id in trip id 1"
echo --------------------------------------------
near call $CONTRACT_ADD get_expense_summary_by_trip_id_account_id '{"trip_id":"1","account_id":"dev-1654439673483-67675783849542"}' --accountId $CONTRACT_ADD --deposit 1

exit 0

echo --------------------------------------------
echo --------------------------------------------
echo "Init contract"
echo --------------------------------------------
echo --------------------------------------------

[ -z "$CONTRACT_ADD" ] && echo "Missing \$CONTRACT_ADD environment variable"

set -e

echo
echo "contract id : " 
echo $CONTRACT_ADD

# init contract
echo
echo "contract init"
near call $CONTRACT_ADD new --accountId $CONTRACT_ADD

echo
echo "!!!REMEMBER TO REPLACE dev-1654439673483-67675783849542 IN SCRIPT 4 WITH ENVIRONMENT VARIABLE'S VALUE IN COMMANDS 5 , 7 , 8 , 9 , 12 , 14 !!!"

exit 0

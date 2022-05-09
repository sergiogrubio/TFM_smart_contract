######### CONTRACT_ADDRESS
export CONTRACT_ADDRESS=erd1qqqqqqqqqqqqqpgqv9kru8n83j240ry4fu82cgnz4mcr5m7ymq2qk0wtvk

######### DEPLOY
erdpy contract deploy --pem="~/wallet/wallet1.pem" --recall-nonce --gas-limit=100000000 --project=. --proxy="https://devnet-gateway.elrond.com" --chain="D" --arguments 0x05 --send

######### getLiquidity - Egld - UOC
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getLiquidityEgld --arguments 0x554f432d643133396262

######### getLiquidity - UOC-d139bb
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getLiquidityToken --arguments 0x554f432d643133396262

######### getLiquidity - Egld - Web
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getLiquidityEgld --arguments 0x5745422d356430386265

######### getLiquidity - Web
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getLiquidityToken --arguments 0x5745422d356430386265

######### getLiquidity - status
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=status --arguments 0x554f432d643133396262

######### calculateK
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=calculateK --arguments 0x554f432d643133396262

######### getFee
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getFee

######### priceEgldToken
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=priceEgldToken --arguments 0x554f432d643133396262 0x01

######### priceEgldTokenNoFee
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=priceEgldTokenNoFee --arguments 0x554f432d643133396262 0x01

######### feeEgldToken
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=feeEgldToken --arguments 0x554f432d643133396262 0x01

######### priceTokenEgld
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=priceTokenEgld --arguments 0x554f432d643133396262 0x01

######### priceTokenEgldNoFee
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=priceTokenEgldNoFee --arguments 0x554f432d643133396262 0x01

######### feeTokenEgld
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=feeTokenEgld --arguments 0x554f432d643133396262 0x01

######## getEarnings -UOC
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getEarnings --arguments 0x554f432d643133396262

######## getEarnings -EGLD
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getEarnings --arguments 0x45474c44

######## egldToToken
erdpy --verbose contract call $CONTRACT_ADDRESS --pem="~/wallet/wallet2.pem" --gas-limit=200000000 --function="egldToToken" --proxy="https://devnet-gateway.elrond.com" --chain=D  --recall-nonce --value 10000000000000000 --arguments 0x554f432d643133396262 --send

######## getEarnings
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getEarnings --arguments 0x554f432d643133396262

######## claimEarnings
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=claimEarnings --arguments 0x554f432d643133396262

######## getTokens
erdpy --verbose contract query $CONTRACT_ADDRESS --proxy https://devnet-gateway.elrond.com --function=getTokens

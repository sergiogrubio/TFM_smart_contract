OWNER_PEM="./wallet/wallet3.pem"
CONTRACT_WASM="./output/testdex.wasm"
PROXY="https://devnet-api.elrond.com"
CHAIN_ID="D"
ARG_FEE="0x05"
TOKEN="0x554f432d643133396262" # UOC-d139bb
TOKEN_EGLD="0x45474c44" # EGLD=0x45474c44
GAS_LIMIT="200000000"
AMOUNT="0x05"
AMOUNT2="1000"
AMOUNT3="100000000000000000"
TOKEN_AMOUNT="0x0f4240" # 1000000
METHOD_NAME_SWAP="0x7377617045676c64466f72546f6b656e" # swapEgldForToken
METHOD_NAME_FUND="0x6164644c6971756964697479546f6b656e" # addLiquidityToken

build() {
    echo "***Snippet: Build contract"

    (set -x; erdpy --verbose contract build "$CONTRACT")
}

deploy() {
    echo "***Snippet: Deploy contract"
    
    erdpy --verbose contract deploy --recall-nonce \
        --bytecode=${CONTRACT_WASM} \
        --pem=${OWNER_PEM} \
        --gas-limit=100000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments ${ARG_FEE} \
        --outfile="deploy-devnet.interaction.json" \
        --send || return

    DEPLOY_TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${DEPLOY_TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
    echo "Tx: ${DEPLOY_TRANSACTION}"

}

checkDeployment() {
    echo "***Snippet: Check deployment"

    erdpy tx get --hash=${DEPLOY_TRANSACTION} --omit-fields="['data', 'signature']" --proxy=${PROXY}
    erdpy account get --address=${ADDRESS} --omit-fields="['code']" --proxy=${PROXY}
}

addLiquidityToken() {
    echo "***Snippet: Funding a pool (token)"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --recall-nonce \
        --arguments ${TOKEN} ${TOKEN_AMOUNT} ${METHOD_NAME_FUND} \
        --send
}

addLiquidityEgld() {
    echo "******Snippet: Funding a pool (EGLD)"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function="addLiquidityEgld" \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --value ${AMOUNT2} \
        --arguments ${TOKEN} \
        --send
}

getLiquidityToken() {
    echo "***Snippet: Token liquidity of a pair EGLD-TOKEN"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=getLiquidityToken \
        --arguments ${TOKEN}
}

getLiquidityEgld() {
    echo "***Snippet: EGLD liquidity of a pair EGLD-TOKEN"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=getLiquidityEgld \
        --arguments ${TOKEN}
}

getLiquidityStatus() {
    echo "***Snippet: Status of a liquidity pool (Funding or Successful)"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=status \
        --arguments ${TOKEN}
}

calculateK() {
    echo "***Snippet: Calculate K constant of a pair"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=calculateK \
        --arguments ${TOKEN}
}

getFee() {
    echo "***Snippet: Fee charged on swaps"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=getFee
}

getPriceEgldToken() {
    echo "***Snippet: Price in TOKEN of EGLD"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceEgldToken \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceEgldTokenNumerator() {
    echo "***Snippet: Price in TOKEN of EGLD (numerator of the division)"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceEgldTokenNumerator \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceEgldTokenDenominator() {
    echo "***Snippet: Price in TOKEN of EGLD (denominator of the division)"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceEgldTokenDenominator \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceEgldTokenNoFee() {
    echo "***Snippet: Price in TOKEN of EGLD without fee"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceEgldTokenNoFee \
        --arguments ${TOKEN} ${AMOUNT}
}

getFeeEgldToken() {
    echo "***Snippet: Fee on swaps of TOKEN for EGLD"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=feeEgldToken \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceTokenEgld() {
    echo "***Snippet: Price in EGLD of TOKEN"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceTokenEgld \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceTokenEgldNumerator() {
    echo "***Snippet: Price in EGLD of TOKEN (numerator of the division)"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceTokenEgldNumerator \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceTokenEgldDenominator() {
    echo "***Snippet: Price in EGLD of TOKEN (denominator of the division)"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceTokenEgldDenominator \
        --arguments ${TOKEN} ${AMOUNT}
}

getPriceTokenEgldNoFee() {
    echo "***Snippet: Price in TOKEN of EGLD without fee"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=priceTokenEgldNoFee \
        --arguments ${TOKEN} ${AMOUNT}
}

getFeeTokenEgld() {
    echo "***Snippet: Fee on swaps of TOKEN for EGLD"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=feeTokenEgld \
        --arguments ${TOKEN} ${AMOUNT}
}

getRatio() {
    echo "***Snippet: Ratio of a pair (90-30 ratio is 3)"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=ratio \
        --arguments ${TOKEN}
}

getEarningsEgld() {
    echo "***Snippet: Get earnings in TOKEN"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=getEarnings \
        --arguments ${TOKEN}
}

getEarningsToken() {
    echo "***Snippet: Get earnings in EGLD"

    erdpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function=getEarnings \
        --arguments ${TOKEN_EGLD}
}

claimEarningsToken() {
    echo "***Snippet: Claim earnings of a token"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function="claimEarnings" \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --value 0 \
        --arguments ${TOKEN} \
        --send
}

claimEarningsEgld() {
    echo "***Snippet: Claim earnings of a token"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function="claimEarnings" \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --value 0 \
        --arguments ${TOKEN_EGLD} \
        --send
}

claimLiquidityEgld() {
    echo "***Snippet: Claim EGLD of a pair EGLD-TOKEN"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function="claimLiquidityEgld" \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --value 0 \
        --arguments ${TOKEN} \
        --send
}

claimLiquidityToken() {
    echo "***Snippet: Claim TOKEN of a pair EGLD-TOKEN"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function="claimLiquidityToken" \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --value 0 \
        --arguments ${TOKEN} \
        --send
}

swapTokenForEgld() {
    echo "***Snippet: Buy EGLD using TOKEN"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --function="swapTokenForEgld" \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --value ${AMOUNT2} \
        --arguments ${TOKEN} \
        --send
}

swapEgldForToken() {
    echo "***Snippet: Buy TOKEN using EGLD"

    erdpy --verbose contract call ${ADDRESS} \
        --pem=${OWNER_PEM} \
        --gas-limit=${GAS_LIMIT} \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --function="ESDTTransfer" \
        --recall-nonce \
        --arguments ${TOKEN} ${TOKEN_AMOUNT} ${METHOD_NAME_SWAP} \
        --send
}



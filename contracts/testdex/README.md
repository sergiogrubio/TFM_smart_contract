# Interaction

## On devnet

To compile the smart contract:

```
cd contracts/testdex/
erdpy contract build

```

To deploy it (requires and argument with the fee used in the DEX):

```
erdpy contract deploy --pem="~/wallet/wallet1.pem" --recall-nonce --gas-limit=100000000 --project=. --proxy="https://devnet-gateway.elrond.com" --chain="D" --arguments 0x05 --send

```

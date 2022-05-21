# testDEX smart contract

testDEX is a simple DEX using the Elrond network. University master's degree final project (cybersecurity/UOC).

In this repo you'll find a smart contract named testdex.rs (a free AMM protocol implementation).

To build the smart contract go to ./contracts/testdex and run:

```
erdpy contract build
```

To deploy it:

```
erdpy contract deploy --pem="~/wallet/wallet1.pem" \
  --recall-nonce --gas-limit=100000000 --project=. \
  --proxy="https://devnet-gateway.elrond.com" \
  --chain="D" --arguments 0x05 --send
```

Note that ~/wallet/wallet1.pem is the private key of a user in the devnet network and the argument is required by the smart contract constructor (0.05 fee).

Yoy may also want to see the followind repos:

- The thesis of the project (the latex template was made by [UOC](https://www.uoc.edu/)) (it is written in catalan).
- The [dApp](https://github.com/sergiogrubio/TFM_dapp) using this smart contract
- A [script to test Elrond's network performance](https://github.com/sergiogrubio/TFM_stats).

#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy, Debug)]
pub enum Status {
    Funding,
    Successful
}

// source: https://users.rust-lang.org/t/ternary-operator/40330
macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}

/// testDEX is a DEX implementing AMM
#[elrond_wasm::contract]
pub trait TestDEX {

    // to store liquidity of the tokens
    #[view(getLiquidityToken)]
    #[storage_mapper("liquidity_token")]
    fn liquidity_token(&self, token: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    // to store liquidity of the tokens in EGLD
    #[view(getLiquidityEgld)]
    #[storage_mapper("liquidity_egld")]
    fn liquidity_egld(&self, token: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    // tokens with pairs ready to swap
    // I chose implement it this way for gas efficiency
    // https://docs.elrond.com/developers/best-practices/storage-mappers/#singlevaluemapper-vs-mapmapper
    // finally, I chose the option is to get the tokens of the smart contract with the Elrond's API Rest,
    // checking before trading that K > 0 (pair status Sucessful)
    // #[view(getTokens)]
    // #[storage_mapper("tokens")]
    // fn tokens(&self) -> SingleValueMapper<ManagedVec<TokenIdentifier>>;

    // K constant for a pair
    #[view(getInitialK)]
    #[storage_mapper("initial_k")]
    fn initial_k(&self, token: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    // fee applied to swaps
    #[view(getFee)]
    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<u32>;

    // fees of the DEX are stored here, the owner of the concract may claim these funds
    #[view(getEarnings)]
    #[storage_mapper("earnings")]
    fn earnings(&self, token: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    // constructor
    #[init]
    fn init(&self, fee: u32) {
        // i.e., value 5 is 0.5 fee
        self.fee().set(&fee);
    }

    // #[endpoint(addLiquidity)]
    // #[only_owner]
    // #[payable("*")]
    // fn add_liquidity(&self, token2: &TokenIdentifier, qty: &BigUint) -> SCResult<()> {
        
    //     let (payment, token) = self.call_value().payment_token_pair();
    //     let caller = self.blockchain().get_caller();
    //     let sc_address 
        
    //     self.liquidity_token(&token).update(|liquidity_token| *liquidity_token += payment);

    //     self.send().direct(&caller, &token2, 0, &qty, &[]);

    //     self.liquidity_egld(&token2).update(|liquidity_egld| *liquidity_egld += payment);

    //     Ok(())

    // }

    // add liquidity of a token to a pair
    #[endpoint(addLiquidityToken)]
    #[only_owner]
    #[payable("*")]
    fn add_liquidity_token(&self) -> SCResult<()> {
        
        let (payment, token) = self.call_value().payment_token_pair();
        let state = self.status(&token);
        require!(
            state == Status::Funding,
            "Pair already funded."
        );

        self.liquidity_token(&token).update(|liquidity_token| *liquidity_token += payment);

        if self.status(&token) == Status::Successful {
            let initial_k = self.calculate_k(&token);
            self.initial_k(&token).set(&initial_k);
            // add token to the tokens vector
            // let mut vec_tokens = self.tokens().get();
            // vec_tokens.push(token);
            // self.tokens().set(vec_tokens);
        }

        Ok(())

    }
    
    // claim liquidity of a token in a pair
    #[endpoint(claimLiquidityToken)]
    #[only_owner]
    #[payable("*")]
    fn claim_liquidity_token(&self, token: &TokenIdentifier) -> SCResult<()>{

        let funds = self.liquidity_token(&token).get();
        require!(funds > 0u32, "cannot claim 0 funds");
        let owner = self.blockchain().get_owner_address();

        if self.status(&token) == Status::Successful {
            // remove token from the tokens vector
            // let mut vec_tokens = self.tokens().get();
            // let index = vec_tokens.iter().position(|x| *x == token.clone()).unwrap();
            // vec_tokens.remove(index);
            // self.tokens().set(vec_tokens);
            // set initial K to 0
            self.initial_k(&token).set(BigUint::zero());
        }
        self.liquidity_token(&token).clear();
        self.send().direct(&owner, &token, 0, &funds, &[]);

        Ok(())

    }

    // add liquidity of EGLD to a pair
    #[endpoint(addLiquidityEgld)]
    #[only_owner]
    #[payable("*")]
    fn add_liquidity_egld(&self, token: &TokenIdentifier) -> SCResult<()> {
        
        let funded = self.status(&token);
        require!(
            funded == Status::Funding,
            "Pair already funded."
        );
        let payment = self.call_value().egld_value();
        
        self.liquidity_egld(token).update(|liquidity_egld| *liquidity_egld += payment);

        if self.status(&token) == Status::Successful {
            let initial_k = self.calculate_k(&token);
            self.initial_k(&token).set(&initial_k);
            // add element to the tokens vector
            // let mut vec_tokens = self.tokens().get();
            // vec_tokens.push(token.clone());
            // self.tokens().set(vec_tokens);
        }

        Ok(())

    }

    // add liquidity (EGLD-token)
    // #[endpoint(addLiquidity)]
    // #[only_owner]
    // #[payable("*")]
    // fn add_liquidity(&self) -> SCResult<()> {

    //     let payment_egld = self.call_value().egld_value();
    //     let (payment_token, token) = self.call_value().payment_token_pair();
        
    //     self.liquidity_egld(&token).update(|liquidity_egld| *liquidity_egld += payment_egld);
    //     self.liquidity_token(&token).update(|liquidity_token| *liquidity_token += payment_token);


    //     if self.status(&token) == Status::Successful {
    //         let initial_k = self.calculate_k(&token);
    //         self.initial_k(&token).set(&initial_k);
    //         // add element to the tokens vector
    //         let mut vec_tokens = self.tokens().get();
    //         vec_tokens.push(token.clone());
    //         self.tokens().set(vec_tokens);
    //     }

    //     Ok(())

    // }

    // claim liquidity of EGLD in a pair
    #[endpoint(claimLiquidityEgld)]
    #[only_owner]
    #[payable("*")]
    fn claim_liquidity_egld(&self, token: &TokenIdentifier) -> SCResult<()> {

        let funds = self.liquidity_egld(&token).get();
        require!(funds > 0u32, "cannot claim 0 funds");
        let owner = self.blockchain().get_owner_address();

        if self.status(&token) == Status::Successful {
            // remove token from the tokens vector
            // let mut vec_tokens = self.tokens().get();
            // let index = vec_tokens.iter().position(|x| *x == token.clone()).unwrap();
            // vec_tokens.remove(index);
            // self.tokens().set(vec_tokens);
            // set initial K to 0
            self.initial_k(&token).set(BigUint::zero());
        }
        self.liquidity_egld(&token).clear();
        self.send().direct(&owner, &TokenIdentifier::egld(), 0, &funds, &[]);

        Ok(())

    }

    // status of a pair for swapping
    #[view]
    fn status(&self, token: &TokenIdentifier) -> Status {

        if self.liquidity_egld(&token).get() > 0 && self.liquidity_token(&token).get() > 0  {
            Status::Successful
        } else {
            Status::Funding
        }

    }

    // #[view(getNumTokens)]
    // fn num_tokens(&self) -> usize {

    //     self.tokens().len()

    // }
    
    // calculate K constant
    #[view(calculateK)]
    fn calculate_k(&self, token: &TokenIdentifier) -> BigUint {

        self.liquidity_egld(&token).get() * self.liquidity_token(&token).get()

    }

    // claim earning of a token
    #[endpoint(claimEarnings)]
    #[only_owner]
    #[payable("*")]
    fn claim_earnings(&self, token: &TokenIdentifier) -> SCResult<()> {
        
        let funds = self.earnings(&token).get();
        require!(funds > 0u32, "cannot claim 0 funds");
        let owner = self.blockchain().get_owner_address();

        self.earnings(&token).clear();
        self.send().direct(&owner, &token, 0, &funds, &[]);

        Ok(())

    }

    // calculte price of qty token in EGLD with fee
    // in: quantity EGLD
    // out: quantity token (with fee subtracted)
    // get how many EGLD I need to get qty token
    #[view(priceEgldToken)]
    fn price_egld_token(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_egld = self.liquidity_egld(&token).get();
        let qty_token = self.liquidity_token(&token).get();
        let fee = self.fee().get();
        let numerator: BigUint = qty_token * qty * (1000u32 - fee);
        let denominator: BigUint = qty_egld * 1000u32 + qty * (1000u32 - fee);

        numerator / denominator
    }

    // calculte price of qty token in EGLD with fee, numerator only
    #[view(priceEgldTokenNumerator)]
    fn price_egld_token_numerator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_token = self.liquidity_token(&token).get();
        let fee = self.fee().get();
        let numerator: BigUint = qty_token * qty * (1000u32 - fee);

        numerator 
    }

    // calculte price of qty token in EGLD with fee, denominator only
    #[view(priceEgldTokenDenominator)]
    fn price_egld_token_denominator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_egld = self.liquidity_egld(&token).get();
        let fee = self.fee().get();
        let denominator: BigUint = qty_egld * 1000u32 + qty * (1000u32 - fee);

        denominator
    }

    // calculte price of qty token in EGLD without fee
    // in: quantity EGLD
    // out: quantity token (without fee)
    #[view(priceEgldTokenNoFee)]
    fn price_egld_token_no_fee(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_egld = self.liquidity_egld(&token).get();
        let qty_token = self.liquidity_token(&token).get();
        let numerator: BigUint =  qty_token * qty;
        let denominator: BigUint = qty_egld + qty;

        numerator / denominator
    }

    #[view(priceEgldTokenNoFeeNumerator)]
    fn price_egld_token_no_fee_numerator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_token = self.liquidity_token(&token).get();
        let numerator: BigUint =  qty_token * qty;

        numerator
    }

    #[view(priceEgldTokenNoFeeDenominator)]
    fn price_egld_token_no_fee_denominator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_egld = self.liquidity_egld(&token).get();
        let denominator: BigUint = qty_egld + qty;

        denominator
    }

    // calcute fee to pay in qty token
    // in: token
    // out: quantity EGLD paid as a fee
    #[view(feeEgldToken)]
    fn fee_egld_token(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {

        let value_fee = self.price_egld_token(&token, &qty);
        let value_no_fee = self.price_egld_token_no_fee(&token, &qty);

        value_no_fee - value_fee

    }

    // calculate price of qty EGLD in token with fee
    // get how many tokens I need to get qty EGLD
    #[view(priceTokenEgld)]
    fn price_token_egld(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_egld = self.liquidity_egld(&token).get();
        let qty_token = self.liquidity_token(&token).get();
        let fee = self.fee().get();
        let numerator: BigUint = qty_egld * qty * (1000u32 - fee);
        let denominator: BigUint = qty_token * 1000u32 + qty * (1000u32 - fee);

        numerator / denominator
    }

    // calculate price of qty EGLD in token with fee, numerator only
    #[view(priceTokenEgldNumerator)]
    fn price_token_egld_numerator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_egld = self.liquidity_egld(&token).get();
        let fee = self.fee().get();
        let numerator: BigUint = qty_egld * qty * (1000u32 - fee);

        numerator
    }

    // calculate price of qty EGLD in token with fee, denominator only
    #[view(priceTokenEgldDenominator)]
    fn price_token_egld_denominator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {
        
        let qty_token = self.liquidity_token(&token).get();
        let fee = self.fee().get();
        let denominator: BigUint = qty_token * 1000u32 + qty * (1000u32 - fee);

        denominator
    }
              
    // in: quantity token
    // out: quantity EGLD (without fee)
    #[view(priceTokenEgldNoFee)]
    fn price_token_egld_no_fee(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {

        let qty_egld = self.liquidity_egld(&token).get();
        let qty_token = self.liquidity_token(&token).get();
        let numerator: BigUint = qty_egld * qty;
        let denominator: BigUint = qty_token + qty;

        numerator / denominator

    }

    #[view(priceTokenEgldNoFeeNumerator)]
    fn price_token_egld_no_fee_numerator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {

        let qty_egld = self.liquidity_egld(&token).get();
        let numerator: BigUint = qty_egld * qty;

        numerator 
    }

    #[view(priceTokenEgldNoFeeDenominator)]
    fn price_token_egld_no_fee_denominator(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {

        let qty_token = self.liquidity_token(&token).get();

        let denominator: BigUint = qty_token + qty;

        denominator
    }

    #[view(feeTokenEgld)]
    fn fee_token_egld(&self, token: &TokenIdentifier, qty: &BigUint) -> BigUint {

        let value_fee = self.price_token_egld(&token, &qty);
        let value_no_fee = self.price_token_egld_no_fee(&token, &qty);

        value_no_fee - value_fee

    }

    #[view(ratio)]
    fn ratio(&self, token: &TokenIdentifier) -> BigUint {

        let liq_egld = self.liquidity_egld(&token).get();
        let liq_token = self.liquidity_token(&token).get();

        let ratio: BigUint = either!(liq_token > liq_egld => liq_token/liq_egld; liq_egld/liq_token);

        if ratio > 1 {
            ratio
        } else {
            BigUint::from(1u32)
        }
            
    }

    // Swap egld for token
    // in: quantity egld, token that identifies the pair
    // out: quantity token (with fee subtracted) sent to the customer's wallet
    #[endpoint(swapEgldForToken)]
    #[payable("*")]
    fn swap_egld_for_token(&self, token: &TokenIdentifier) ->  SCResult<()> {
        
        let state = self.status(&token);

        require!(
            state == Status::Successful,
            "Pair still funding!"
        );


        // egld paid for token with fees
        let payment = self.call_value().egld_value(); // EGLD
        // token bought with egld with fees
        let token_fee =  self.price_egld_token(&token, &payment);
        // token bought with egld without fees
        let token_no_fee =  self.price_egld_token_no_fee(&token, &payment);
        // fees paid in token
        let earning_token = &token_no_fee - &token_fee;
        // customer's address
        let caller = self.blockchain().get_caller();
        let initial_k = self.initial_k(&token).get();


        self.liquidity_egld(&token).update(|liquidity_egld| *liquidity_egld += &payment);
        self.liquidity_token(&token).update(|liquidity_token| *liquidity_token -= &token_no_fee);
        self.earnings(&token).update(|earnings| *earnings += &earning_token);
        
        let new_k = self.calculate_k(&token);

        // Adjusting K constant
        // I correct it adjusting the token side of the pair
        // I get the correction from the earnings, another option
        // is to get it from the tokens sent to the customer's wallet
        if new_k > initial_k {
            let new_liq_egld = self.liquidity_egld(&token).get();
            let new_liq_token = self.liquidity_token(&token).get();
            let earnings_token = self.earnings(&token).get();
            let liq_token_corrected = &new_k / &new_liq_egld;
            let amount_correction = &liq_token_corrected - &new_liq_token;
            let final_correction;
            if amount_correction <= earnings_token {
                final_correction = amount_correction;
            } else {
                final_correction = earnings_token;
            }
            self.liquidity_token(&token).update(|liquidity_token| *liquidity_token -= final_correction.clone());
            self.earnings(&token).update(|earnings| *earnings += final_correction.clone());
        } else if new_k < initial_k {
            let new_liq_egld = self.liquidity_egld(&token).get();
            let new_liq_token = self.liquidity_token(&token).get();
            let earnings_token = self.earnings(&token).get();
            let liq_token_corrected = &new_k / &new_liq_egld;
            let amount_correction = &new_liq_token - &liq_token_corrected;
            let final_correction;
            if amount_correction <= earnings_token {
                final_correction = amount_correction;
            } else {
                final_correction = earnings_token;
            }
            self.liquidity_token(&token).update(|liquidity_token| *liquidity_token += final_correction.clone());
            self.earnings(&token).update(|earnings| *earnings -= final_correction.clone());
        }

        // send token bought (token_fee) to customer address
        self.send().direct(&caller, &token, 0, &token_fee, &[]);

        Ok(())
    }

    // Swap token for egld
    // in: quantity token, token that identifies the pair
    // out: quantity egld (with fee subtracted) sent to the customer's wallet
    #[endpoint(swapTokenForEgld)]
    #[payable("*")]
    fn swap_token_for_egld(&self) -> SCResult<()> {

        let (payment, token) = self.call_value().payment_token_pair();

        let state = self.status(&token);

        require!(
            state == Status::Successful,
            "Pair still funding!"
        );


        let egld_fee =  self.price_token_egld(&token, &payment);
        let egld_no_fee =  self.price_token_egld_no_fee(&token, &payment);
        let earning_egld = &egld_no_fee - &egld_fee;
        let caller = self.blockchain().get_caller();
        let initial_k = self.initial_k(&token).get();

        self.liquidity_token(&token).update(|liquidity_token| *liquidity_token += &payment);
        self.liquidity_egld(&token).update(|liquidity_egld| *liquidity_egld -= &egld_no_fee);
        self.earnings(&TokenIdentifier::egld()).update(|earnings| *earnings += &earning_egld);
        
        let new_k = self.calculate_k(&token);


        // Adjusting K constant
        // I correct it adjusting the egld side of the pair
        // I get the correction from the earnings, another option
        // is to get it from the egld sent to the customer's wallet
        if new_k > initial_k {
            let new_liq_egld = self.liquidity_egld(&token).get();
            let new_liq_token = self.liquidity_token(&token).get();
            let earnings_egld =  self.earnings(&TokenIdentifier::egld()).get();
            let liq_egld_corrected = &new_k / &new_liq_token;
            let amount_correction = &liq_egld_corrected - &new_liq_egld;
            let final_correction;
            if amount_correction <= earnings_egld {
                final_correction = amount_correction;
            } else {
                final_correction = earnings_egld;
            }
            self.liquidity_egld(&token).update(|liquidity_egld| *liquidity_egld -= final_correction.clone());
            self.earnings(&TokenIdentifier::egld()).update(|earnings| *earnings += final_correction.clone());
        } else if new_k < initial_k {
            let new_liq_egld = self.liquidity_egld(&token).get();
            let new_liq_token = self.liquidity_token(&token).get();
            let earnings_egld =  self.earnings(&TokenIdentifier::egld()).get();
            let liq_egld_corrected = &new_k / &new_liq_token;
            let amount_correction = &new_liq_egld - &liq_egld_corrected;
            let final_correction;
            if amount_correction <= new_liq_egld {
                final_correction = amount_correction;
            } else {
                final_correction = earnings_egld;
            }
            self.liquidity_egld(&token).update(|liquidity_egld| *liquidity_egld += final_correction.clone());
            self.earnings(&TokenIdentifier::egld()).update(|earnings| *earnings -= final_correction.clone());     
        }

        // send token bought (token_fee) to customer address
        self.send().direct(&caller, &TokenIdentifier::egld(), 0, &egld_fee, &[]);

        Ok(())

    }
}
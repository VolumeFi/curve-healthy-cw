# Curve Healthy Bot CosmWasm smart contract

This is a CosmWasm smart contract to manage Curve Leverage Lending Bot smart contract on EVM chain written in Vyper.

Users can create a crvUSD loan by deposit their token into a Vyper smart contract with leverage option on EVM chain.

A scheduler or script fetch events from the Vyper smart contract and run `repay_bot` function with the parameters via Compass-EVM on high risk or expiration.

And then, the Vyper smart contract will repay the bot.

## ExecuteMsg

### RepayBot

Run `repay_bot` function on Vyper smart contract.

| Key                        | Type           | Description                     |
|----------------------------|----------------|---------------------------------|
| bot_info                   | Vec\<BotInfo\> | Array of data to add collateral |

### SetPaloma

Run `set_paloma` function on Vyper smart contract to register this contract address data in the Vyper contract.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

    UpdateCompass { new_compass: String },

### Update*

Run `update_*` function on Vyper smart contract to register this contract address data in the Vyper contract.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

## QueryMsg

### GetJobId

Get `job_id` of Paloma message to run `multiple_withdraw` function on a Vyper smart contract.

| Key | Type | Description |
|-----|------|-------------|
| -   | -    | -           |

#### Response

| Key    | Type   | Description      |
|--------|--------|------------------|
| job_id | String | Job Id on Paloma |

## Structs

### BotInfo

| Key           | Type           | Description                           |
|---------------|----------------|---------------------------------------|
| bot           | String         | Bot address                           |
| callbacker    | String         | Callbacker contract address           |
| callback_args | Vec\<Uint256\> | Callback args for callbacker contract |
| swap_infos    | SwapInfo       | Curve Swap info                       |


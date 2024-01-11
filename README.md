# rfq-marketplace

rfq-marketplace is a contract that provides a token exchange platform.

## Prerequisites

- Node.js
- NPM

## Deploy the contract

The first step, clone the project and install the dependency.
```
git clone https://github.com/second-state/rfq-marketplace.git
cd rfq-marketplace
npm install
```
You can edit the `hardhat.config.js` to choose which network and account to deploy the contract.
In the example, we use CyberMiles chain to demo.

Next step, run the following command to run the deploy script.
```
npx hardhat run script/deploy.js --network cyber
```

Then you can see the contract address.
```
Deployed to 0xAe7FEF0Afdaf803E2665Ec6D83D8126D97dFcbFE
```

## Deploy web service on flows.network

To interact with rfq-marketplace contract, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Prerequisite

You will need a wallet private key. If you do not already have one, use [Metamask](https://metamask.io/) to create it.

#### Fork this repo and write your own code

Fork [this repo](https://github.com/second-state/rfq-marketplace.git). 

#### Deploy the code on flows.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the web service.
3. Authenticate the [flows.network](https://flows.network/) to access the `rfq-marketplace` repo you just forked. 

4. Click on the Advanced text and you will see more settings including branch and environment variables. The function code is stored in `flows` folder, you need to change Directory to `/flows`.
 In this example, we have four variables to fill in, `PRIVATE_KEY` is the wallet private key, and `CONTRACT_ADDRESS` is rfq-market contract [address]( #deploy-the-contract).
The default network is Arbitrum sepolia. If you want to change the network, you can set `RPC_NODE_URL` and `CHAIN_ID` variable.

<img width="899" alt="image" src="https://i.imgur.com/xSAxwLF.png">

5. Click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow. Here we can see: there is no SaaS needs to be connected since it's a lambda service. Just click the Check button to see your flow details.

<img width="964" alt="image" src="https://user-images.githubusercontent.com/45785633/226959151-0e8a159a-02b3-4130-b7b5-8831b65c8d75.png">

## Try this demo
### Create an exchange request

You can use the `create-exchange` function to create a new exchange request. You need to provide three query parameters.<br>
`tokenA`: The token address you want to exchange out. <br>
`tokenB`: The token address you want to exchange in. <br>
`amount`: The amount of tokenA you want to exchange.<br>
Copy and paste the endpoint URL to your browser and add `/create-exchange?tokenA=0x30D30c71d8618Ce42783eDd2C7Ae6f15eeD69Fec&tokenB=0x948Fa9010EFBEed5f4943893a383B7e2210bA145&amount=100`. Then you can see the transaction result.<br>
(You need to approve enough tokens to rfq contract before creating an exchange request)

<img width="964" alt="image" src="https://i.imgur.com/47UskMW.png">

### Get an exchange request

After creating an exchange request, use the `get-exchange` function to query all the exchange requests.<br>
Copy and paste the endpoint URL to your browser and add `/get-exchange`.
Then you can see all the exchange requests in the rfq-marketplace.

<img width="964" alt="image" src="https://i.imgur.com/4wJWy9n.png">

### Response exchange

If you want to exchange tokens with others, use `response-exchange` to respond to other's exchange requests.<br>
Copy and paste the endpoint URL to your browser and add `/get-exchange?request-id=0`.
Then you can see the transaction result.<br>
(You need to approve enough tokens to rfq contract before the response exchange)
> You need to set the buyer private key in flows.network

<img width="964" alt="image" src="https://i.imgur.com/uFJVaxv.png">

### Get response

You can use `get-response` to query all the buyers of the exchange request. <br>
Copy and paste the endpoint URL to your browser and add `/get-exchange?request-id=0`.
Then you can see the buyer information.

The amount the buyer wants to exchange is the amount of tokenB to you.

<img width="964" alt="image" src="https://i.imgur.com/ru0B9kB.png">

### Accept exchange

If you are the owner of the exchange request, you have the right to decide which buyer you want to exchange. Using `accept-exchange` to accept the response.
Copy and paste the endpoint URL to your browser and add `/get-exchange?request-id=0&buy-id=0`.

<img width="964" alt="image" src="https://i.imgur.com/7hVz4Kx.png">

> You need to set the owner private key in flows.network

### Withdraw

When the request is not finished until exceeds `requestLiveTime` or the owner accepts the exchange, all the buyer and owner can use `withdraw` to withdraw the token that is locked in rfq-marketplace. <br>

> The two following demo images are different private keys that are set in the flows.network to call the function

<img width="964" alt="image" src="https://i.imgur.com/yPCj10R.png">
<img width="964" alt="image" src="https://i.imgur.com/h4EbPUm.png">

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!

## Others


To build locally, make sure you have intsalled Rust and added `wasm32-wasi` target.

```
cd flows
cargo build target wasm32-wasi --release
```

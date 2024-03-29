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
npx hardhat run scripts/deploy.js --network cybermiles
```

Then you can see the contract address.
```
Deployed to 0xD80B7630C7D7674b0BBF3bC34328d4CAdcE91FA2
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
In this example, we have three variables to fill in, `CONTRACT_ADDRESS` is rfq-marketplace contract [address]( #deploy-the-contract).
The default network is cybermiles. If you want to change the network, you can set `RPC_NODE_URL` and `CHAIN_ID` variable.

<img width="899" alt="image" src="https://i.imgur.com/xSAxwLF.png">

5. Click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow. Here we can see: there is no SaaS needs to be connected since it's a lambda service. Just click the Check button to see your flow details.

<img width="964" alt="image" src="https://user-images.githubusercontent.com/45785633/226959151-0e8a159a-02b3-4130-b7b5-8831b65c8d75.png">

## Try this demo
### Create an exchange request

You can use the `submit-request` function to create a new exchange request. You need to provide three query parameters.<br>
`token-out`: The token address you want to exchange out. <br>
`token-in`: The token address you want to exchange in. <br>
`amount`: The amount of token-out you want to exchange.<br>
`lifetime`(option) : This request lifetime unit in seconds, default is `requestLifetime`. <br>
In the demo, we use curl to send the post request, you can copy and paste the endpoint URL to your shell and add `/submit-request?token-out=0x30D30c71d8618Ce42783eDd2C7Ae6f15eeD69Fec&token-in=0x948Fa9010EFBEed5f4943893a383B7e2210bA145&amount=100`. <br>
(You need to approve enough tokens to rfq contract before creating an exchange request)

``` shell
curl -X POST "https://code.flows.network/webhook/ekbbxC47MjjtIaP8RmO8/submit-request?token-out=0x30D30c71d8618Ce42783eDd2C7Ae6f15eeD69Fec&token-in=0x948Fa9010EFBEed5f4943893a383B7e2210bA145&amount=100&lifetime=3600" \
-d '{"PRIVATE_KEY": "Exchange request owner private key"}'
```
Then you can see the transaction result.<br>

### List requests

After creating an exchange request, use the `list-requests` function to query all the exchange requests.<br>
Copy and paste the endpoint URL to your browser and add `/list-requests`.
Then you can see all the exchange requests in the rfq-marketplace.

<img width="964" alt="image" src="https://i.imgur.com/nK8ks1Z.png">

### Response exchange

If you want to exchange tokens with others, use `submit-response` to respond to other's exchange requests.<br>
You can copy and paste the endpoint URL to your shell and add `/submit-response?request-id=0&amount=10&lifetime=3600`. <br>
`request-id` : The request you want to exchange. <br>
`amount` : The amount you want to exchange out ([token-in](#create-an-exchange-request)).<br>
`lifetime`(option) : This response lifetime unit in seconds, default is the same as request lifetime. You can withdraw after the lifetime.<br>

(You need to approve enough tokens to rfq contract before the response exchange)

``` shell
curl -X POST "https://code.flows.network/webhook/ekbbxC47MjjtIaP8RmO8/submit-response?request-id=0&amount=10&lifetime=3600" \
-d '{"PRIVATE_KEY": "Buyer private key"}'
```
Then you can see the transaction result.<br>

### Get request

You can use `get-request` to query all the buyers of the exchange request. <br>
Copy and paste the endpoint URL to your browser and add `/get-request?request-id=0`.
Then you can see the buyer information.

The amount the buyer wants to exchange is the amount of token-in to you.

<img width="964" alt="image" src="https://i.imgur.com/fBwhn0V.png">

### Accept exchange

If you are the owner of the exchange request, you have the right to decide which buyer you want to exchange.
> You need accept not expired response.

Using `accept-exchange` to accept the response.
You can copy and paste the endpoint URL to your shell and add `/accept-exchange?request-id=0&response-id=0`.

``` shell
curl -X POST "https://code.flows.network/webhook/ekbbxC47MjjtIaP8RmO8/accept-exchange?request-id=0&response-id=0" \
-d '{"PRIVATE_KEY": "Exchange request owner private key"}'
```
Then you can see the transaction result.<br>

### Withdraw

When the request is not finished until exceeds `expire_time` or the owner accepts the exchange, all the buyer and owner can use `withdraw` to withdraw the token that is locked in rfq-marketplace. You can copy and paste the endpoint URL to your shell and add `/withdraw?request-id=0` <br>

``` shell
curl -X POST "https://code.flows.network/webhook/ekbbxC47MjjtIaP8RmO8/withdraw?request-id=0" \
-d '{"PRIVATE_KEY": "Buyer private key"}'
```
``` shell
curl -X POST "https://code.flows.network/webhook/ekbbxC47MjjtIaP8RmO8/withdraw?request-id=0" \
-d '{"PRIVATE_KEY": "Exchange request owner private key"}'
```
Then you can see the transaction result. The owner and the buyer will get token-in and token-out respectively.<br>

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!

## Others


To build locally, make sure you have intsalled Rust and added `wasm32-wasi` target.

```
cd flows
cargo build target wasm32-wasi --release
```

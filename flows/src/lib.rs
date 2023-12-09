use webhook_flows::{create_endpoint, request_handler, send_response, route::{get, route, RouteError, Router}};
use flowsnet_platform_sdk::logger;
use serde_json::Value;
use serde_json::json;
use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use ethers_signers::{LocalWallet, Signer};
use ethers_core::types::{NameOrAddress, Bytes, U256, U64, H160, TransactionRequest, transaction::eip2718::TypedTransaction};
use ethers_core::abi::{Abi, Function, Token};
use ethers_core::utils::hex;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(_headers: Vec<(String, String)>, _subpath: String, _qry: HashMap<String, Value>, _body: Vec<u8>) {
    let mut router = Router::new();
    router
        .insert(
            "/create-exchange",
            vec![get(create_exchange)],
        )
        .unwrap();

    router
        .insert(
            "/response-exchange",
            vec![get(response_exchange)],
        )
        .unwrap();

    router
        .insert(
            "/accept-exchange",
            vec![get(accept_exchange)],
        )
        .unwrap();
    
    router
        .insert(
            "/withdraw",
            vec![get(withdraw)],
        )
        .unwrap();

    if let Err(e) = route(router).await {
        match e {
            RouteError::NotFound => {
                send_response(404, vec![], b"No route matched".to_vec());
            }
            RouteError::MethodNotAllowed => {
                send_response(405, vec![], b"Method not allowed".to_vec());
            }
        }
    }
}

fn init_rpc(path: &str, _qry: &HashMap<String, Value>, private_key: &str) -> (String, u64, NameOrAddress, LocalWallet){
    logger::init();
    log::info!("{} Query -- {:?}", path, _qry);
    
    let rpc_node_url = std::env::var("RPC_NODE_URL").unwrap_or("https://sepolia-rollup.arbitrum.io/rpc".to_string());
    let chain_id = std::env::var("CHAIN_ID").unwrap_or("421614".to_string()).parse::<u64>().unwrap_or(421614u64);
    let contract_address = NameOrAddress::from(H160::from_str(std::env::var("CONTRACT_ADDRESS").unwrap().as_str()).unwrap());
    let wallet: LocalWallet = private_key
    .parse::<LocalWallet>()
    .unwrap()
    .with_chain_id(chain_id);
    return (rpc_node_url, chain_id, contract_address, wallet);
}

async fn create_exchange(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let private_key = _qry.get("private-key").unwrap().as_str().unwrap();
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("create_exchange", &_qry, private_key);
    let token_a = H160::from_str(_qry.get("tokenA").unwrap().as_str().unwrap()).unwrap();
    let token_b = H160::from_str(_qry.get("tokenB").unwrap().as_str().unwrap()).unwrap();
    let amount =  U256::from_dec_str(_qry.get("amount").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Address(token_a.into()), Token::Address(token_b.into()), Token::Uint(amount.into())];
    let data = create_contract_call_data("createExchange", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp =json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}

async fn response_exchange(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let private_key = _qry.get("private-key").unwrap().as_str().unwrap();
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("response_exchange", &_qry, private_key);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let amount =  U256::from_dec_str(_qry.get("amount").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(amount.into())];
    let data = create_contract_call_data("bidToken", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp =json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}


async fn accept_exchange(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let private_key = _qry.get("private-key").unwrap().as_str().unwrap();
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("accept_exchange", &_qry, private_key);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let amount =  U256::from_dec_str(_qry.get("amount").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(amount.into())];
    let data = create_contract_call_data("bidToken", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp =json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}

async fn withdraw(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let private_key = _qry.get("private-key").unwrap().as_str().unwrap();
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("withdraw", &_qry, private_key);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let address =  H160::from_str(_qry.get("address").unwrap().as_str().unwrap()).unwrap();
    let is_owner = false;
    let buy_id: U256 = U256::from(0);
    let contract_call_params :Vec<Token>;
    // Not implement that handle if address is owner.

    if is_owner {
        contract_call_params = vec![Token::Uint(request_id.into())];
    }else{
        contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(buy_id.into())];
    }
    let data = create_contract_call_data("withdraw", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp =json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}


pub fn create_contract_call_data(name: &str, tokens: Vec<Token>) -> Result<Bytes> {
    
    let contract_abi = fs::read_to_string("abi.json").expect("Read ABI failed.");
    let abi: Abi = serde_json::from_str(&contract_abi).unwrap();
    let function: &Function = abi
        .functions()
        .find(|&f| f.name == name)
        .ok_or("Function not found in ABI")?;

    let data = function.encode_input(&tokens).unwrap();

    Ok(Bytes::from(data))
}



pub async fn wrap_transaction(rpc_node_url: &str, chain_id: u64, wallet: LocalWallet, address_to: NameOrAddress, data: Bytes, value: U256) -> Result<String> {
    let address_from = wallet.address();
    let nonce = get_nonce(&rpc_node_url, format!("{:?}", wallet.address()).as_str()).await.unwrap();
    let estimate_gas = get_estimate_gas(&rpc_node_url, format!("{:?}", address_from).as_str(), 
                                        format!("{:?}", address_to.as_address().expect("Failed to transfer address")).as_str(), 
                                        format!("0x{:x}", value).as_str(), format!("{:}", data).as_str())
                                        .await
                                        .expect("Failed to gat estimate gas.") * U256::from(12) / U256::from(10);
    
    let tx: TypedTransaction = TransactionRequest::new()
    .from(address_from)
    .to(address_to) 
    .nonce::<U256>(nonce.into())
    .gas_price::<U256>(get_gas_price(&rpc_node_url).await.expect("Failed to get gas price.").into())
    .gas::<U256>(estimate_gas.into())
    .chain_id::<U64>(chain_id.into())
    .data::<Bytes>(data.into())
    .value(value).into();    
    
    log::info!("Tx: {:#?}", tx); 
    
    let signature = wallet.sign_transaction(&tx).await.expect("Failed to sign.");
    

    Ok(format!("0x{}", hex::encode(tx.rlp_signed(&signature))))
}

pub async fn get_gas_price(rpc_node_url: &str) -> Result<U256> {
    let params = json!([]);
    let result = json_rpc(rpc_node_url, "eth_gasPrice", params).await.expect("Failed to send json.");
    
    Ok(U256::from_str(&result)?)
}

pub async fn get_nonce(rpc_node_url: &str, address: &str) -> Result<U256> {
    let params = json!([address, "pending"]);
    let result = json_rpc(rpc_node_url, "eth_getTransactionCount", params).await.expect("Failed to send json.");
    
    Ok(U256::from_str(&result)?)
}

pub async fn get_estimate_gas(rpc_node_url: &str, from: &str, to: &str, value: &str, data: &str) -> Result<U256> {
    let params = json!([{"from": from, "to": to, "value":value, "data":data}]);
    let result = json_rpc(rpc_node_url, "eth_estimateGas", params).await.expect("Failed to send json.");
    
    Ok(U256::from_str(&result)?)
}

pub async fn json_rpc(url: &str, method: &str, params: Value) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Content-Type","application/json")
        .body(json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }).to_string())
        .send()
        .await?;

    let body = res.text().await?;
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(body.as_str())?;
    
    if !map.contains_key("result"){
        log::error!("{} request body: {:#?}", method, json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }));
        log::error!("{} response body: {:#?}", method, map);
    }
    Ok(map["result"].as_str().expect("Failed to parse json.").to_string())
}
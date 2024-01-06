use webhook_flows::{create_endpoint, request_handler, send_response, route::{get, route, RouteError, Router}};
use flowsnet_platform_sdk::logger;
use serde_json::Value;
use serde_json::json;
// use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use ethers_signers::{LocalWallet, Signer};
use ethers_core::types::{NameOrAddress, U256, H160};
use ethers_core::abi::Token;
use ethers_core::rand::thread_rng;

pub mod ether_lib;
use ether_lib::*;

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
    
    router
        .insert(
            "/get-exchange",
            vec![get(get_exchange)],
        )
        .unwrap();

    router
        .insert(
            "/get-response",
            vec![get(get_response)],
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

fn init_rpc(path: &str, _qry: &HashMap<String, Value>) -> (String, u64, NameOrAddress, LocalWallet){
    logger::init();
    log::info!("{} Query -- {:?}", path, _qry);
    
    let rpc_node_url = std::env::var("RPC_NODE_URL").unwrap_or("https://sepolia-rollup.arbitrum.io/rpc".to_string());
    let chain_id = std::env::var("CHAIN_ID").unwrap_or("421614".to_string()).parse::<u64>().unwrap_or(421614u64);
    let contract_address = NameOrAddress::from(H160::from_str(std::env::var("CONTRACT_ADDRESS").unwrap().as_str()).unwrap());
    let mut wallet: LocalWallet = LocalWallet::new(&mut thread_rng());
    
    if let Ok(private_key) = std::env::var("PRIVATE_KEY") {
        let private_key = private_key.as_str();
        wallet = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    }
    return (rpc_node_url, chain_id, contract_address, wallet);
}

async fn create_exchange(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("create_exchange", &_qry);
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
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("response_exchange", &_qry);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let amount =  U256::from_dec_str(_qry.get("amount").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(amount.into())];
    let data = create_contract_call_data("bidToken", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp = json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}


async fn accept_exchange(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("accept_exchange", &_qry);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let buy_id =  U256::from_dec_str(_qry.get("buy-id").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(buy_id.into())];
    let data = create_contract_call_data("acceptBid", contract_call_params).unwrap();

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
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("withdraw", &_qry);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let address =  H160::from_str(_qry.get("address").unwrap().as_str().unwrap()).unwrap();
    let is_owner = false;
    let contract_call_params :Vec<Token>;
    
    if is_owner {
        contract_call_params = vec![Token::Uint(request_id.into())];
    }else{
        
        let buy_id: U256 = U256::from(0);
        contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(buy_id.into())];
    }
    let data = create_contract_call_data("withdraw", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp = json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}

async fn get_exchange(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, _, contract_address, _) = init_rpc("withdraw", &_qry);
    let contract_address = format!("{:?}", contract_address.as_address().unwrap());
    // Keccak-256 exchangeEvent(address,uint256,address,address,uint256)
    let log = get_log(&rpc_node_url, &contract_address, json!(["0xb981be592ff12d76d951facfbbe36a4fd0607fef8ab19502903f32c5fe451460"])).await.unwrap();
    let mut event: Vec<Value> = vec!();
    let len = log.as_array().unwrap().len();
    for idx in 0..len{
        let now = log.get(idx).unwrap();
        let new_vec = json!({
            "owner": format!("0x{}", &(now["topics"][1].to_string()).trim_matches('"')[26..]),
            "requestId": U256::from_str(&(now["topics"][2].to_string()).trim_matches('"')[2..]).unwrap().to_string(),
            "tokenA": format!("0x{}", &now["data"].as_str().unwrap()[26..66]),
            "tokenB": format!("0x{}", &now["data"].as_str().unwrap()[67..130]),
            "amount": U256::from_str(&now["data"].as_str().unwrap()[131..194]).unwrap().to_string(),
        });
        event.push(new_vec);
    } 
    let res_json:Value = json!(Into::<Value>::into(event));
    
    send_response(
        200,
        vec![(String::from("content-type"), String::from("application/json"))],
        serde_json::to_vec_pretty(&res_json).unwrap(),
    );
}

async fn get_response(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, _, contract_address, _) = init_rpc("withdraw", &_qry);
    let contract_address = format!("{:?}", contract_address.as_address().unwrap());
    // Keccak-256 bidEvent(uint256,address,uint256,uint256)
    let log = get_log(&rpc_node_url, &contract_address, json!(["0x5f809e0f670ff1d5d393b4775ee4f31f942aa16ca64ad7b62b25a95920fa37d1"])).await.unwrap();
    let mut event: Vec<Value> = vec!();
    let len = log.as_array().unwrap().len();
    for idx in 0..len{
        let now = log.get(idx).unwrap();
        let new_vec = json!({
            "buyerId": U256::from_str(&(now["topics"][1].to_string()).trim_matches('"')[26..]).unwrap().to_string(),
            "buyer": format!("0x{}", &now["data"].as_str().unwrap()[26..66]),
            "requestId": U256::from_str(&(now["topics"][2].to_string()).trim_matches('"')[2..]).unwrap().to_string(),
            "amount": U256::from_str(&now["data"].as_str().unwrap()[67..130]).unwrap().to_string(),
        });
        event.push(new_vec);
    } 
    let res_json:Value = json!(Into::<Value>::into(event));
    
    send_response(
        200,
        vec![(String::from("content-type"), String::from("application/json"))],
        serde_json::to_vec_pretty(&res_json).unwrap(),
    );
}
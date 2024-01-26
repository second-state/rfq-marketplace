use webhook_flows::{create_endpoint, request_handler, send_response, route::{get, post, route, RouteError, Router}};
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
            "/submit-request",
            vec![post(submit_request)],
        )
        .unwrap();

    router
        .insert(
            "/submit-response",
            vec![post(submit_response)],
        )
        .unwrap();

    router
        .insert(
            "/accept-exchange",
            vec![post(accept_exchange)],
        )
        .unwrap();
    
    router
        .insert(
            "/withdraw",
            vec![post(withdraw)],
        )
        .unwrap();
    
    router
        .insert(
            "/list-requests",
            vec![get(list_requests)],
        )
        .unwrap();

    router
        .insert(
            "/get-request",
            vec![get(get_request)],
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

fn init_rpc(path: &str, _qry: &HashMap<String, Value>, _body: Vec<u8>) -> (String, u64, NameOrAddress, LocalWallet){
    logger::init();
    log::info!("{} Query -- {:?}", path, _qry);
    
    let rpc_node_url = std::env::var("RPC_NODE_URL").unwrap_or("https://mainnet.cybermiles.io".to_string());
    let chain_id = std::env::var("CHAIN_ID").unwrap_or("18".to_string()).parse::<u64>().unwrap_or(18u64);
    let contract_address = NameOrAddress::from(H160::from_str(std::env::var("CONTRACT_ADDRESS").unwrap().as_str()).unwrap());
    let mut wallet: LocalWallet = LocalWallet::new(&mut thread_rng());
    
    let private_key = String::from_utf8(_body)
    .ok() 
    .and_then(|body_str| serde_json::from_str::<Value>(&body_str).ok())
    .and_then(|json| json.get("PRIVATE_KEY").and_then(|v| v.as_str()).map(|s| s.to_string()));

    if let Some(private_key) = private_key {
        log::info!("key -- {:?}", private_key);
        wallet = private_key.as_str()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    }
    return (rpc_node_url, chain_id, contract_address, wallet);
}

async fn submit_request(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("submit_request", &_qry, _body);
    let token_a = H160::from_str(_qry.get("tokenA").unwrap().as_str().unwrap()).unwrap();
    let token_b = H160::from_str(_qry.get("tokenB").unwrap().as_str().unwrap()).unwrap();
    let amount =  U256::from_dec_str(_qry.get("amount").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Address(token_a.into()), Token::Address(token_b.into()), Token::Uint(amount.into())];
    let data = create_contract_call_data("submitRequest", contract_call_params).unwrap();

    let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
    let resp =json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");

    log::info!("resp: {:#?}", resp);

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );
}

async fn submit_response(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("response_exchange", &_qry, _body);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let amount =  U256::from_dec_str(_qry.get("amount").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(amount.into())];
    let data = create_contract_call_data("submitResponse", contract_call_params).unwrap();

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
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("accept_exchange", &_qry, _body);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let response_id =  U256::from_dec_str(_qry.get("response-id").unwrap().as_str().unwrap()).unwrap();
    let contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(response_id.into())];
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
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("withdraw", &_qry, _body);
    let request_id =  U256::from_dec_str(_qry.get("request-id").unwrap().as_str().unwrap()).unwrap();
    let address = format!("{:?}", wallet.address());
    let mut is_owner = false;
    let mut contract_call_params :Vec<Token> = vec!();
    let log = get_log(&rpc_node_url, format!("{:?}", contract_address.as_address().unwrap()).as_str(), json!(["0xb981be592ff12d76d951facfbbe36a4fd0607fef8ab19502903f32c5fe451460", null, format!("{:#066x}", request_id.as_usize())])).await.unwrap();
    if format!("0x{}", &(log[0]["topics"][1].to_string()).trim_matches('"')[26..]) == address {
        is_owner = true;
    }
    if is_owner {
        contract_call_params = vec![Token::Uint(request_id.into())];
    }else{
        let log = get_log(&rpc_node_url, format!("{:?}", contract_address.as_address().unwrap()).as_str(), json!(["0x5f809e0f670ff1d5d393b4775ee4f31f942aa16ca64ad7b62b25a95920fa37d1", null, format!("{:#066x}", request_id.as_usize())])).await.unwrap();
        let len = log.as_array().unwrap().len();
        for idx in 0..len{
            let now = log.get(idx).unwrap();
            if address == format!("0x{}", &now["data"].as_str().unwrap()[26..66]) {
                let response_id: U256 = U256::from_str(&(now["topics"][1].to_string()).trim_matches('"')[26..]).unwrap();
                contract_call_params = vec![Token::Uint(request_id.into()), Token::Uint(response_id.into())];
                break;
            }
        }
    }
    let resp: String;

    if contract_call_params.len() != 0 {
        let data = create_contract_call_data("withdraw", contract_call_params).unwrap();
        
        let tx_params = json!([wrap_transaction(&rpc_node_url, chain_id, wallet, contract_address, data, U256::from(0)).await.unwrap().as_str()]);
        resp = json_rpc(&rpc_node_url, "eth_sendRawTransaction", tx_params).await.expect("Failed to send raw transaction.");
    } else {
        resp = "Not fund!".to_string();
    }
    
    log::info!("resp: {:#?}", resp);
    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.into_bytes().to_vec(),
    );

}

async fn list_requests(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, _, contract_address, _) = init_rpc("list_requests", &_qry, _body);
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

async fn get_request(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, _, contract_address, _) = init_rpc("get_request", &_qry, _body);
    let contract_address = format!("{:?}", contract_address.as_address().unwrap());
    // Keccak-256 bidEvent(uint256,address,uint256,uint256)
    let log = get_log(&rpc_node_url, &contract_address, json!(["0x5f809e0f670ff1d5d393b4775ee4f31f942aa16ca64ad7b62b25a95920fa37d1"])).await.unwrap();
    let mut event: Vec<Value> = vec!();
    let len = log.as_array().unwrap().len();
    for idx in 0..len{
        let now = log.get(idx).unwrap();
        let new_vec = json!({
            "responseId": U256::from_str(&(now["topics"][1].to_string()).trim_matches('"')[26..]).unwrap().to_string(),
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
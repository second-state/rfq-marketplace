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
    if let Some(private_key) = _qry.get("private-key") {
        let private_key = private_key.as_str().unwrap();
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
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("withdraw", &_qry);

}

async fn get_response(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>){
    let (rpc_node_url, chain_id, contract_address, wallet) = init_rpc("withdraw", &_qry);
    
}
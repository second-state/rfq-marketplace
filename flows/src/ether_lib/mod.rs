use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use ethers_signers::{LocalWallet, Signer};
use ethers_core::types::{NameOrAddress, Bytes, U256, U64, TransactionRequest, transaction::eip2718::TypedTransaction};
use ethers_core::abi::{Abi, Function, Token};
use ethers_core::utils::hex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn create_contract_call_data(name: &str, tokens: Vec<Token>) -> Result<Bytes> {
    
    let contract_abi: &str = include_str!("../abi.json");
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

pub async fn eth_call(rpc_node_url: &str, from: &str, to: &str, data: &str) -> Result<String> {
	let params = json!([{"from": from, "to": to, "data": data}, "latest"]);
	let result = json_rpc(rpc_node_url, "eth_call", params).await.expect("Failed to send json.");

	Ok(result)
}

pub async fn eth_get_block_by_hash(rpc_node_url: &str, hash: &str) -> Result<Value>{
	let params = json!([hash, false]);
	let result = json_rpc(rpc_node_url, "eth_getBlockByHash", params).await.expect("Failed to send json.");
	Ok(serde_json::from_str(&result).unwrap())
}

pub async fn eth_get_tx_by_hash(rpc_node_url: &str, hash: &str) -> Result<Value>{
	let params = json!([hash]);
	let result = json_rpc(rpc_node_url, "eth_getTransactionByHash", params).await.expect("Failed to send json.");
	Ok(serde_json::from_str(&result).unwrap())
}

pub async fn get_ethbalance(rpc_node_url: &str, address: &str) -> Result<U256> {
	let params = json!([address, "latest"]);
	let result = json_rpc(rpc_node_url, "eth_getBalance", params).await.expect("Failed to send json.");
	Ok(U256::from_str(&result)?)
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

pub async fn get_log(rpc_node_url: &str, address: &str, topic: Value) -> Result<Value>{
	let params = json!([{"address": address, "fromBlock": "earliest", "topics":topic}]);
	let result = json_rpc(rpc_node_url, "eth_getLogs", params).await.expect("Failed to send json.");
	Ok(serde_json::from_str(&result).unwrap())
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
		println!("{} request body: {:#?}", method, json!({
			"jsonrpc": "2.0",
			"method": method,
			"params": params,
			"id": 1
		}));
		println!("{} response body: {:#?}", method, map);
	}
	Ok(serde_json::to_string(&map["result"]).expect("Failed to parse str.").trim_matches('"').to_string())
}
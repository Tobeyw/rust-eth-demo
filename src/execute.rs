 use std::{sync::Arc, error::Error};

/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-06-16 16:21:05
 */
use ethers::{prelude::*};
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;
abigen!(
    MyNFT,
    "abi/my.json",
    event_derives(serde::Deserialize, serde::Serialize)
);


#[derive(Debug, Serialize, Deserialize)]
struct TestContract {
    MyNFT: String,
    Account : String,
}

pub async fn mint(client: &Client)->Result<(),Box<dyn Error>> {

    let file = std::fs::File::open("address/contract.yaml").expect("Could not open file.");

    let test_contract: TestContract = serde_yaml::from_reader(file).expect("Could not read values.");
        
    let contract_address = test_contract.MyNFT.parse::<Address>()?;
   
    let contract = MyNFT::new(contract_address,Arc::new(client.clone()));
    
    let to = test_contract.Account.parse::<Address>().unwrap();
    let token_id = U256::from(1);
    let url  = "https://ipfs.io/ipfs/bafybeihzn4wkezoa35g7wwl3kxk6tr2i34jvfx2esqn4tpdx5iwm52c2vm/1.json".to_string();

    let mint_tx = contract.mint(to, token_id, url).send().await?.await?;
    print!("tx receipt{}", serde_json::to_string(&mint_tx)?);
 
Ok(())
}
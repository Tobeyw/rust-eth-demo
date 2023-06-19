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
    
    let _to = test_contract.Account.parse::<Address>().unwrap();
    let _tokenid = U256::from(4);
    let _url  = "https://ipfs.io/ipfs/bafybeihzn4wkezoa35g7wwl3kxk6tr2i34jvfx2esqn4tpdx5iwm52c2vm/4.json".to_string();

    let mint_tx = contract.mint(_to, _tokenid, _url).send().await?.await?;
    println!("tx receipt{}", serde_json::to_string(&mint_tx)?);
 
   Ok(())
}
pub async  fn balance_of(client:&Client) ->Result<(),Box<dyn Error>>{
    let file = std::fs::File::open("address/contract.yaml").expect("Could not open file.");

    let test_contract: TestContract = serde_yaml::from_reader(file).expect("Could not read values.");
        
    let contract_address = test_contract.MyNFT.parse::<Address>()?;
   
    let contract = MyNFT::new(contract_address,Arc::new(client.clone()));
    
    let _owner = test_contract.Account.parse::<Address>().unwrap();
  

    let balanceof = contract.balance_of(_owner).call().await?;
    println!("balanceOf:{}", serde_json::to_string(&balanceof)?);
 
   Ok(())
}



pub async  fn safe_tranfer_from(client:&Client) ->Result<(),Box<dyn Error>>{
    let file = std::fs::File::open("address/contract.yaml").expect("Could not open file.");

    let test_contract: TestContract = serde_yaml::from_reader(file).expect("Could not read values.");
        
    let contract_address = test_contract.MyNFT.parse::<Address>()?;
   
    let contract = MyNFT::new(contract_address,Arc::new(client.clone()));
    
    let _from =  "0xA83e88525eAaA73bC831455A2A6b41F88eEC1e75".parse::<Address>().unwrap();
    let _to =  "0xcf41E8a906bFc1b8eB8ABE6e073353b942a5d363".parse::<Address>().unwrap();
    let _tokenid = U256::from(2);


    let transfer_tx = contract.safe_transfer_from(_from, _to, _tokenid).send().await?.await?;
    print!("tx receipt{}", serde_json::to_string(&transfer_tx)?);
 
   Ok(())
}


 use std::{sync::Arc, error::Error};

/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-06-16 16:21:05
 */
use ethers::{prelude::{*, k256::elliptic_curve::consts::U160}};
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

pub async fn mint(client: &Client,to:H160,tokenid:U256,url:String)->Result<(),Box<dyn Error>> {

    let file = std::fs::File::open("address/contract.yaml").expect("Could not open file.");

    let test_contract: TestContract = serde_yaml::from_reader(file).expect("Could not read values.");
        
    let contract_address = test_contract.MyNFT.parse::<Address>()?;
   
    let contract = MyNFT::new(contract_address,Arc::new(client.clone()));
    
    let mint_tx = contract.mint(to, tokenid, url).send().await?.await?;
    println!("tx receipt{}", serde_json::to_string(&mint_tx)?);
 
   Ok(())
}
pub async  fn balance_of(client:&Client,owner:H160) ->Result<(),Box<dyn Error>>{
    let file = std::fs::File::open("address/contract.yaml").expect("Could not open file.");

    let test_contract: TestContract = serde_yaml::from_reader(file).expect("Could not read values.");
        
    let contract_address = test_contract.MyNFT.parse::<Address>()?;
   
    let contract = MyNFT::new(contract_address,Arc::new(client.clone()));
    
    let balanceof = contract.balance_of(owner).call().await?;
    println!("balanceOf:{}", serde_json::to_string(&balanceof)?);
 
   Ok(())
}



pub async  fn safe_tranfer_from(client:&Client,from:H160,to:H160,tokenid:U256) ->Result<(),Box<dyn Error>>{
    let file = std::fs::File::open("address/contract.yaml").expect("Could not open file.");

    let test_contract: TestContract = serde_yaml::from_reader(file).expect("Could not read values.");
        
    let contract_address = test_contract.MyNFT.parse::<Address>()?;
   
    let contract = MyNFT::new(contract_address,Arc::new(client.clone()));
    
    let transfer_tx = contract.safe_transfer_from(from, to, tokenid).send().await?.await?;
    print!("tx receipt{}", serde_json::to_string(&transfer_tx)?);
 
   Ok(())
}


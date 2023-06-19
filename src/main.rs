/*
 * @Descripttion: 
 * @version: 
 * @Author: Mindy
 * @Date: 2023-06-15 15:21:53
 */
use ethers::{prelude::*};
use rust_eth_demo::execute;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    const RPC_URL: &str = "https://sepolia.infura.io/v3/cf25bc9652db4640b83e23517e8e5594";
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("{block_number}");

    let wallet: LocalWallet = "908cb93129f920d7f9575962e074b5591cf0e19540041ca147bae7dbf53569b9"
            .parse::<LocalWallet>()?.with_chain_id(11155111u64);
    
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());


    let _to =  "0xA83e88525eAaA73bC831455A2A6b41F88eEC1e75".parse::<Address>().unwrap();
    let _tokenid = U256::from(6);
    let _url  = "https://ipfs.io/ipfs/bafybeihzn4wkezoa35g7wwl3kxk6tr2i34jvfx2esqn4tpdx5iwm52c2vm/6.json".to_string();
    execute::mint(&client,_to,_tokenid,_url).await?;


    let _owner =  "0xA83e88525eAaA73bC831455A2A6b41F88eEC1e75".parse::<Address>().unwrap();
    execute::balance_of(&client,_owner).await?;   


    let _from =  "0xA83e88525eAaA73bC831455A2A6b41F88eEC1e75".parse::<Address>().unwrap();
    let _to =  "0xcf41E8a906bFc1b8eB8ABE6e073353b942a5d363".parse::<Address>().unwrap();
    let _tokenid = U256::from(2);
    execute::safe_tranfer_from(&client,_from,_to,_tokenid).await?;


    Ok(())
}
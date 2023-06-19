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

    execute::mint(&client).await?;
    execute::balance_of(&client).await?;   
    execute::safe_tranfer_from(&client).await?;


    Ok(())
}
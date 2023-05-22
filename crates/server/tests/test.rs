// use clarity::Uint256;
// use cosmrs::proto::cosmos::tx::v1beta1::GetTxRequest;
// use prost_10::Message;

#[tokio::test]
async fn test() -> util::Result<()> {
    // let client =
    //     server::util::CosmosClient::new("https://canto.gravitychain.io:9090".to_string())
    //         .await?;
    // let resp = client
    //     .tx_client
    //     .clone()
    //     .get_tx(GetTxRequest {
    //         hash: "EF93F74F0E73E7CDEE4CED8A2AF965508B1926A31E2BC2AA90C1052285374CEC".to_string(),
    //     })
    //     .await?;

    // let tx = resp.get_ref().tx.as_ref().unwrap().clone();

    // for msg in tx.body.unwrap().messages {
    //     if msg.type_url == "/ethermint.evm.v1.MsgEthereumTx" {
    //         if let Ok(eth_tx) =
    //             althea_proto::ethermint::evm::v1::MsgEthereumTx::decode(msg.value.as_slice())
    //         {
    //             let data = eth_tx.data.ok_or("could not find eth tx data")?;
    //             let tx = althea_proto::ethermint::evm::v1::LegacyTx::decode(data.value.as_slice())?;

    //             let sig = clarity::Signature::new(
    //                 Uint256::from_be_bytes(&tx.v),
    //                 Uint256::from_be_bytes(&tx.r),
    //                 Uint256::from_be_bytes(&tx.s),
    //             );

    //             // "0xf1829676DB577682E944fc3493d451B67Ff3E29F" - expected

    //             let addr = sig
    //                 .recover(clarity::utils::hex_str_to_bytes(&eth_tx.hash)?.as_slice())?
    //                 .to_string();

    //             println!("{:?}", addr);
    //         }
    //     }
    // }

    Ok(())
}

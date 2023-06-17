use fuels::{prelude::*, types::ContractId};

// Load abi from json
abigen!(Contract(
    name = "Counter",
    abi = "out/debug/test-contract-counter-abi.json"
));

async fn get_contract_instance() -> (Counter<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/test-contract-counter.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxParameters::default())
    .await
    .unwrap();

    let instance = Counter::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;
    let result = _instance.methods().get_count().call().await.unwrap();

    assert_eq!(1, result.value);
}

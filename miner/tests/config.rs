//! Functional test for delay module

#![forbid(unsafe_code)]
use libra_types::PeerId;
use miner::config::*;
use abscissa_core::path::PathBuf;
use ol_cli::config::{
    Workspace,
    Profile,
    ChainInfo,
};
#[test]
fn test_genesis_preimage() {
    let mut configs = miner::block::build_block::test_make_configs_fixture();
    // // Create fixtures.
    // let configs = MinerConfig {
    //     workspace: Workspace {
    //         node_home: PathBuf::from("."),
    //     },
    //     profile: Profile {
    //         auth_key: "3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2".to_owned(),
    //         account: PeerId::from_hex_literal("0x000000000000000000000000deadbeef").unwrap(),
    //         ip: "1.1.1.1".parse().unwrap(),
    //         statement: "Protests rage across the Nation".to_owned(),
    //     },
    //     chain_info: ChainInfo {
    //         chain_id: "0L testnet".to_owned(),
    //         block_dir: "blocks".to_owned(),
    //         base_waypoint: None,
    //         default_node: None,
    //         upstream_nodes: None,
    //     },
    // };

    let first_preimage = miner::block::genesis_preimage();

    println!("proof:\n{:?}", hex::encode(&first_preimage));

    let correct_preimage = "3e4629ba1e63114b59a161e89ad4a083b3a31b5fd59e39757c493e96398e4df2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000304c20746573746e65746400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865204e6174696f6e";

    assert_eq!(hex::encode(&first_preimage), correct_preimage, "Preimages do not match")
}

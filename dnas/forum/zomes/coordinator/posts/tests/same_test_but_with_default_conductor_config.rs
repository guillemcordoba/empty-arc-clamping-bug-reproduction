use std::sync::Arc;

use hdk::prelude::*;
use holochain::prelude::dependencies::kitsune_p2p_types::config::{
    tuning_params_struct::KitsuneP2pTuningParams, KitsuneP2pConfig,
};
use holochain::{conductor::config::ConductorConfig, sweettest::*};
use posts::post_to_referreds::*;
use posts_integrity::*;

#[tokio::test(flavor = "multi_thread")]
async fn default_config_leads_to_test_success() {
    // Use prebuilt DNA file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../../workdir/forum.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    let mut empty_arc_conductor_config = ConductorConfig::default();

    let mut network_config = KitsuneP2pConfig::default();

    let mut tuning_params = KitsuneP2pTuningParams::default();

    // tuning_params.gossip_arc_clamping = String::from("empty");
    network_config.tuning_params = Arc::new(tuning_params);

    empty_arc_conductor_config.network = network_config;

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_configs(vec![
        ConductorConfig::default(),
        empty_arc_conductor_config,
    ])
    .await;
    let apps = conductors.setup_app("posts", &[dna]).await.unwrap();
    conductors.exchange_peer_info().await;

    let ((alice,), (bobbo,)) = apps.into_tuples();

    let alice_zome = alice.zome("posts");
    let bob_zome = bobbo.zome("posts");

    let alice_pub_key = alice.agent_pubkey();

    // Try to get my profile before creating one. Should return None.
    let record_1: Record = conductors[0]
        .call(
            &alice_zome,
            "create_post",
            Post {
                content: String::from("hello"),
            },
        )
        .await;

    await_consistency(120, vec![&alice, &bobbo]).await.unwrap();

    let input = AddReferredForPostInput {
        base_post_hash: record_1.action_address().clone(),
        target_referred: alice_pub_key.clone(),
    };
    let _: () = conductors[1]
        .call(&bob_zome, "add_referred_for_post", input)
        .await;

    let input = RemoveReferredForPostInput {
        base_post_hash: record_1.action_address().clone(),
        target_referred: alice_pub_key.clone(),
    };
    let _: () = conductors[1]
        .call(&bob_zome, "remove_referred_for_post", input)
        .await;
}

use hdk::prelude::*;
use posts_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddReferredForPostInput {
    pub base_post_hash: ActionHash,
    pub target_referred: AgentPubKey,
}

#[hdk_extern]
pub fn add_referred_for_post(input: AddReferredForPostInput) -> ExternResult<()> {
    create_link(
        input.base_post_hash.clone(),
        input.target_referred.clone(),
        LinkTypes::PostToReferreds,
        (),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_referreds_for_post(post_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(GetLinksInputBuilder::try_new(post_hash, LinkTypes::PostToReferreds)?.build())
}

#[hdk_extern]
pub fn get_deleted_referreds_for_post(
    post_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        post_hash,
        LinkTypes::PostToReferreds,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveReferredForPostInput {
    pub base_post_hash: ActionHash,
    pub target_referred: AgentPubKey,
}

#[hdk_extern]
pub fn remove_referred_for_post(input: RemoveReferredForPostInput) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(input.base_post_hash.clone(), LinkTypes::PostToReferreds)?
            .build(),
    )?;

    for link in links {
        if AgentPubKey::from(link.target.clone().into_entry_hash().ok_or(wasm_error!(
            WasmErrorInner::Guest("No entry_hash associated with link".to_string())
        ))?)
        .eq(&input.target_referred)
        {
            delete_link(link.create_link_hash)?;
        }
    }

    Ok(())
}

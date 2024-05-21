use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<Record> {
    let post_hash = create_entry(&EntryTypes::Post(post.clone()))?;

    let record = get(post_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Post".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_post(post_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(post_hash, GetOptions::default())? else {
                    return Ok(None);
                };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}

#[hdk_extern]
pub fn delete_post(original_post_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_post_hash)
}

#[hdk_extern]
pub fn get_all_deletes_for_post(
    original_post_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_post_hash, GetOptions::default())? else {
                return Ok(None);
            };

    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_post(
    original_post_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_post(original_post_hash)? else {
                return Ok(None);
            };

    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });

    Ok(deletes.first().cloned())
}

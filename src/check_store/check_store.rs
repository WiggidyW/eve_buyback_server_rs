use crate::{
    error::{Error, FirestoreError},
    pb,
    typedb::TypeDb,
};
use firestoredb::FirestoreDb;

pub async fn write(
    firestoredb: &FirestoreDb,
    hash: &str,
    rep: &pb::buyback::Rep,
) -> Result<(), Error> {
    Ok(firestoredb
        .write(hash, rep.clone())
        .await
        .map_err(|e| FirestoreError::Write(e))?)
}

pub async fn read(
    firestoredb: &FirestoreDb,
    hash: &str,
    language: &str,
    typedb: &impl TypeDb,
) -> Result<Option<pb::buyback::Rep>, Error> {
    match firestoredb.read::<pb::buyback::Rep>(hash).await {
        Ok(Some(rep)) => Ok(Some(with_named_items(rep, typedb, language).await?)),
        Ok(None) => Ok(None),
        Err(e) => Err(FirestoreError::Read(e).into()),
    }
}

async fn with_named_items(
    mut rep: pb::buyback::Rep,
    type_db: &impl TypeDb,
    language: &str,
) -> Result<pb::buyback::Rep, Error> {
    let mut name_futs = Vec::with_capacity(rep.items.len());
    for rep_item in &rep.items {
        name_futs.push(type_db.get_name(rep_item.type_id, language));
    }
    for (rep_item, name_fut) in rep.items.iter_mut().zip(name_futs) {
        rep_item.name = name_fut.await?;
    }
    Ok(rep)
}

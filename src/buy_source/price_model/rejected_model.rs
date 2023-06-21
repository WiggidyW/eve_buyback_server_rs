use crate::{error::Error, pb, typedb::TypeDb};

pub async fn get_rejected_rep_item(
    type_db: &impl TypeDb,
    language: &str,
    type_id: u32,
    parent_type_id: u32,
    quantity: f64,
) -> Result<pb::buyback::RepItem, Error> {
    Ok(pb::buyback::RepItem {
        type_id: type_id,
        parent_type_id: parent_type_id,
        quantity: quantity,
        name: type_db.get_name(type_id, language).await?,
        price_per: 0.0,
        description: "Rejected".to_string(),
        accepted: false,
        meta: None,
    })
}

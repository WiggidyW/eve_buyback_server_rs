use super::{price_model::PriceModelResponse, MarketNameStore, PriceModelStore, TokenStore};
use crate::{error::Error, pb, typedb::TypeDb};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub async fn buy(
    req: pb::buyback::BuyReq,
    typedb: &impl TypeDb,
    market_store: &impl MarketNameStore,
    token_store: &impl TokenStore,
    model_store: &impl PriceModelStore,
    client: &pb::WeveEsiClient,
) -> Result<pb::buyback::Rep, Error> {
    let mut rep_items = Vec::with_capacity(req.items.len());
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let futs = req
        .items
        .into_iter()
        .map(|item| {
            model_store
                .get_price_model(&req.location, &item.type_id)
                .get_rep_items(
                    typedb,
                    &req.language,
                    item,
                    market_store,
                    token_store,
                    model_store,
                    client,
                    &req.location,
                )
        })
        .collect::<Vec<_>>();

    let version = model_store.get_version();
    let mut sum = 0.0;

    let mut hasher = DefaultHasher::new();
    req.location.hash(&mut hasher);
    version.hash(&mut hasher);

    for fut in futs {
        match fut.await? {
            PriceModelResponse::Single(item) => {
                item.type_id.hash(&mut hasher);
                sum += item.price_per * item.quantity;
                rep_items.push(item);
            }
            PriceModelResponse::Multi(items) => {
                sum += items[0].price_per * items[0].quantity;
                rep_items.reserve(items.len() - 1);
                for item in items {
                    item.type_id.hash(&mut hasher);
                    rep_items.push(item);
                }
            }
        };
    }

    Ok(pb::buyback::Rep {
        items: rep_items,
        hash: {
            if sum > 0.0 {
                (sum as u64).hash(&mut hasher);
                format!("{:x}", hasher.finish())
            } else {
                "".to_string()
            }
        },
        sum: sum,
        timestamp: timestamp,
        version: version.to_string(),
        location: req.location,
    })
}

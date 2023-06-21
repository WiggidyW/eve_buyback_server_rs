use super::{
    super::{MarketNameStore, PriceModelStore, TokenStore},
    item_model::ItemModel,
    rejected_model::get_rejected_rep_item,
    reprocess_model::ReprocessModel,
};
use crate::{error::Error, pb, typedb::TypeDb};

#[derive(Debug)]
pub enum PriceModelResponse {
    Single(pb::buyback::RepItem),
    Multi(Vec<pb::buyback::RepItem>),
}

#[derive(Debug)]
pub enum PriceModel {
    Rejected,
    Item(ItemModel),
    Reprocess(ReprocessModel),
    ReprocessAs(ItemModel, ReprocessModel),
}

impl PriceModel {
    pub async fn get_rep_items(
        self,
        type_db: &impl TypeDb,
        language: &str,
        req_item: pb::buyback::ReqItem,
        market_store: &impl MarketNameStore,
        token_store: &impl TokenStore,
        model_store: &impl PriceModelStore,
        client: &pb::WeveEsiClient,
        region: &str,
    ) -> Result<PriceModelResponse, Error> {
        Ok(match self {
            PriceModel::Rejected => PriceModelResponse::Single(
                get_rejected_rep_item(
                    type_db,
                    language,
                    req_item.type_id,
                    req_item.type_id,
                    req_item.quantity as f64,
                )
                .await?,
            ),
            PriceModel::Item(i) => PriceModelResponse::Single(
                i.get_rep_item(
                    type_db,
                    language,
                    req_item.type_id,
                    req_item.type_id,
                    req_item.quantity as f64,
                    market_store,
                    token_store,
                    client,
                )
                .await?,
            ),
            PriceModel::Reprocess(r) => PriceModelResponse::Multi(
                r.get_rep_items(
                    type_db,
                    language,
                    req_item.type_id,
                    req_item.quantity as f64,
                    market_store,
                    token_store,
                    client,
                    model_store,
                    region,
                )
                .await?,
            ),
            PriceModel::ReprocessAs(i, r) => PriceModelResponse::Multi(
                r.get_rep_items_as(
                    type_db,
                    language,
                    req_item.type_id,
                    req_item.quantity as f64,
                    market_store,
                    token_store,
                    client,
                    i,
                )
                .await?,
            ),
        })
    }
}

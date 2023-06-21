use super::{
    super::{MarketNameStore, PriceModelStore, TokenStore},
    item_model::ItemModel,
    price_model::PriceModel,
    rejected_model::get_rejected_rep_item,
};
use crate::{error::Error, pb, typedb::TypeDb};
use either::Either;

#[derive(Debug)]
pub struct ReprocessModel {
    pub efficiency: f64,
}

impl ReprocessModel {
    pub async fn get_rep_items(
        self,
        type_db: &impl TypeDb,
        language: &str,
        type_id: u32,
        quantity: f64,
        market_store: &impl MarketNameStore,
        token_store: &impl TokenStore,
        client: &pb::WeveEsiClient,
        model_store: &impl PriceModelStore,
        region: &str,
    ) -> Result<Vec<pb::buyback::RepItem>, Error> {
        let children_fut = type_db.get_reprocess(type_id);
        let parent_name_fut = type_db.get_name(type_id, language);

        let mut sum = 0.0;

        let children = children_fut.await?;
        let mut rep_items = Vec::with_capacity(children.len() + 1);
        rep_items.push(pb::buyback::RepItem {
            type_id: type_id,
            parent_type_id: type_id,
            quantity: quantity,
            name: parent_name_fut.await?,
            price_per: 0.0,
            description: format!("{}% Reprocessed", self.efficiency * 100.0),
            accepted: false,
            meta: None,
        });

        for either_fut in children
            .into_iter()
            .map(|(child_type_id, child_quantity)| {
                match model_store.get_price_model(region, &child_type_id) {
                    PriceModel::Item(item_model) => Either::Left(item_model.get_rep_item(
                        type_db,
                        language,
                        child_type_id,
                        type_id,
                        child_quantity * quantity * self.efficiency,
                        market_store,
                        token_store,
                        client,
                    )),
                    _ => Either::Right(get_rejected_rep_item(
                        type_db,
                        language,
                        child_type_id,
                        type_id,
                        child_quantity * quantity * self.efficiency,
                    )),
                }
            })
            .collect::<Vec<Either<_, _>>>()
        {
            rep_items.push(match either_fut {
                Either::Left(item_fut) => {
                    let rep_item = item_fut.await?;
                    sum += rep_item.price_per * rep_item.quantity;
                    rep_item
                }
                Either::Right(rejected_fut) => rejected_fut.await?,
            });
        }

        // Update the parent_rep_item in place
        let mut parent_rep_item = rep_items.get_mut(0).unwrap();
        if sum > 0.0 {
            parent_rep_item.price_per = sum / quantity;
            parent_rep_item.accepted = true;
        } else {
            parent_rep_item.description = format!(
                "{} - All reprocessed items rejected",
                parent_rep_item.description,
            );
        }

        // Return the rep_items
        Ok(rep_items)
    }

    pub async fn get_rep_items_as(
        self,
        type_db: &impl TypeDb,
        language: &str,
        type_id: u32,
        quantity: f64,
        market_store: &impl MarketNameStore,
        token_store: &impl TokenStore,
        client: &pb::WeveEsiClient,
        item_model: ItemModel,
    ) -> Result<Vec<pb::buyback::RepItem>, Error> {
        let children_fut = type_db.get_reprocess(type_id);
        let parent_name_fut = type_db.get_name(type_id, language);

        let market = market_store.get_market_name(&item_model.location);
        let token = token_store.get_token(&item_model.location);
        let mut sum = 0.0;

        let children = children_fut.await?;
        let mut rep_items = Vec::with_capacity(children.len() + 1);
        rep_items.push(pb::buyback::RepItem {
            type_id: type_id,
            parent_type_id: type_id,
            quantity: quantity,
            name: parent_name_fut.await?,
            price_per: 0.0,
            description: format!("{}% Reprocessed", self.efficiency * 100.0),
            accepted: false,
            meta: None,
        });

        for fut in children
            .into_iter()
            .map(|(child_type_id, child_quantity)| {
                item_model.clone().get_rep_item_as(
                    type_db,
                    language,
                    child_type_id,
                    type_id,
                    child_quantity * quantity * self.efficiency,
                    market.clone(),
                    token.clone(),
                    client,
                )
            })
            .collect::<Vec<_>>()
        {
            let rep_item = fut.await?;
            sum += rep_item.price_per * rep_item.quantity;
            rep_items.push(rep_item);
        }

        // Update the parent_rep_item in place
        let mut parent_rep_item = rep_items.get_mut(0).unwrap();
        if sum > 0.0 {
            parent_rep_item.price_per = sum / quantity;
            parent_rep_item.accepted = true;
        } else {
            parent_rep_item.description = format!(
                "{} - All reprocessed items rejected",
                parent_rep_item.description,
            );
        }

        // Return the rep_items
        Ok(rep_items)
    }
}

use super::{
    super::{MarketNameStore, TokenStore},
    get_price::get_price,
    order_target::OrderTarget,
};
use crate::{error::Error, pb, typedb::TypeDb};

#[derive(Debug, Clone)]
pub struct ItemModel {
    pub order_target: OrderTarget,
    pub modifier: f64,
    pub location: u64,
}

impl ItemModel {
    pub async fn get_rep_item(
        self,
        type_db: &impl TypeDb,
        language: &str,
        type_id: u32,
        parent_type_id: u32,
        quantity: f64,
        market_store: &impl MarketNameStore,
        token_store: &impl TokenStore,
        client: &pb::WeveEsiClient,
    ) -> Result<pb::buyback::RepItem, Error> {
        let market_name = market_store.get_market_name(&self.location);
        let refresh_token = token_store.get_token(&self.location);
        self.get_rep_item_as(
            type_db,
            language,
            type_id,
            parent_type_id,
            quantity,
            market_name,
            refresh_token,
            client,
        )
        .await
    }

    pub async fn get_rep_item_as(
        self,
        type_db: &impl TypeDb,
        language: &str,
        type_id: u32,
        parent_type_id: u32,
        quantity: f64,
        market: String,
        refresh_token: String,
        client: &pb::WeveEsiClient,
    ) -> Result<pb::buyback::RepItem, Error> {
        let price_fut = get_price(
            client,
            refresh_token,
            type_id,
            self.location,
            self.order_target,
        );
        let name_fut = type_db.get_name(type_id, language);

        let (accepted, price_per, description) = match price_fut.await? {
            Some(p) => (
                true,
                p * self.modifier,
                format!(
                    "{} {}% {}",
                    market,
                    self.modifier * 100.0,
                    self.order_target.to_string(),
                ),
            ),
            None => (
                false,
                0.0,
                format!(
                    "{} {}% {} - No orders found at {}",
                    market,
                    self.modifier * 100.0,
                    self.order_target.to_string(),
                    market,
                ),
            ),
        };

        Ok(pb::buyback::RepItem {
            type_id: type_id,
            parent_type_id: parent_type_id,
            quantity: quantity,
            name: name_fut.await?,
            price_per: price_per,
            description: description,
            accepted: accepted,
            meta: None,
        })
    }
}

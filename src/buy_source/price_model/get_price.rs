use super::order_target::OrderTarget;
use crate::{error::Error, pb};

pub async fn get_price(
    client: &pb::WeveEsiClient,
    refresh_token: String,
    type_id: u32,
    location: u64,
    order_target: OrderTarget,
) -> Result<Option<f64>, Error> {
    let orders = client
        .clone()
        .market_orders(pb::weve_esi::MarketOrdersReq {
            location_id: location,
            type_id: type_id,
            token: refresh_token,
            buy: order_target.is_buy(),
        })
        .await
        .map_err(|e| Error::WeveEsi(e))?
        .into_inner()
        .inner;
    Ok(get_price_from_orders(orders, order_target))
}

fn get_price_from_orders(
    orders: Vec<pb::weve_esi::MarketOrder>,
    order_target: OrderTarget,
) -> Option<f64> {
    match (orders.len(), order_target) {
        (0, _) => None,
        (_, OrderTarget::MinSell) => {
            let mut highest = orders[0].price;
            for order in orders {
                if order.price > highest {
                    highest = order.price;
                }
            }
            Some(highest)
        }
        (_, OrderTarget::MaxBuy) => {
            let mut lowest = orders[0].price;
            for order in orders {
                if order.price < lowest {
                    lowest = order.price;
                }
            }
            Some(lowest)
        }
    }
}

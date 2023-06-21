use super::{
    item_model::ItemModel, order_target::OrderTarget, reprocess_model::ReprocessModel, PriceModel,
};
use crate::static_map::{PRICE_VERSION, REGION_MAP};

pub trait PriceModelStore: Send + Sync + 'static {
    fn get_price_model(&self, region: &str, type_id: &u32) -> PriceModel;
    fn get_version(&self) -> &str;
}

pub fn new_price_model_store() -> impl PriceModelStore {
    PriceModelStoreMarker
}

#[derive(Debug)]
pub struct PriceModelStoreMarker;

impl PriceModelStore for PriceModelStoreMarker {
    fn get_version(&self) -> &str {
        PRICE_VERSION
    }

    fn get_price_model(&self, region: &str, type_id: &u32) -> PriceModel {
        if let Some(type_map) = REGION_MAP.get(region) {
            if let Some((efficiency, order_target, modifier, location)) = type_map.get(type_id) {
                return match (
                    item_model_from_map(*order_target, *modifier, *location),
                    reprocess_model_from_map(*efficiency),
                ) {
                    (Some(i), None) => PriceModel::Item(i),
                    (None, Some(r)) => PriceModel::Reprocess(r),
                    (Some(i), Some(r)) => PriceModel::ReprocessAs(i, r),
                    (None, None) => panic!("Invalid price model found for type_id: {}", type_id),
                };
            }
        }
        PriceModel::Rejected
    }
}

fn item_model_from_map(order_target: u8, modifier: u8, location: u64) -> Option<ItemModel> {
    Some(ItemModel {
        order_target: match order_target {
            1 => OrderTarget::MinSell,
            2 => OrderTarget::MaxBuy,
            _ => return None,
        },
        modifier: modifier as f64 / 100.0,
        location: location,
    })
}

fn reprocess_model_from_map(efficiency: u8) -> Option<ReprocessModel> {
    match efficiency {
        0 => None,
        _ => Some(ReprocessModel {
            efficiency: efficiency as f64 / 100.0,
        }),
    }
}

pub const PRICE_VERSION: &str = "2023-06-22 19:42:42";

pub static MARKET_MAP: phf::Map<u64, &str> = phf::phf_map! {
    1030049082711: "1DQ1-A",
    60003760: "Jita",
};

pub static REGION_MAP: phf::Map<&str, phf::Map<u32, (u8, u8, u8, u64)>> = phf::phf_map! {
};

#[derive(Debug, Clone, Copy)]
pub enum OrderTarget {
    MinSell,
    MaxBuy,
}

impl OrderTarget {
    pub fn is_buy(&self) -> bool {
        match self {
            OrderTarget::MinSell => false,
            OrderTarget::MaxBuy => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OrderTarget::MinSell => "MinSell",
            OrderTarget::MaxBuy => "MaxBuy",
        }
        .to_string()
    }
}

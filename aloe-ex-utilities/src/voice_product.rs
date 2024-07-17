crate::ix!();

pub struct VoiceProduct
{
    identifier:           *const u8,
    human_readable:       *const u8,
    is_purchased:         bool,
    price_is_known:       bool,
    purchase_in_progress: bool,
    purchase_price:       String,
}

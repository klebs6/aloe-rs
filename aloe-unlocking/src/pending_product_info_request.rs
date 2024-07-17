crate::ix!();

pub enum PendingProductInfoRequestType
{
    query = 0,
    purchase
}

/**
  | Represents a pending request initialised
  | with [SKProductRequest start].
  |
  */
pub struct PendingProductInfoRequest {
    ty:      PendingProductInfoRequestType,
    request: Box<SKProductsRequest,NSObjectDeleter>,
}

/**
  | Represents a pending request started
  | from [SKReceiptRefreshRequest start].
  |
  */
pub struct PendingReceiptRefreshRequest
{
    subscriptions_shared_secret: String,
    request:                     Box<SKReceiptRefreshRequest,NSObjectDeleter>,
}

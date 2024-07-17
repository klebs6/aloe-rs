/*!
  | Represents an object that gets notified
  | about events such as product info returned
  | or product purchase finished.
  |
  */

crate::ix!();

pub trait InAppPurchasesListenerInterface: 
    InAppPurchasesProductsInfoReturned 
    + InAppPurchasesProductPurchaseFinished
    + InAppPurchasesPurchaseListRestored
    + InAppPurchasesProductConsumed
    + InAppPurchasesProductDownloadProgressUpdate
    + InAppPurchasesProductDownloadPaused
    + InAppPurchasesProductDownloadFinished {}

/**
  | Structure holding purchase information
  |
  */
pub struct InAppPurchasesListenerPurchaseInfo
{
    purchase:  InAppPurchase,
    downloads: Vec<*mut dyn InAppPurchasesDownload>,
}

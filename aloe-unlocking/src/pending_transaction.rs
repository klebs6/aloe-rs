crate::ix!();

/**
  | Represents a transaction with pending
  | downloads. Only after all downloads
  | are finished, the transaction is marked
  | as finished.
  |
  */
pub struct PendingDownloadsTransaction {
    downloads:   Vec<Box<InAppPurchasesPimplDownloadImpl>>,
    transaction: *const SKPaymentTransaction,
}

impl PendingDownloadsTransaction {

    pub fn new(t: *mut SKPaymentTransaction) -> Self {
    
        todo!();
        /*
        : transaction(t),

            addDownloadsFromSKTransaction (transaction);
        */
    }
    
    pub fn add_downloads_from_sk_transaction(&mut self, transaction_to_use: *mut SKPaymentTransaction)  {
        
        todo!();
        /*
            for (SKDownload* download in transactionToUse.downloads)
                    downloads.add (new InAppPurchasesPimplDownloadImpl (download));
        */
    }
    
    pub fn can_be_marked_as_finished(&self) -> bool {
        
        todo!();
        /*
            for (SKDownload* d in transaction.downloads)
                {
                  #if ALOE_IOS
                    SKDownloadState state = d.downloadState;
                  #else
                    SKDownloadState state = d.state;
                  #endif
                    if (state != SKDownloadStateFinished
                         && state != SKDownloadStateFailed
                         && state != SKDownloadStateCancelled)
                    {
                        return false;
                    }
                }

                return true;
        */
    }
}

crate::ix!();

pub trait InAppPurchasesProductsInfoReturned {

    /**
      | Called whenever a product info is returned
      | after a call to InAppPurchases::getProductsInformation().
      |
      */
    fn products_info_returned(&mut self, products: &[InAppPurchasesProduct])  { }
}

pub trait InAppPurchasesProductPurchaseFinished {

    /**
      | Called whenever a purchase is complete,
      | with additional state whether the purchase
      | completed successfully.
      | 
      | For hosted content (iOS only), the downloads
      | array within PurchaseInfo will contain
      | all download objects corresponding
      | with the purchase. For non-hosted content,
      | the downloads array will be empty.
      | 
      | InAppPurchases class will own downloads
      | and will delete them as soon as they are
      | finished.
      | 
      | -----------
      | @note
      | 
      | It is possible to receive this callback
      | for the same purchase multiple times.
      | If that happens, only the newest set
      | of downloads and the newest orderId
      | will be valid, the old ones should be
      | not used anymore!
      |
      */
    fn product_purchase_finished(
        &mut self, 
        _0:                 &InAppPurchasesListenerPurchaseInfo,
        success:            bool,
        status_description: &String)  { }
}

pub trait InAppPurchasesPurchaseListRestored {

    /**
      | Called when a list of all purchases is
      | restored. This can be used to figure
      | out to which products a user is entitled
      | to.
      | 
      | -----------
      | @note
      | 
      | It is possible to receive this callback
      | for the same purchase multiple times.
      | If that happens, only the newest set
      | of downloads and the newest orderId
      | will be valid, the old ones should be
      | not used anymore!
      |
      */
    fn purchases_list_restored(
        &mut self, 
        _0:                 &[InAppPurchasesListenerPurchaseInfo],
        success:            bool,
        status_description: &String)  { }
}

pub trait InAppPurchasesProductConsumed {

    /**
      | Called whenever a product consumption
      | finishes.
      |
      */
    fn product_consumed(&mut self, 
        product_id:         &String,
        success:            bool,
        status_description: &String)  { }
}

pub trait InAppPurchasesProductDownloadProgressUpdate {

    /**
      | iOS only: Called when a product download
      | progress gets updated. If the download
      | was interrupted in the last application
      | session, this callback may be called
      | after the application starts.
      | 
      | If the download was in progress and the
      | application was closed, the download
      | may happily continue in the background
      | by OS. If you open the app and the download
      | is still in progress, you will receive
      | this callback.
      | 
      | If the download finishes in the background
      | before you start the app again, you will
      | receive productDownloadFinished
      | callback instead. The download will
      | only stop when it is explicitly cancelled
      | or when it is finished.
      |
      */
    fn product_download_progress_update(&mut self, 
        _0:             &mut dyn InAppPurchasesDownload,
        progress:       f32,
        time_remaining: RelativeTime)  { }
}

pub trait InAppPurchasesProductDownloadPaused {

    /**
      | iOS only: Called when a product download
      | is paused. This may also be called after
      | the application starts, if the download
      | was in a paused state and the application
      | was closed before finishing the download.
      | 
      | Only after the download is finished
      | successfully or cancelled you will
      | stop receiving this callback on startup.
      |
      */
    fn product_download_paused(&mut self, _0: &mut dyn InAppPurchasesDownload)  { }
}

pub trait InAppPurchasesProductDownloadFinished {

    /**
      | iOS only: Called when a product download
      | finishes (successfully or not). Call
      | InAppPurchasesDownload::getStatus() to check if
      | the downloaded finished successfully.
      | 
      | It is your responsibility to move the
      | download content into your app directory
      | and to clean up any files that are no longer
      | needed.
      | 
      | After the download is finished, the
      | download object is destroyed and should
      | not be accessed anymore.
      |
      */
    fn product_download_finished(
        &mut self, 
        _0:                      &mut dyn InAppPurchasesDownload,
        downloaded_content_path: &Url) { }
}

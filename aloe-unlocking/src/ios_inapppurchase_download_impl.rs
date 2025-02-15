crate::ix!();

/**
  | AppStore implementation of hosted
  | content download.
  |
  */
pub struct InAppPurchasesImplDownloadImpl {
    download: *mut SKDownload,
}

impl InAppPurchasesDownload for InAppPurchasesImplDownloadImpl {

    fn get_product_id(&self) -> String {
        
        todo!();
        /*
            return nsStringToAloe (download.contentIdentifier);
        */
    }
    
    fn get_content_version(&self) -> String {
        
        todo!();
        /*
            return nsStringToAloe (download.contentVersion);
        */
    }

    #[cfg(target_os="ios")]
    fn get_content_length(&self) -> i64 {
        
        todo!();
        /*
            return download.contentLength;
        */
    }
    
    #[cfg(target_os="ios")]
    fn get_status(&self) -> InAppPurchasesDownloadStatus {
        
        todo!();
        /*
            return SKDownloadStateToDownloadStatus (download.downloadState);
        */
    }

    #[cfg(not(target_os="ios"))]
    fn get_content_length(&self) -> i64 {
        
        todo!();
        /*
            return download.expectedContentLength;
        */
    }
    
    #[cfg(not(target_os="ios"))]
    fn get_status(&self) -> InAppPurchasesDownloadStatus {
        
        todo!();
        /*
            return SKDownloadStateToDownloadStatus (download.state);
        */
    }
}

impl Drop for InAppPurchasesImplDownloadImpl {

    fn drop(&mut self) {
        todo!();
        /*
            [download release];
        */
    }
}

impl InAppPurchasesImplDownloadImpl {

    pub fn new(download_to_use: *mut SKDownload) -> Self {
    
        todo!();
        /*
        : download(downloadToUse),

            [download retain];
        */
    }
}

crate::ix!();

pub enum InAppPurchasesDownloadStatus
{
    /**
      | The download is waiting to start. Called
      | at the beginning of a download operation.
      |
      */
    waiting = 0,    

    /**
      | The download is in progress.
      |
      */
    active,         

    /**
      | The download was paused and is awaiting
      | resuming or cancelling.
      |
      */
    paused,         

    /**
      | The download was finished successfully.
      |
      */
    finished,       

    /**
      | The download failed (e.g. because of
      | no internet connection).
      |
      */
    failed,         

    /**
      | The download was cancelled.
      |
      */
    cancelled,      

}

/**
  | iOS only: represents in-app purchase
  | download. InAppPurchasesDownload will be available
  | only for purchases that are hosted on
  | the AppStore.
  |
  */
pub trait InAppPurchasesDownload {

    /**
      | A unique identifier for the in-app product
      | to be downloaded.
      |
      */
    fn get_product_id(&self) -> String;

    /**
      | Content length in bytes.
      |
      */
    fn get_content_length(&self) -> i64;

    /**
      | Content version.
      |
      */
    fn get_content_version(&self) -> String;

    /**
      | Returns current status of the download.
      |
      */
    fn get_status(&self) -> InAppPurchasesDownloadStatus;

}

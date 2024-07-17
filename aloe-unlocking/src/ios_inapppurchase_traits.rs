crate::ix!();

pub trait SKDelegateAndPaymentObserverInterface: 
DidReceiveResponse
+ RequestDidFinish
+ RequestDidFailWithError
+ UpdatedTransactions
+ RestoreCompletedTransactionsFailedWithError
+ RestoreCompletedTransactionsFinished
+ UpdatedDownloads {}

pub trait DidReceiveResponse {

    fn did_receive_response(&mut self, 
            _0: *mut SKProductsRequest,
            _1: *mut SKProductsResponse);

}

pub trait RequestDidFinish {

    fn request_did_finish(&mut self, _0: *mut SKRequest);
}

pub trait RequestDidFailWithError {

    fn request_did_fail_with_error(&mut self, 
            _0: *mut SKRequest,
            _1: *mut NSError);
}

pub trait UpdatedTransactions {

    fn updated_transactions(&mut self, 
            _0: *mut SKPaymentQueue,
            _1: *mut NSArray<*mut SKPaymentTransaction>);
}

pub trait RestoreCompletedTransactionsFailedWithError {

    fn restore_completed_transactions_failed_with_error(&mut self, 
            _0: *mut SKPaymentQueue,
            _1: *mut NSError);
}

pub trait RestoreCompletedTransactionsFinished {

    fn restore_completed_transactions_finished(&mut self, _0: *mut SKPaymentQueue);
}

pub trait UpdatedDownloads {

    fn updated_downloads(&mut self, 
            _0: *mut SKPaymentQueue,
            _1: *mut NSArray<*mut SKDownload>);

}

crate::ix!();

type PaymentObserverClass = NSObject/*<SKProductsRequestDelegate,SKPaymentTransactionObserver>*/;

pub struct SKDelegateAndPaymentObserverClass {
    base: ObjCClass<PaymentObserverClass>,
}

impl Default for SKDelegateAndPaymentObserverClass {
    
    fn default() -> Self {
        todo!();
        /*


            : ObjCClass<NSObject<SKProductsRequestDelegate, SKPaymentTransactionObserver>> ("SKDelegateAndPaymentObserverBase_")
                addIvar<SKDelegateAndPaymentObserver*> ("self");

                addMethod (@selector (productsRequest:didReceiveResponse:),                       didReceiveResponse,                          "v@:@@");
                addMethod (@selector (requestDidFinish:),                                         requestDidFinish,                            "v@:@");
                addMethod (@selector (request:didFailWithError:),                                 requestDidFailWithError,                     "v@:@@");
                addMethod (@selector (paymentQueue:updatedTransactions:),                         updatedTransactions,                         "v@:@@");
                addMethod (@selector (paymentQueue:restoreCompletedTransactionsFailedWithError:), restoreCompletedTransactionsFailedWithError, "v@:@@");
                addMethod (@selector (paymentQueueRestoreCompletedTransactionsFinished:),         restoreCompletedTransactionsFinished,        "v@:@");
                addMethod (@selector (paymentQueue:updatedDownloads:),                            updatedDownloads,                            "v@:@@");

                registerClass();
        */
    }
}

impl SKDelegateAndPaymentObserverClass {

    pub fn get_this<'a>(self_: objc_id::Id<NSObject>) -> &'a mut SKDelegateAndPaymentObserver {
        
        todo!();
        /*
            return *getIvar<SKDelegateAndPaymentObserver*> (self, "self");
        */
    }
    
    pub fn set_this(
        self_: objc_id::Id<NSObject>,
        s:     *mut SKDelegateAndPaymentObserver)  {
        
        todo!();
        /*
            object_setInstanceVariable (self, "self", s);
        */
    }
    
    pub fn did_receive_response(
        self_:    objc_id::Id<NSObject>,
        _1:       Sel,
        request:  *mut SKProductsRequest,
        response: *mut SKProductsResponse)  {
        
        todo!();
        /*
            getThis (self).didReceiveResponse (request, response);
        */
    }
    
    pub fn request_did_finish(
        self_:   objc_id::Id<NSObject>,
        _1:      Sel,
        request: *mut SKRequest)  {
        
        todo!();
        /*
            getThis (self).requestDidFinish (request);
        */
    }
    
    pub fn request_did_fail_with_error(
        self_:   objc_id::Id<NSObject>,
        _1:      Sel,
        request: *mut SKRequest,
        err:     *mut NSError)  {
        
        todo!();
        /*
            getThis (self).requestDidFailWithError (request, err);
        */
    }
    
    pub fn updated_transactions(
        self_: objc_id::Id<NSObject>,
        _1:    Sel,
        queue: *mut SKPaymentQueue,
        trans: *mut NSArray<*mut SKPaymentTransaction>)  {
        
        todo!();
        /*
            getThis (self).updatedTransactions (queue, trans);
        */
    }
    
    pub fn restore_completed_transactions_failed_with_error(
        self_: objc_id::Id<NSObject>,
        _1:    Sel,
        q:     *mut SKPaymentQueue,
        err:   *mut NSError)  {
        
        todo!();
        /*
            getThis (self).restoreCompletedTransactionsFailedWithError (q, err);
        */
    }
    
    pub fn restore_completed_transactions_finished(
        self_: objc_id::Id<NSObject>,
        _1:    Sel,
        queue: *mut SKPaymentQueue)  {
        
        todo!();
        /*
            getThis (self).restoreCompletedTransactionsFinished (queue);
        */
    }
    
    pub fn updated_downloads(
        self_:     objc_id::Id<NSObject>,
        _1:        Sel,
        queue:     *mut SKPaymentQueue,
        downloads: *mut NSArray<*mut SKDownload>)  {
        
        todo!();
        /*
            getThis (self).updatedDownloads (queue, downloads);
        */
    }
}

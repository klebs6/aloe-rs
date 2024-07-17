crate::ix!();

pub struct InAppPurchasesPimpl<'a> {
    base:                             SKDelegateAndPaymentObserver,
    owner:                            &'a mut InAppPurchases<'a>,
    pending_product_info_requests:    Vec<Box<PendingProductInfoRequest>>,
    pending_receipt_refresh_requests: Vec<Box<PendingReceiptRefreshRequest>>,
    pending_downloads_transactions:   Vec<Box<PendingDownloadsTransaction>>,
    restored_purchases:               Vec<InAppPurchasesListenerPurchaseInfo>,
}

impl<'a> Drop for InAppPurchasesPimpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            [[SKPaymentQueue defaultQueue] removeTransactionObserver: delegate.get()];
        */
    }
}

impl<'a> InAppPurchasesPimpl<'a> {

    pub fn new(p: &mut InAppPurchases) -> Self {
    
        todo!();
        /*
        : owner(p),

            [[SKPaymentQueue defaultQueue] addTransactionObserver:    delegate.get()];
        */
    }
    
    pub fn is_in_app_purchases_supported(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn get_products_information(&mut self, product_identifiers: &StringArray)  {
        
        todo!();
        /*
            auto productsRequest = [[SKProductsRequest alloc] initWithProductIdentifiers: [NSSet setWithArray: createNSArrayFromStringArray (productIdentifiers)]];

            pendingProductInfoRequests.add (new PendingProductInfoRequest { PendingProductInfoRequest::Type::query,
                                                                            std::unique_ptr<SKProductsRequest, NSObjectDeleter> (productsRequest) });

            productsRequest.delegate = delegate.get();
            [productsRequest start];
        */
    }
    
    pub fn purchase_product(&mut self, 
        product_identifier: &String,
        _1:                 &String,
        _2:                 bool)  {
        
        todo!();
        /*
            if (! [SKPaymentQueue canMakePayments])
            {
                owner.listeners.call ([&] (Listener& l) { l.productPurchaseFinished ({}, false, NEEDS_TRANS ("Payments not allowed")); });
                return;
            }

            auto productIdentifiers = [NSArray arrayWithObject: aloeStringToNS (productIdentifier)];
            auto productsRequest    = [[SKProductsRequest alloc] initWithProductIdentifiers:[NSSet setWithArray:productIdentifiers]];

            pendingProductInfoRequests.add (new PendingProductInfoRequest { PendingProductInfoRequest::Type::purchase,
                                                                            std::unique_ptr<SKProductsRequest, NSObjectDeleter> (productsRequest) });

            productsRequest.delegate = delegate.get();
            [productsRequest start];
        */
    }
    
    pub fn restore_products_bought_list(&mut self, 
        include_download_info:       bool,
        subscriptions_shared_secret: &String)  {
        
        todo!();
        /*
            if (includeDownloadInfo)
            {
                [[SKPaymentQueue defaultQueue] restoreCompletedTransactions];
            }
            else
            {
                auto receiptRequest = [[SKReceiptRefreshRequest alloc] init];

                pendingReceiptRefreshRequests.add (new PendingReceiptRefreshRequest { subscriptionsSharedSecret,
                                                                                      std::unique_ptr<SKReceiptRefreshRequest, NSObjectDeleter> ([receiptRequest retain]) });
                receiptRequest.delegate = delegate.get();
                [receiptRequest start];
            }
        */
    }
    
    pub fn consume_purchase(&mut self, 
        _0: &String,
        _1: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn start_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            [[SKPaymentQueue defaultQueue] startDownloads: downloadsToSKDownloads (removeInvalidDownloads (downloads))];
        */
    }
    
    pub fn pause_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            [[SKPaymentQueue defaultQueue] pauseDownloads: downloadsToSKDownloads (removeInvalidDownloads (downloads))];
        */
    }
    
    pub fn resume_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            [[SKPaymentQueue defaultQueue] resumeDownloads: downloadsToSKDownloads (removeInvalidDownloads (downloads))];
        */
    }
    
    pub fn cancel_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            [[SKPaymentQueue defaultQueue] cancelDownloads: downloadsToSKDownloads (removeInvalidDownloads (downloads))];
        */
    }
    
    pub fn did_receive_response(&mut self, 
        request:  *mut SKProductsRequest,
        response: *mut SKProductsResponse)  {
        
        todo!();
        /*
            for (auto i = 0; i < pendingProductInfoRequests.size(); ++i)
            {
                auto& pendingRequest = *pendingProductInfoRequests[i];

                if (pendingRequest.request.get() == request)
                {
                    if      (pendingRequest.type == PendingProductInfoRequest::Type::query)    notifyProductsInfoReceived (response.products);
                    else if (pendingRequest.type == PendingProductInfoRequest::Type::purchase) startPurchase (response.products);
                    else break;

                    pendingProductInfoRequests.remove (i);
                    return;
                }
            }

            // Unknown request received!
            jassertfalse;
        */
    }
    
    pub fn request_did_finish(&mut self, request: *mut SKRequest)  {
        
        todo!();
        /*
            if (auto receiptRefreshRequest = getAs<SKReceiptRefreshRequest> (request))
            {
                for (auto i = 0; i < pendingReceiptRefreshRequests.size(); ++i)
                {
                    auto& pendingRequest = *pendingReceiptRefreshRequests[i];

                    if (pendingRequest.request.get() == receiptRefreshRequest)
                    {
                        processReceiptRefreshResponseWithSubscriptionsSharedSecret (pendingRequest.subscriptionsSharedSecret);
                        pendingReceiptRefreshRequests.remove (i);
                        return;
                    }
                }
            }
        */
    }
    
    pub fn request_did_fail_with_error(&mut self, 
        request: *mut SKRequest,
        error:   *mut NSError)  {
        
        todo!();
        /*
            if (auto receiptRefreshRequest = getAs<SKReceiptRefreshRequest> (request))
            {
                for (auto i = 0; i < pendingReceiptRefreshRequests.size(); ++i)
                {
                    auto& pendingRequest = *pendingReceiptRefreshRequests[i];

                    if (pendingRequest.request.get() == receiptRefreshRequest)
                    {
                        auto errorDetails = error != nil ? (", " + nsStringToAloe ([error localizedDescription])) : String();
                        owner.listeners.call ([&] (Listener& l) { l.purchasesListRestored ({}, false, NEEDS_TRANS ("Receipt fetch failed") + errorDetails); });
                        pendingReceiptRefreshRequests.remove (i);
                        return;
                    }
                }
            }
        */
    }
    
    pub fn updated_transactions(&mut self, 
        _0:           *mut SKPaymentQueue,
        transactions: *mut NSArray<*mut SKPaymentTransaction>)  {
        
        todo!();
        /*
            for (SKPaymentTransaction* transaction in transactions)
            {
                switch (transaction.transactionState)
                {
                    case SKPaymentTransactionStatePurchasing: break;
                    case SKPaymentTransactionStateDeferred:   break;
                    case SKPaymentTransactionStateFailed:     processTransactionFinish (transaction, false); break;
                    case SKPaymentTransactionStatePurchased:  processTransactionFinish (transaction, true);  break;
                    case SKPaymentTransactionStateRestored:   processTransactionFinish (transaction, true);  break;
                    default:                                  jassertfalse; break;  // Unexpected transaction state
                }
            }
        */
    }
    
    pub fn restore_completed_transactions_failed_with_error(&mut self, 
        _0:    *mut SKPaymentQueue,
        error: *mut NSError)  {
        
        todo!();
        /*
            owner.listeners.call ([&] (Listener& l) { l.purchasesListRestored ({}, false, nsStringToAloe (error.localizedDescription)); });
        */
    }
    
    pub fn restore_completed_transactions_finished(&mut self, _0: *mut SKPaymentQueue)  {
        
        todo!();
        /*
            owner.listeners.call ([this] (Listener& l) { l.purchasesListRestored (restoredPurchases, true, NEEDS_TRANS ("Success")); });
            restoredPurchases.clear();
        */
    }
    
    pub fn updated_downloads(&mut self, 
        _0:        *mut SKPaymentQueue,
        downloads: *mut NSArray<*mut SKDownload>)  {
        
        todo!();
        /*
            for (SKDownload* download in downloads)
            {
                if (auto* pendingDownload = getPendingDownloadFor (download))
                {
                  #if ALOE_IOS
                    switch (download.downloadState)
                  #else
                    switch (download.state)
                  #endif
                    {
                        case SKDownloadStateWaiting: break;
                        case SKDownloadStatePaused:  owner.listeners.call ([&] (Listener& l) { l.productDownloadPaused (*pendingDownload); }); break;
                        case SKDownloadStateActive:  owner.listeners.call ([&] (Listener& l) { l.productDownloadProgressUpdate (*pendingDownload,
                                                                                                                                download.progress,
                                                                                                                                RelativeTime (download.timeRemaining)); }); break;
                        case SKDownloadStateFinished:
                        case SKDownloadStateFailed:
                        case SKDownloadStateCancelled: processDownloadFinish (pendingDownload, download); break;

                        default:  jassertfalse; break;  // Unexpected download state
                    }
                }
            }
        */
    }
    
    pub fn notify_products_info_received(&mut self, products: *mut NSArray<*mut SKProduct>)  {
        
        todo!();
        /*
            Vec<Product> productsToReturn;

            for (SKProduct* skProduct in products)
                productsToReturn.add (SKProductToIAPProduct (skProduct));

            owner.listeners.call ([&] (Listener& l) { l.productsInfoReturned (productsToReturn); });
        */
    }
    
    pub fn start_purchase(&mut self, products: *mut NSArray<*mut SKProduct>)  {
        
        todo!();
        /*
            if ([products count] > 0)
            {
                // Only one product can be bought at once!
                jassert ([products count] == 1);

                auto* product = products[0];
                auto payment = [SKPayment paymentWithProduct: product];
                [[SKPaymentQueue defaultQueue] addPayment: payment];
            }
            else
            {
                owner.listeners.call ([] (Listener& l) { l.productPurchaseFinished ({}, false, NEEDS_TRANS ("Your app is not setup for payments")); });
            }
        */
    }
    
    pub fn remove_invalid_downloads(&mut self, downloads_to_use: &[*mut dyn InAppPurchasesDownload]) -> Vec<*mut dyn InAppPurchasesDownload> {
        
        todo!();
        /*
            Vec<InAppPurchasesDownload*> downloads (downloadsToUse);

            for (int i = downloads.size(); --i >= 0;)
            {
                auto hasPendingDownload = hasDownloadInPendingDownloadsTransaction (*downloads[i]);

                // Invalid download passed, it does not exist in pending downloads list
                jassert (hasPendingDownload);

                if (! hasPendingDownload)
                    downloads.remove (i);
            }

            return downloads;
        */
    }
    
    pub fn has_download_in_pending_downloads_transaction(&mut self, download: &dyn InAppPurchasesDownload) -> bool {
        
        todo!();
        /*
            for (auto* pdt : pendingDownloadsTransactions)
                for (auto* pendingDownload : pdt->downloads)
                    if (pendingDownload == &download)
                        return true;

            return false;
        */
    }
    
    pub fn process_transaction_finish(&mut self, 
        transaction: *mut SKPaymentTransaction,
        success:     bool)  {
        
        todo!();
        /*
            auto orderId      = nsStringToAloe (transaction.transactionIdentifier);
            auto packageName  = nsStringToAloe ([[NSBundle mainBundle] bundleIdentifier]);
            auto productId    = nsStringToAloe (transaction.payment.productIdentifier);
            auto purchaseTime = Time (1000 * (int64) transaction.transactionDate.timeIntervalSince1970)
                                  .toString (true, true, true, true);

            Purchase purchase { orderId, productId, packageName, purchaseTime, {} };

            Vec<InAppPurchasesDownload*> downloads;

            // If transaction failed or there are no downloads, finish the transaction immediately, otherwise
            // finish the transaction only after all downloads are finished.
            if (transaction.transactionState == SKPaymentTransactionStateFailed
                 || transaction.downloads == nil
                 || [transaction.downloads count] == 0)
            {
                [[SKPaymentQueue defaultQueue]  finishTransaction: transaction];
            }
            else
            {
                // On application startup or when the app is resumed we may receive multiple
                // "purchased" callbacks with the same underlying transaction. Sadly, only
                // the last set of downloads will be valid.
                auto* pdt = getPendingDownloadsTransactionForSKTransaction (transaction);

                if (pdt == nullptr)
                {
                    pdt = pendingDownloadsTransactions.add (new PendingDownloadsTransaction (transaction));
                }
                else
                {
                    pdt->downloads.clear();
                    pdt->addDownloadsFromSKTransaction (transaction);
                }

                for (auto* download : pdt->downloads)
                    downloads.add (download);
            }

            if (transaction.transactionState == SKPaymentTransactionStateRestored)
                restoredPurchases.add ({ purchase, downloads });
            else
                owner.listeners.call ([&] (Listener& l) { l.productPurchaseFinished ({ purchase, downloads }, success,
                                                                                     SKPaymentTransactionStateToString (transaction.transactionState)); });
        */
    }
    
    pub fn get_pending_downloads_transaction_for_sk_transaction(&mut self, transaction: *mut SKPaymentTransaction) -> *mut PendingDownloadsTransaction {
        
        todo!();
        /*
            for (auto* pdt : pendingDownloadsTransactions)
                if (pdt->transaction == transaction)
                    return pdt;

            return nullptr;
        */
    }
    
    pub fn get_pending_downloads_transaction_sk_download_for(&mut self, download: *mut SKDownload) -> *mut PendingDownloadsTransaction {
        
        todo!();
        /*
            for (auto* pdt : pendingDownloadsTransactions)
                for (auto* pendingDownload : pdt->downloads)
                    if (pendingDownload->download == download)
                        return pdt;

            jassertfalse;
            return nullptr;
        */
    }
    
    pub fn get_pending_download_for(&mut self, download: *mut SKDownload) -> *mut dyn InAppPurchasesDownload {
        
        todo!();
        /*
            if (auto* pdt = getPendingDownloadsTransactionSKDownloadFor (download))
                for (auto* pendingDownload : pdt->downloads)
                    if (pendingDownload->download == download)
                        return pendingDownload;

            jassertfalse;
            return nullptr;
        */
    }
    
    pub fn process_download_finish(&mut self, 
        pending_download: *mut dyn InAppPurchasesDownload,
        download:         *mut SKDownload)  {
        
        todo!();
        /*
            if (auto* pdt = getPendingDownloadsTransactionSKDownloadFor (download))
            {
              #if ALOE_IOS
                SKDownloadState state = download.downloadState;
              #else
                SKDownloadState state = download.state;
              #endif

                auto contentURL = state == SKDownloadStateFinished
                                    ? Url (nsStringToAloe (download.contentURL.absoluteString))
                                    : Url();

                owner.listeners.call ([&] (Listener& l) { l.productDownloadFinished (*pendingDownload, contentURL); });

                if (pdt->canBeMarkedAsFinished())
                {
                    // All downloads finished, mark transaction as finished too.
                    [[SKPaymentQueue defaultQueue]  finishTransaction: pdt->transaction];

                    pendingDownloadsTransactions.removeObject (pdt);
                }
            }
        */
    }
    
    pub fn process_receipt_refresh_response_with_subscriptions_shared_secret(&mut self, secret: &String)  {
        
        todo!();
        /*
            auto receiptURL = [[NSBundle mainBundle] appStoreReceiptURL];

            if (auto receiptData = [NSData dataWithContentsOfURL: receiptURL])
                fetchReceiptDetailsFromAppStore (receiptData, secret);
            else
                owner.listeners.call ([&] (Listener& l) { l.purchasesListRestored ({}, false, NEEDS_TRANS ("Receipt fetch failed")); });
        */
    }
    
    pub fn fetch_receipt_details_from_app_store(&mut self, 
        receipt_data: *mut NSData,
        secret:       &String)  {
        
        todo!();
        /*
            auto requestContents = [NSMutableDictionary dictionaryWithCapacity: (NSUInteger) (secret.isNotEmpty() ? 2 : 1)];
            [requestContents setObject: [receiptData base64EncodedStringWithOptions:0] forKey: nsStringLiteral ("receipt-data")];

            if (secret.isNotEmpty())
                [requestContents setObject: aloeStringToNS (secret) forKey: nsStringLiteral ("password")];

            NSError* error;
            auto requestData = [NSJSONSerialization dataWithJSONObject: requestContents
                                                               options: 0
                                                                 error: &error];
            if (requestData == nil)
            {
                sendReceiptFetchFail();
                return;
            }

           #if ALOE_IN_APP_PURCHASES_USE_SANDBOX_ENVIRONMENT
            auto storeURL = "https://sandbox.itunes.apple.com/verifyReceipt";
           #else
            auto storeURL = "https://buy.itunes.apple.com/verifyReceipt";
           #endif

            // TODO: use aloe Url here
            auto* urlPtr = [NSURL URLWithString: nsStringLiteral (storeURL)];
            auto storeRequest = [NSMutableURLRequest requestWithURL: urlPtr];
            [storeRequest setHTTPMethod: nsStringLiteral ("POST")];
            [storeRequest setHTTPBody: requestData];

            auto task = [[NSURLSession sharedSession] dataTaskWithRequest: storeRequest
                                                        completionHandler:
                                                           ^(NSData* data, NSURLResponse*, NSError* connectionError)
                                                           {
                                                               if (connectionError != nil)
                                                               {
                                                                   sendReceiptFetchFail();
                                                               }
                                                               else
                                                               {
                                                                   NSError* err;

                                                                   if (NSDictionary* receiptDetails = [NSJSONSerialization JSONObjectWithData: data options: 0 error: &err])
                                                                       processReceiptDetails (receiptDetails);
                                                                   else
                                                                       sendReceiptFetchFail();
                                                               }
                                                           }];

            [task resume];
        */
    }
    
    pub fn process_receipt_details<K,V>(&mut self, receipt_details: *mut NSDictionary<K,V>)  {
        
        todo!();
        /*
            if (auto receipt = getAs<NSDictionary> (receiptDetails[nsStringLiteral ("receipt")]))
            {
                if (auto bundleId = getAs<NSString> (receipt[nsStringLiteral ("bundle_id")]))
                {
                    if (auto inAppPurchases = getAs<NSArray> (receipt[nsStringLiteral ("in_app")]))
                    {
                        Vec<Listener::PurchaseInfo> purchases;

                        for (id inAppPurchaseData in inAppPurchases)
                        {
                            if (auto* purchaseData = getAs<NSDictionary> (inAppPurchaseData))
                            {
                                // Ignore products that were cancelled.
                                if (purchaseData[nsStringLiteral ("cancellation_date")] != nil)
                                    continue;

                                if (auto transactionId = getAs<NSString> (purchaseData[nsStringLiteral ("original_transaction_id")]))
                                {
                                    if (auto productId = getAs<NSString> (purchaseData[nsStringLiteral ("product_id")]))
                                    {
                                        auto purchaseTime = getPurchaseDateMs (purchaseData[nsStringLiteral ("purchase_date_ms")]);

                                        if (purchaseTime > 0)
                                        {
                                            purchases.add ({ { nsStringToAloe (transactionId),
                                                               nsStringToAloe (productId),
                                                               nsStringToAloe (bundleId),
                                                               Time (purchaseTime).toString (true, true, true, true),
                                                               {} }, {} });
                                        }
                                        else
                                        {
                                            return sendReceiptFetchFailAsync();
                                        }
                                    }
                                }
                            }
                            else
                            {
                                return sendReceiptFetchFailAsync();
                            }
                        }

                        MessageManager::callAsync ([this, purchases] { owner.listeners.call ([&] (Listener& l) { l.purchasesListRestored (purchases, true, NEEDS_TRANS ("Success")); }); });
                        return;
                    }
                }
            }

            sendReceiptFetchFailAsync();
        */
    }
    
    pub fn send_receipt_fetch_fail(&mut self)  {
        
        todo!();
        /*
            owner.listeners.call ([] (Listener& l) { l.purchasesListRestored ({}, false, NEEDS_TRANS ("Receipt fetch failed")); });
        */
    }
    
    pub fn send_receipt_fetch_fail_async(&mut self)  {
        
        todo!();
        /*
            MessageManager::callAsync ([this] { sendReceiptFetchFail(); });
        */
    }
    
    pub fn get_purchase_date_ms(date: objc_id::Id<NSObject>) -> i64 {
        
        todo!();
        /*
            if (auto dateAsNumber = getAs<NSNumber> (date))
            {
                return [dateAsNumber longLongValue];
            }
            else if (auto dateAsString = getAs<NSString> (date))
            {
                auto formatter = [[NSNumberFormatter alloc] init];
                [formatter setNumberStyle: NSNumberFormatterDecimalStyle];
                dateAsNumber = [formatter numberFromString: dateAsString];
                [formatter release];
                return [dateAsNumber longLongValue];
            }

            return -1;
        */
    }
    
    pub fn sk_product_to_iap_product(sk_product: *mut SKProduct) -> InAppPurchasesProduct {
        
        todo!();
        /*
            NSNumberFormatter* numberFormatter = [[NSNumberFormatter alloc] init];
            [numberFormatter setFormatterBehavior: NSNumberFormatterBehavior10_4];
            [numberFormatter setNumberStyle: NSNumberFormatterCurrencyStyle];
            [numberFormatter setLocale: skProduct.priceLocale];

            auto identifier   = nsStringToAloe (skProduct.productIdentifier);
            auto title        = nsStringToAloe (skProduct.localizedTitle);
            auto description  = nsStringToAloe (skProduct.localizedDescription);
            auto priceLocale  = nsStringToAloe ([skProduct.priceLocale objectForKey: NSLocaleLanguageCode]);
            auto price        = nsStringToAloe ([numberFormatter stringFromNumber: skProduct.price]);

            [numberFormatter release];

            return { identifier, title, description, price, priceLocale };
        */
    }
    
    pub fn sk_payment_transaction_state_to_string(state: SKPaymentTransactionState) -> String {
        
        todo!();
        /*
            switch (state)
            {
                case SKPaymentTransactionStatePurchasing: return NEEDS_TRANS ("Purchasing");
                case SKPaymentTransactionStatePurchased:  return NEEDS_TRANS ("Success");
                case SKPaymentTransactionStateFailed:     return NEEDS_TRANS ("Failure");
                case SKPaymentTransactionStateRestored:   return NEEDS_TRANS ("Restored");
                case SKPaymentTransactionStateDeferred:   return NEEDS_TRANS ("Deferred");
                default:                                  jassertfalse; return NEEDS_TRANS ("Unknown status");
            }
        */
    }
    
    pub fn sk_download_state_to_download_status(state: SKDownloadState) -> InAppPurchasesDownloadStatus {
        
        todo!();
        /*
            switch (state)
            {
                case SKDownloadStateWaiting:    return InAppPurchasesDownload::Status::waiting;
                case SKDownloadStateActive:     return InAppPurchasesDownload::Status::active;
                case SKDownloadStatePaused:     return InAppPurchasesDownload::Status::paused;
                case SKDownloadStateFinished:   return InAppPurchasesDownload::Status::finished;
                case SKDownloadStateFailed:     return InAppPurchasesDownload::Status::failed;
                case SKDownloadStateCancelled:  return InAppPurchasesDownload::Status::cancelled;
                default:                        jassertfalse; return InAppPurchasesDownload::Status::waiting;
            }
        */
    }
    
    pub fn downloads_to_sk_downloads(downloads: &[*mut dyn InAppPurchasesDownload]) -> *mut NSArray<*mut SKDownload> {
        
        todo!();
        /*
            NSMutableArray<SKDownload*>* skDownloads = [NSMutableArray arrayWithCapacity: (NSUInteger) downloads.size()];

            for (const auto& d : downloads)
                if (auto impl = dynamic_cast<InAppPurchasesPimplDownloadImpl*>(d))
                    [skDownloads addObject: impl->download];

            return skDownloads;
        */
    }
    
    
    pub fn get_as<ObjCType>(o: objc_id::Id<ObjCType>) -> *mut ObjCType {
    
        todo!();
        /*
            if (o == nil || ! [o isKindOfClass: [ObjCType class]])
                return nil;

            return (ObjCType*) o;
        */
    }
}

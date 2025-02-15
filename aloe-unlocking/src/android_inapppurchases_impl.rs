crate::ix!();

#[cfg(target_os="android")]
#[no_copy]
#[leak_detector]
pub struct InAppPurchasesImpl<'a> {
    owner:                               &'a mut InAppPurchases,
    billing_client:                      GlobalRef,
    sku_details_query_callback_queue:    SegQueue<fn(_0: LocalRef<jobject>) -> ()>,
    purchases_list_query_callback_queue: SegQueue<fn(_0: LocalRef<jobject>) -> ()>,
}

#[cfg(target_os="android")]
pub mod impl_ {

    use super::*;

    lazy_static!{
        /*
        InAppPurchases::InAppPurchasesImpl::AloeBillingClient_Class InAppPurchases::InAppPurchasesImpl::AloeBillingClient;
        */
    }
}

#[cfg(target_os="android")]
impl<'a> Drop for InAppPurchasesImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            getEnv()->CallVoidMethod (billingClient, AloeBillingClient.endConnection);
        */
    }
}

#[cfg(target_os="android")]
impl<'a> InAppPurchasesImpl<'a> {

    pub fn new(parent: &mut InAppPurchases) -> Self {
    
        todo!();
        /*


            : owner (parent),
              billingClient (LocalRef<jobject> (getEnv()->NewObject (AloeBillingClient,
                                                                     AloeBillingClient.constructor,
                                                                     getAppContext().get(),
                                                                     (jlong) this)))
        */
    }
    
    pub fn is_in_app_purchases_supported(&self) -> bool {
        
        todo!();
        /*
            return isReady() && getEnv()->CallBooleanMethod (billingClient, AloeBillingClient.isBillingSupported);
        */
    }
    
    pub fn get_products_information(&mut self, product_identifiers: &StringArray)  {
        
        todo!();
        /*
            skuDetailsQueryCallbackQueue.emplace ([this] (LocalRef<jobject> skuDetailsList)
            {
                if (skuDetailsList != nullptr)
                {
                    auto* env = getEnv();
                    Vec<InAppPurchases::Product> products;

                    for (int i = 0; i < env->CallIntMethod (skuDetailsList, JavaList.size); ++i)
                        products.add (buildProduct (LocalRef<jobject> (env->CallObjectMethod (skuDetailsList, JavaList.get, i))));

                    owner.listeners.call ([&] (Listener& l) { l.productsInfoReturned (products); });
                }
            });

            querySkuDetailsAsync (convertToLowerCase (productIdentifiers));
        */
    }
    
    pub fn purchase_product(&mut self, 
        product_identifier:             &String,
        subscription_identifier:        &String,
        credit_for_unused_subscription: bool)  {
        
        todo!();
        /*
            skuDetailsQueryCallbackQueue.emplace ([=] (LocalRef<jobject> skuDetailsList)
            {
                if (skuDetailsList != nullptr)
                {
                    auto* env = getEnv();

                    if (env->CallIntMethod (skuDetailsList, JavaList.size) > 0)
                    {
                        LocalRef<jobject> skuDetails (env->CallObjectMethod (skuDetailsList, JavaList.get, 0));

                        if (subscriptionIdentifier.isNotEmpty())
                            changeExistingSubscription (skuDetails, subscriptionIdentifier, creditForUnusedSubscription);
                        else
                            purchaseProductWithSkuDetails (skuDetails);
                    }
                }
            });

            querySkuDetailsAsync (convertToLowerCase ({ productIdentifier }));
        */
    }
    
    pub fn restore_products_bought_list(&mut self, 
        _0: bool,
        _1: &String)  {
        
        todo!();
        /*
            purchasesListQueryCallbackQueue.emplace ([this] (LocalRef<jobject> purchasesList)
            {
                if (purchasesList != nullptr)
                {
                    auto* env = getEnv();
                    Vec<InAppPurchases::Listener::PurchaseInfo> purchases;

                    for (int i = 0; i < env->CallIntMethod (purchasesList, JavaArrayList.size); ++i)
                    {
                        LocalRef<jobject> purchase (env->CallObjectMethod (purchasesList, JavaArrayList.get, i));
                        purchases.add ({ buildPurchase (purchase), {} });
                    }

                    owner.listeners.call ([&] (Listener& l) { l.purchasesListRestored (purchases, true, NEEDS_TRANS ("Success")); });
                }
                else
                {
                    owner.listeners.call ([&] (Listener& l) { l.purchasesListRestored ({}, false, NEEDS_TRANS ("Failure")); });
                }
            });

            getProductsBoughtAsync();
        */
    }
    
    pub fn consume_purchase(&mut self, 
        product_identifier: &String,
        purchase_token:     &String)  {
        
        todo!();
        /*
            if (purchaseToken.isEmpty())
            {
                skuDetailsQueryCallbackQueue.emplace ([=] (LocalRef<jobject> skuDetailsList)
                {
                    if (skuDetailsList != nullptr)
                    {
                        auto* env = getEnv();

                        if (env->CallIntMethod (skuDetailsList, JavaList.size) > 0)
                        {
                            LocalRef<jobject> sku (env->CallObjectMethod (skuDetailsList, JavaList.get, 0));

                            auto token = aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (sku, AndroidPurchase.getSku)));

                            if (token.isNotEmpty())
                            {
                                consumePurchaseWithToken (productIdentifier, token);
                                return;
                            }
                        }
                    }

                    notifyListenersAboutConsume (productIdentifier, false, NEEDS_TRANS ("Item unavailable"));
                });

                querySkuDetailsAsync (convertToLowerCase ({ productIdentifier }));
            }

            consumePurchaseWithToken (productIdentifier, purchaseToken);
        */
    }
    
    pub fn start_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            // Not available on this platform.
            ignoreUnused (downloads);
            jassertfalse;
        */
    }
    
    pub fn pause_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            // Not available on this platform.
            ignoreUnused (downloads);
            jassertfalse;
        */
    }
    
    pub fn resume_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            // Not available on this platform.
            ignoreUnused (downloads);
            jassertfalse;
        */
    }
    
    pub fn cancel_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            // Not available on this platform.
            ignoreUnused (downloads);
            jassertfalse;
        */
    }

    #[JNICALL]
    pub fn sku_details_query_callback(
        _0:               *mut JNIEnv,
        _1:               jobject,
        host:             i64,
        sku_details_list: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<InAppPurchasesImpl*> (host))
                myself->updateSkuDetails (skuDetailsList);
        */
    }
    
    pub fn purchases_list_query_callback(
        _0:             *mut JNIEnv,
        _1:             jobject,
        host:           i64,
        purchases_list: jobject)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<InAppPurchasesImpl*> (host))
                myself->updatePurchasesList (purchasesList);
        */
    }
    
    pub fn purchase_completed_callback(
        _0:            *mut JNIEnv,
        _1:            jobject,
        host:          i64,
        purchase:      jobject,
        response_code: i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<InAppPurchasesImpl*> (host))
                myself->purchaseCompleted (purchase, responseCode);
        */
    }
    
    pub fn purchase_consumed_callback(
        _0:                 *mut JNIEnv,
        _1:                 jobject,
        host:               i64,
        product_identifier: jstring,
        response_code:      i32)  {
        
        todo!();
        /*
            if (auto* myself = reinterpret_cast<InAppPurchasesImpl*> (host))
                myself->purchaseConsumed (productIdentifier, responseCode);
        */
    }
    
    pub fn is_ready(&self) -> bool {
        
        todo!();
        /*
            return getEnv()->CallBooleanMethod (billingClient, AloeBillingClient.isReady);
        */
    }
    
    pub fn check_is_ready(&self) -> bool {
        
        todo!();
        /*
            for (int i = 0; i < 10; ++i)
            {
                if (isReady())
                    return true;

                Thread::sleep (500);
            }

            return false;
        */
    }
    
    pub fn convert_to_lower_case(strings_to_convert: &StringArray) -> StringArray {
        
        todo!();
        /*
            StringArray lowerCase;

            for (auto& s : stringsToConvert)
                lowerCase.add (s.toLowerCase());

            return lowerCase;
        */
    }
    
    pub fn query_sku_details_async(&mut self, product_identifiers: &StringArray)  {
        
        todo!();
        /*
            Thread::launch ([=]
            {
                if (! checkIsReady())
                    return;

                MessageManager::callAsync ([=]
                {
                    getEnv()->CallVoidMethod (billingClient,
                                              AloeBillingClient.querySkuDetails,
                                              aloeStringArrayToJava (productIdentifiers).get());
                });
            });
        */
    }
    
    pub fn get_products_bought_async(&mut self)  {
        
        todo!();
        /*
            Thread::launch ([=]
            {
                if (! checkIsReady())
                    return;

                MessageManager::callAsync ([=]
                {
                    getEnv()->CallVoidMethod (billingClient,
                                              AloeBillingClient.queryPurchases);
                });
            });
        */
    }
    
    pub fn notify_listeners_about_purchase(&mut self, 
        purchase:           &InAppPurchases::Purchase,
        success:            bool,
        status_description: &String)  {
        
        todo!();
        /*
            owner.listeners.call ([&] (Listener& l) { l.productPurchaseFinished ({ purchase, {} }, success, statusDescription); });
        */
    }
    
    pub fn notify_listeners_about_consume(&mut self, 
        product_identifier: &String,
        success:            bool,
        status_description: &String)  {
        
        todo!();
        /*
            owner.listeners.call ([&] (Listener& l) { l.productConsumed (productIdentifier, success, statusDescription); });
        */
    }
    
    pub fn create_billing_flow_params_builder(&mut self, sku_details: LocalRef<jobject>) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

            auto builder = LocalRef<jobject> (env->CallStaticObjectMethod (BillingFlowParams, BillingFlowParams.newBuilder));

            return LocalRef<jobject> (env->CallObjectMethod (builder.get(),
                                      BillingFlowParamsBuilder.setSkuDetails,
                                      skuDetails.get()));
        */
    }
    
    pub fn launch_billing_flow_with_parameters(&mut self, params: LocalRef<jobject>)  {
        
        todo!();
        /*
            LocalRef<jobject> activity (getCurrentActivity());

            if (activity == nullptr)
                activity = getMainActivity();

            getEnv()->CallVoidMethod (billingClient,
                                      AloeBillingClient.launchBillingFlow,
                                      activity.get(),
                                      params.get());
        */
    }
    
    pub fn change_existing_subscription(&mut self, 
        sku_details:                    LocalRef<jobject>,
        subscription_identifier:        &String,
        credit_for_unused_subscription: bool)  {
        
        todo!();
        /*
            if (! isReady())
            {
                notifyListenersAboutPurchase ({}, false, NEEDS_TRANS ("In-App purchases unavailable"));
                return;
            }

            purchasesListQueryCallbackQueue.emplace ([=] (LocalRef<jobject> purchasesList)
            {
                if (purchasesList != nullptr)
                {
                    auto* env = getEnv();

                    for (int i = 0; i < env->CallIntMethod (purchasesList, JavaArrayList.size); ++i)
                    {
                        auto purchase = buildPurchase (LocalRef<jobject> (env->CallObjectMethod (purchasesList.get(), JavaArrayList.get, i)));

                        if (purchase.productId == subscriptionIdentifier)
                        {
                            auto builder = createBillingFlowParamsBuilder (skuDetails);

                            builder = LocalRef<jobject> (env->CallObjectMethod (builder.get(),
                                                                                BillingFlowParamsBuilder.setOldSku,
                                                                                javaString (subscriptionIdentifier).get(),
                                                                                javaString (purchase.purchaseToken).get()));

                            if (! creditForUnusedSubscription)
                                builder = LocalRef<jobject> (env->CallObjectMethod (builder.get(),
                                                                                    BillingFlowParamsBuilder.setReplaceSkusProrationMode,
                                                                                    3 /*IMMEDIATE_WITHOUT_PRORATION*/));

                            launchBillingFlowWithParameters (LocalRef<jobject> (env->CallObjectMethod (builder.get(),
                                                                                                       BillingFlowParamsBuilder.build)));
                        }
                    }
                }

                notifyListenersAboutPurchase ({}, false, NEEDS_TRANS ("Unable to get subscription details"));
            });

            getProductsBoughtAsync();
        */
    }
    
    pub fn purchase_product_with_sku_details(&mut self, sku_details: LocalRef<jobject>)  {
        
        todo!();
        /*
            if (! isReady())
            {
                notifyListenersAboutPurchase ({}, false, NEEDS_TRANS ("In-App purchases unavailable"));
                return;
            }

            launchBillingFlowWithParameters (LocalRef<jobject> (getEnv()->CallObjectMethod (createBillingFlowParamsBuilder (skuDetails).get(),
                                                                                            BillingFlowParamsBuilder.build)));
        */
    }
    
    pub fn consume_purchase_with_token(&mut self, 
        product_identifier: &String,
        purchase_token:     &String)  {
        
        todo!();
        /*
            if (! isReady())
            {
                notifyListenersAboutConsume (productIdentifier, false, NEEDS_TRANS ("In-App purchases unavailable"));
                return;
            }

            getEnv()->CallObjectMethod (billingClient,
                                        AloeBillingClient.consumePurchase,
                                        LocalRef<jstring> (javaString (productIdentifier)).get(),
                                        LocalRef<jstring> (javaString (purchaseToken)).get());
        */
    }
    
    pub fn build_purchase(purchase: LocalRef<jobject>) -> InAppPurchases::Purchase {
        
        todo!();
        /*
            if (purchase == nullptr)
                return {};

            auto* env = getEnv();

            return { aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (purchase, AndroidPurchase.getOrderId))),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (purchase, AndroidPurchase.getSku))),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (purchase, AndroidPurchase.getPackageName))),
                     Time (env->CallLongMethod (purchase, AndroidPurchase.getPurchaseTime)).toString (true, true, true, true),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (purchase, AndroidPurchase.getPurchaseToken))) };
        */
    }
    
    pub fn build_product(product_sku_details: LocalRef<jobject>) -> InAppPurchases::Product {
        
        todo!();
        /*
            if (productSkuDetails == nullptr)
                return {};

            auto* env = getEnv();

            return { aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (productSkuDetails, SkuDetails.getSku))),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (productSkuDetails, SkuDetails.getTitle))),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (productSkuDetails, SkuDetails.getDescription))),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (productSkuDetails, SkuDetails.getPrice))),
                     aloeString (LocalRef<jstring> ((jstring) env->CallObjectMethod (productSkuDetails, SkuDetails.getPriceCurrencyCode))) };
        */
    }
    
    pub fn get_status_description_from_response_code(response_code: i32) -> String {
        
        todo!();
        /*
            switch (responseCode)
            {
                case 0:   return NEEDS_TRANS ("Success");
                case 1:   return NEEDS_TRANS ("Cancelled by user");
                case 2:   return NEEDS_TRANS ("Service unavailable");
                case 3:   return NEEDS_TRANS ("Billing unavailable");
                case 4:   return NEEDS_TRANS ("Item unavailable");
                case 5:   return NEEDS_TRANS ("Internal error");
                case 6:   return NEEDS_TRANS ("Generic error");
                case 7:   return NEEDS_TRANS ("Item already owned");
                case 8:   return NEEDS_TRANS ("Item not owned");
                default:  return NEEDS_TRANS ("Unknown status");
            }
        */
    }
    
    pub fn was_successful(response_code: i32) -> bool {
        
        todo!();
        /*
            return responseCode == 0;
        */
    }
    
    pub fn purchase_completed(&mut self, 
        purchase:      jobject,
        response_code: i32)  {
        
        todo!();
        /*
            notifyListenersAboutPurchase (buildPurchase (LocalRef<jobject> (purchase)),
                                          wasSuccessful (responseCode),
                                          getStatusDescriptionFromResponseCode (responseCode));
        */
    }
    
    pub fn purchase_consumed(&mut self, 
        product_identifier: jstring,
        response_code:      i32)  {
        
        todo!();
        /*
            notifyListenersAboutConsume (aloeString (LocalRef<jstring> (productIdentifier)),
                                         wasSuccessful (responseCode),
                                         getStatusDescriptionFromResponseCode (responseCode));
        */
    }
    
    pub fn update_sku_details(&mut self, sku_details_list: jobject)  {
        
        todo!();
        /*
            jassert (! skuDetailsQueryCallbackQueue.empty());
            skuDetailsQueryCallbackQueue.front() (LocalRef<jobject> (skuDetailsList));
            skuDetailsQueryCallbackQueue.pop();
        */
    }
    
    pub fn update_purchases_list(&mut self, purchases_list: jobject)  {
        
        todo!();
        /*
            jassert (! purchasesListQueryCallbackQueue.empty());
            purchasesListQueryCallbackQueue.front() (LocalRef<jobject> (purchasesList));
            purchasesListQueryCallbackQueue.pop();
        */
    }
}

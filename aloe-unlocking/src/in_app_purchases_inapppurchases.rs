crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/in_app_purchases/aloe_InAppPurchases.h]

/**
  | Provides in-app purchase functionality.
  | 
  | Your app should create a single instance
  | of this class, and on iOS it should be
  | created as soon as your app starts. This
  | is because on application startup any
  | previously pending transactions will
  | be resumed.
  | 
  | Once an InAppPurchases object is created,
  | call addListener() to attach listeners.
  | 
  | @tags{InAppPurchasesProductUnlocking}
  |
  */
pub struct InAppPurchases<'a> {
    base: DeletedAtShutdown,

    listeners: ListenerList<Box<dyn InAppPurchasesListenerInterface>>,

    #[cfg(any(any(target_os="android",target_os="ios"),target_os="macos"))]
    pimpl: Box<InAppPurchasesPimpl<'a>>,
}

aloe_declare_singleton!{
    InAppPurchases, false
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/in_app_purchases/aloe_InAppPurchases.cpp]
aloe_implement_singleton!{
    InAppPurchases
}

impl<'a> Drop for InAppPurchases<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

impl<'a> Default for InAppPurchases<'a> {
    
    fn default() -> Self {
    
        todo!();
        /*


            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        : pimpl (new Pimpl (*this))
       #endif
        */
    }
}

impl<'a> InAppPurchases<'a> {
    
    /**
      | Checks whether in-app purchases is
      | supported on current platform. On iOS
      | this always returns true.
      |
      */
    pub fn is_in_app_purchases_supported(&self) -> bool {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        return pimpl->isInAppPurchasesSupported();
       #else
        return false;
       #endif
        */
    }
    
    /**
      | Asynchronously requests information
      | for products with given ids. Upon completion,
      | for each enquired product there is going
      | to be a corresponding InAppPurchasesProduct object.
      | 
      | If there is no information available
      | for the given product identifier, it
      | will be ignored.
      |
      */
    pub fn get_products_information(&mut self, product_identifiers: &StringArray)  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->getInAppPurchasesProductsInformation (productIdentifiers);
       #else
        Vec<InAppPurchasesProduct> products;
        for (auto productId : productIdentifiers)
            products.add (InAppPurchasesProduct { productId, {}, {}, {}, {}  });

        listeners.call ([&] (Listener& l) { l.productsInfoReturned (products); });
       #endif
        */
    }
    
    /**
      | Asynchronously requests to buy a product
      | with given id.
      | 
      | -----------
      | @param productIdentifier
      | 
      | The product identifier.
      | ----------
      | @param upgradeOrDowngradeFromSubscriptionWithInAppPurchasesProductIdentifier
      | 
      | (Android only) specifies the subscription
      | that will be replaced by the one being
      | purchased now. Used only when buying
      | a subscription that is an upgrade or
      | downgrade from another.
      | ----------
      | @param creditForUnusedSubscription
      | 
      | (Android only) controls whether a user
      | should be credited for any unused subscription
      | time on the product that is being upgraded
      | or downgraded.
      |
      */
    pub fn purchase_product(
        &mut self, 
        product_identifier:             &String,
        upgrade_product_identifier:     &String,
        credit_for_unused_subscription: Option<bool>

    )  {

        let credit_for_unused_subscription: bool = credit_for_unused_subscription.unwrap_or(true);
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->purchaseInAppPurchasesProduct (productIdentifier, upgradeInAppPurchasesProductIdentifier, creditForUnusedSubscription);
       #else
        InAppPurchasesListenerPurchaseInfo purchaseInfo { InAppPurchase { "", productIdentifier, {}, {}, {} }, {} };

        listeners.call ([&] (Listener& l) { l.productInAppPurchaseFinished (purchaseInfo, false, "In-app purchases unavailable"); });
        ignoreUnused (upgradeInAppPurchasesProductIdentifier, creditForUnusedSubscription);
       #endif
        */
    }
    
    /**
      | Asynchronously asks about a list of
      | products that a user has already bought.
      | Upon completion, Listener::purchasesListReceived()
      | callback will be invoked. The user may
      | be prompted to login first.
      | 
      | -----------
      | @param includeInAppPurchasesDownloadInfo
      | 
      | (iOS only) if true, then after restoration
      | is successful, the downloads array
      | passed to
      | 
      | Listener::purchasesListReceived()
      | callback will contain all the download
      | objects corresponding with the purchase.
      | In the opposite case, the downloads
      | array will be empty.
      | ----------
      | @param subscriptionsSharedSecret
      | 
      | (iOS only) required when not including
      | download information and when there
      | are auto-renewable subscription set
      | up with this app. Refer to In-App-InAppPurchase
      | settings in the store.
      |
      */
    pub fn restore_products_bought_list(&mut self, 
        include_download_info:       bool,
        subscriptions_shared_secret: &String)  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->restoreInAppPurchasesProductsBoughtList (includeInAppPurchasesDownloadInfo, subscriptionsSharedSecret);
       #else
        listeners.call ([] (Listener& l) { l.purchasesListRestored ({}, false, "In-app purchases unavailable"); });
        ignoreUnused (includeInAppPurchasesDownloadInfo, subscriptionsSharedSecret);
       #endif
        */
    }
    
    /**
      | Android only: asynchronously sends
      | a request to mark a purchase with given
      | identifier as consumed.
      | 
      | To consume a product, provide product
      | identifier as well as a purchase token
      | that was generated when the product
      | was purchased. The purchase token can
      | also be retrieved by using getInAppPurchasesProductsInformation().
      | 
      | In general if it is available on hand,
      | it is better to use it, because otherwise
      | another async request will be sent to
      | the store, to first retrieve the token.
      | 
      | After successful consumption, a product
      | will no longer be returned in getInAppPurchasesProductsBought()
      | and it will be available for purchase.
      | 
      | On iOS consumption happens automatically.
      | If the product was set as consumable,
      | this function is a no-op.
      |
      */
    pub fn consume_purchase(&mut self, 
        product_identifier: &String,
        purchase_token:     &String)  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->consumeInAppPurchase (productIdentifier, purchaseToken);
       #else
        listeners.call ([&] (Listener& l) { l.productConsumed (productIdentifier, false, "In-app purchases unavailable"); });
        ignoreUnused (purchaseToken);
       #endif
        */
    }
    
    /**
      | Adds a listener.
      |
      */
    pub fn add_listener(&mut self, l: *mut dyn InAppPurchasesListenerInterface)  {
        
        todo!();
        /*
            listeners.add (l);
        */
    }
    
    /**
      | Removes a listener.
      |
      */
    pub fn remove_listener(&mut self, l: *mut dyn InAppPurchasesListenerInterface)  {
        
        todo!();
        /*
            listeners.remove (l);
        */
    }
    
    /**
      | iOS only: Starts downloads of hosted
      | content from the store.
      |
      */
    pub fn start_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->startInAppPurchasesDownloads (downloads);
       #else
        ignoreUnused (downloads);
       #endif
        */
    }
    
    /**
      | iOS only: Pauses downloads of hosted
      | content from the store.
      |
      */
    pub fn pause_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->pauseInAppPurchasesDownloads (downloads);
       #else
        ignoreUnused (downloads);
       #endif
        */
    }
    
    /**
      | iOS only: Resumes downloads of hosted
      | content from the store.
      |
      */
    pub fn resume_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->resumeInAppPurchasesDownloads (downloads);
       #else
        ignoreUnused (downloads);
       #endif
        */
    }
    
    /**
      | iOS only: Cancels downloads of hosted
      | content from the store.
      |
      */
    pub fn cancel_downloads(&mut self, downloads: &[*mut dyn InAppPurchasesDownload])  {
        
        todo!();
        /*
            #if ALOE_ANDROID || ALOE_IOS || ALOE_MAC
        pimpl->cancelInAppPurchasesDownloads (downloads);
       #else
        ignoreUnused (downloads);
       #endif
        */
    }
}

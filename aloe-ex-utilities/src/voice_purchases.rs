crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/InAppPurchasesDemo.h]

/**
  | To finish the setup of this demo, do the
  | following in the Proaloer project:
  | 
  | 1. In the project settings, set the "Bundle
  | Identifier" to com.rmsl.aloeInAppPurchaseSample
  | 
  | 2. In the Android exporter settings,
  | change the following settings:
  | 
  | - "In-App Billing" - Enabled
  | 
  | - "Key Signing: key.store" - path to
  | InAppPurchase.keystore file in examples/Assets/Signing
  | 
  | - "Key Signing: key.store.password"
  | - amazingvoices
  | 
  | - "Key Signing: key-alias" - InAppPurchase
  | 
  | - "Key Signing: key.alias.password"
  | - amazingvoices
  | 
  | 3. Re-save the project
  |
  */
#[no_copy]
#[leak_detector]
pub struct VoicePurchases<'a> {
    gui_updater:                  &'a mut AsyncUpdater<'a>,
    have_purchases_been_restored: bool, // default = false
    have_prices_been_fetched:     bool, // default = false
    purchase_in_progress:         bool, // default = false
    voice_products:               Vec<VoiceProduct>,
}

impl<'a> InAppPurchasesListenerInterface for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesProductDownloadFinished for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesProductDownloadPaused for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesProductDownloadProgressUpdate for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesProductConsumed for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesPurchaseListRestored for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesProductPurchaseFinished for VoicePurchases<'a> {

}

impl<'a> InAppPurchasesProductsInfoReturned for VoicePurchases<'a> {

}

impl<'a> Drop for VoicePurchases<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            InAppPurchases::getInstance()->removeListener (this);
         */
    }
}

impl<'a> VoicePurchases<'a> {

    pub fn new(async_updater: &mut AsyncUpdater) -> Self {
    
        todo!();
        /*
        : gui_updater(asyncUpdater),

            voiceProducts = Vec<VoiceProduct>(
                            { VoiceProduct {"robot",  "Robot",  true,   true,  false, "Free" },
                              VoiceProduct {"jules",  "Jules",  false,  false, false, "Retrieving price..." },
                              VoiceProduct {"fabian", "Fabian", false,  false, false, "Retrieving price..." },
                              VoiceProduct {"ed",     "Ed",     false,  false, false, "Retrieving price..." },
                              VoiceProduct {"lukasz", "Lukasz", false,  false, false, "Retrieving price..." },
                              VoiceProduct {"jb",     "JB",     false,  false, false, "Retrieving price..." } });
        */
    }
    
    pub fn get_purchase(&mut self, voice_index: i32) -> VoiceProduct {
        
        todo!();
        /*
            if (! havePurchasesBeenRestored)
            {
                havePurchasesBeenRestored = true;
                InAppPurchases::getInstance()->addListener (this);

                InAppPurchases::getInstance()->restoreProductsBoughtList (true);
            }

            return voiceProducts[voiceIndex];
        */
    }
    
    pub fn purchase_voice(&mut self, voice_index: i32)  {
        
        todo!();
        /*
            if (havePricesBeenFetched && isPositiveAndBelow (voiceIndex, voiceProducts.size()))
            {
                auto& product = voiceProducts.getReference (voiceIndex);

                if (! product.isPurchased)
                {
                    purchaseInProgress = true;

                    product.purchaseInProgress = true;
                    InAppPurchases::getInstance()->purchaseProduct (product.identifier);

                    guiUpdater.triggerAsyncUpdate();
                }
            }
        */
    }
    
    pub fn get_voice_names(&self) -> StringArray {
        
        todo!();
        /*
            StringArray names;

            for (auto& voiceProduct : voiceProducts)
                names.add (voiceProduct.humanReadable);

            return names;
        */
    }
    
    pub fn is_purchase_in_progress(&self) -> bool {
        
        todo!();
        /*
            return purchaseInProgress;
        */
    }
    
    pub fn products_info_returned(&mut self, products: &[InAppPurchasesProduct])  {
        
        todo!();
        /*
            if (! InAppPurchases::getInstance()->isInAppPurchasesSupported())
            {
                for (auto idx = 1; idx < voiceProducts.size(); ++idx)
                {
                    auto& voiceProduct = voiceProducts.getReference (idx);

                    voiceProduct.isPurchased  = false;
                    voiceProduct.priceIsKnown = false;
                    voiceProduct.purchasePrice = "In-App purchases unavailable";
                }

                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                  "In-app purchase is unavailable!",
                                                  "In-App purchases are not available. This either means you are trying "
                                                  "to use IAP on a platform that does not support IAP or you haven't setup "
                                                  "your app correctly to work with IAP.",
                                                  "OK");
            }
            else
            {
                for (auto product : products)
                {
                    auto idx = findVoiceIndexFromIdentifier (product.identifier);

                    if (isPositiveAndBelow (idx, voiceProducts.size()))
                    {
                        auto& voiceProduct = voiceProducts.getReference (idx);

                        voiceProduct.priceIsKnown = true;
                        voiceProduct.purchasePrice = product.price;
                    }
                }

                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                  "Your credit card will be charged!",
                                                  "You are running the sample code for Aloe In-App purchases. "
                                                  "Although this is only sample code, it will still CHARGE YOUR CREDIT CARD!",
                                                  "Understood!");
            }

            guiUpdater.triggerAsyncUpdate();
        */
    }
    
    pub fn product_purchase_finished(&mut self, 
        info:    &InAppPurchasesListenerPurchaseInfo,
        success: bool,
        _2:      &String)  {
        
        todo!();
        /*
            purchaseInProgress = false;

            auto idx = findVoiceIndexFromIdentifier (info.purchase.productId);

            if (isPositiveAndBelow (idx, voiceProducts.size()))
            {
                auto& voiceProduct = voiceProducts.getReference (idx);

                voiceProduct.isPurchased = success;
                voiceProduct.purchaseInProgress = false;
            }
            else
            {
                // On failure Play Store will not tell us which purchase failed
                for (auto& voiceProduct : voiceProducts)
                    voiceProduct.purchaseInProgress = false;
            }

            guiUpdater.triggerAsyncUpdate();
        */
    }
    
    pub fn purchases_list_restored(&mut self, 
        infos:   &[InAppPurchasesListenerPurchaseInfo],
        success: bool,
        _2:      &String)  {
        
        todo!();
        /*
            if (success)
            {
                for (auto& info : infos)
                {
                    auto idx = findVoiceIndexFromIdentifier (info.purchase.productId);

                    if (isPositiveAndBelow (idx, voiceProducts.size()))
                    {
                        auto& voiceProduct = voiceProducts.getReference (idx);

                        voiceProduct.isPurchased = true;
                    }
                }

                guiUpdater.triggerAsyncUpdate();
            }

            if (! havePricesBeenFetched)
            {
                havePricesBeenFetched = true;
                StringArray identifiers;

                for (auto& voiceProduct : voiceProducts)
                    identifiers.add (voiceProduct.identifier);

                InAppPurchases::getInstance()->getProductsInformation (identifiers);
            }
        */
    }
    
    pub fn find_voice_index_from_identifier(&self, identifier: String) -> i32 {
        
        todo!();
        /*
            identifier = identifier.toLowerCase();

            for (auto i = 0; i < voiceProducts.size(); ++i)
                if (String (voiceProducts.getReference (i).identifier) == identifier)
                    return i;

            return -1;
        */
    }
}

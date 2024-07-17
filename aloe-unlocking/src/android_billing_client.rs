crate::ix!();

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
              METHOD (constructor,                  "<init>",                     "(Landroid/content/Context;J)V")                                              
              METHOD (endConnection,                "endConnection",              "()V")                                                                        
              METHOD (isReady,                      "isReady",                    "()Z")                                                                        
              METHOD (isBillingSupported,           "isBillingSupported",         "()Z")                                                                        
              METHOD (querySkuDetails,              "querySkuDetails",            "([Ljava/lang/String;)V")                                                     
              METHOD (launchBillingFlow,            "launchBillingFlow",          "(Landroid/app/Activity;Lcom/android/billingclient/api/BillingFlowParams;)V") 
              METHOD (queryPurchases,               "queryPurchases",             "()V")                                                                        
              METHOD (consumePurchase,              "consumePurchase",            "(Ljava/lang/String;Ljava/lang/String;)V")                                    
                                                                                                                                                                
              CALLBACK (skuDetailsQueryCallback,    "skuDetailsQueryCallback",    "(JLjava/util/List;)V")                                                       
              CALLBACK (purchasesListQueryCallback, "purchasesListQueryCallback", "(JLjava/util/List;)V")                                                       
              CALLBACK (purchaseCompletedCallback,  "purchaseCompletedCallback",  "(JLcom/android/billingclient/api/Purchase;I)V")                              
              CALLBACK (purchaseConsumedCallback,   "purchaseConsumedCallback",   "(JLjava/lang/String;I)V")
        */
    }
}

declare_jni_class!{
    AloeBillingClient, 
    "com/rmsl/aloe/AloeBillingClient"
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/native/aloe_android_InAppPurchases.cpp]

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (getSku,               "getSku",               "()Ljava/lang/String;") 
          METHOD (getTitle,             "getTitle",             "()Ljava/lang/String;") 
          METHOD (getDescription,       "getDescription",       "()Ljava/lang/String;") 
          METHOD (getPrice,             "getPrice",             "()Ljava/lang/String;") 
          METHOD (getPriceCurrencyCode, "getPriceCurrencyCode", "()Ljava/lang/String;")
        */
    }
}

declare_jni_class!{
    SkuDetails, 
    "com/android/billingclient/api/SkuDetails"
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          STATICMETHOD (newBuilder, "newBuilder", "()Lcom/android/billingclient/api/BillingFlowParams$Builder;")
        */
    }
}

declare_jni_class!{
    BillingFlowParams, 
    "com/android/billingclient/api/BillingFlowParams"
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (build,                       "build",                       "()Lcom/android/billingclient/api/BillingFlowParams;")                                             
          METHOD (setOldSku,                   "setOldSku",                   "(Ljava/lang/String;Ljava/lang/String;)Lcom/android/billingclient/api/BillingFlowParams$Builder;") 
          METHOD (setReplaceSkusProrationMode, "setReplaceSkusProrationMode", "(I)Lcom/android/billingclient/api/BillingFlowParams$Builder;")                                    
          METHOD (setSkuDetails,               "setSkuDetails",               "(Lcom/android/billingclient/api/SkuDetails;)Lcom/android/billingclient/api/BillingFlowParams$Builder;")
        */
    }
}

declare_jni_class!{
    BillingFlowParamsBuilder, 
    "com/android/billingclient/api/BillingFlowParams$Builder"
}

macro_rules! jni_class_members {
    ($METHOD:ident, 
     $STATICMETHOD:ident, 
     $FIELD:ident, 
     $STATICFIELD:ident, 
     $CALLBACK:ident) => {
        /*
        
          METHOD (getOrderId,       "getOrderId",       "()Ljava/lang/String;") 
          METHOD (getSku,           "getSku",           "()Ljava/lang/String;") 
          METHOD (getPackageName,   "getPackageName",   "()Ljava/lang/String;") 
          METHOD (getPurchaseTime,  "getPurchaseTime",  "()J")                  
          METHOD (getPurchaseToken, "getPurchaseToken", "()Ljava/lang/String;")
        */
    }
}

declare_jni_class!{
    AndroidPurchase, 
    "com/android/billingclient/api/Purchase"
}

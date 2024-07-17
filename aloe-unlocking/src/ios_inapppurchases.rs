crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_product_unlocking/native/aloe_ios_InAppPurchases.cpp]

type PaymentObserverDelegate = NSObject/*<SKProductsRequestDelegate,SKPaymentTransactionObserver>*/;

pub struct SKDelegateAndPaymentObserver {
    delegate: Box<PaymentObserverDelegate,NSObjectDeleter>,
}

impl Default for SKDelegateAndPaymentObserver {
    
    fn default() -> Self {
        todo!();
        /*


            : delegate ([getClass().createInstance() init])
            SKDelegateAndPaymentObserverClass::setThis (delegate.get(), this);
        */
    }
}

impl SKDelegateAndPaymentObserver {

    pub fn get_class() -> &'static mut SKDelegateAndPaymentObserverClass {
        
        todo!();
        /*
            static SKDelegateAndPaymentObserverClass c;
            return c;
        */
    }
}

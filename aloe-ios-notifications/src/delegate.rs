crate::ix!();

type DelegateType = NSObject /*<UIApplicationDelegate>*/;

pub struct PushNotificationsDelegate {
    delegate: NSUniquePtr<DelegateType>,
}

impl Default for PushNotificationsDelegate {
    
    fn default() -> Self {
        todo!();
        /*


            : delegate ([getClass().createInstance() init])

            PushNotificationsDelegateClass::setThis (delegate.get(), this);

            id<UIApplicationDelegate> appDelegate = [[UIApplication sharedApplication] delegate];

            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
            if ([appDelegate respondsToSelector: @selector (setPushNotificationsDelegateToUse:)])
                [appDelegate performSelector: @selector (setPushNotificationsDelegateToUse:) withObject: delegate.get()];
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE
        */
    }
}

impl PushNotificationsDelegate {

    pub fn get_class<'a>() -> &'a mut PushNotificationsDelegateClass {
        
        todo!();
        /*
            static PushNotificationsDelegateClass c;
            return c;
        */
    }
}

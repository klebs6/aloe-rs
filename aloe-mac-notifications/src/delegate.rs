crate::ix!();

type DelegateType = NSObject/*<NSApplicationDelegate,NSUserNotificationCenterDelegate>*/;

pub struct PushNotificationsDelegate {
    delegate: NSUniquePtr<DelegateType>,
}

impl Default for PushNotificationsDelegate {
    
    fn default() -> Self {
        todo!();
        /*


            : delegate ([getClass().createInstance() init])
            PushNotificationsDelegateClass::setThis (delegate.get(), this);

            id<NSApplicationDelegate> appDelegate = [[NSApplication sharedApplication] delegate];

            ALOE_BEGIN_IGNORE_WARNINGS_GCC_LIKE ("-Wundeclared-selector")
            if ([appDelegate respondsToSelector: @selector (setPushNotificationsDelegate:)])
                [appDelegate performSelector: @selector (setPushNotificationsDelegate:) withObject: delegate.get()];
            ALOE_END_IGNORE_WARNINGS_GCC_LIKE

            [NSUserNotificationCenter defaultUserNotificationCenter].delegate = delegate.get();
        */
    }
}

impl Drop for PushNotificationsDelegate {

    fn drop(&mut self) {
        todo!();
        /*
            [NSUserNotificationCenter defaultUserNotificationCenter].delegate = nil;
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

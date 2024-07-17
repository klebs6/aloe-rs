crate::ix!();

pub fn push_notifications_delegate_action_to_ns_action(
    a:                 &PushNotificationsDelegateAction,
    os_earlier_than10: bool
) {
    
    todo!();
        /*
            if (iOSEarlierThan10)
            {
                auto action = [[UIMutableUserNotificationAction alloc] init];

                action.identifier     = aloeStringToNS (a.identifier);
                action.title          = aloeStringToNS (a.title);
                action.behavior       = a.style == PushNotificationsDelegateAction::text ? UIUserNotificationActionBehaviorTextInput
                                                                : UIUserNotificationActionBehaviorDefault;
                action.parameters     = varObjectToNSDictionary (a.parameters);
                action.activationMode = a.triggerInBackground ? UIUserNotificationActivationModeBackground
                                                              : UIUserNotificationActivationModeForeground;
                action.destructive    = (bool) a.destructive;

                [action autorelease];

                return action;
            }
            else
            {
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                if (a.style == PushNotificationsDelegateAction::text)
                {
                    return [UNTextInputNotificationAction actionWithIdentifier: aloeStringToNS (a.identifier)
                                                                         title: aloeStringToNS (a.title)
                                                                       options: NSUInteger (a.destructive << 1 | (! a.triggerInBackground) << 2)
                                                          textInputButtonTitle: aloeStringToNS (a.textInputButtonText)
                                                          textInputPlaceholder: aloeStringToNS (a.textInputPlaceholder)];
                }

                return [UNNotificationAction actionWithIdentifier: aloeStringToNS (a.identifier)
                                                            title: aloeStringToNS (a.title)
                                                          options: NSUInteger (a.destructive << 1 | (! a.triggerInBackground) << 2)];
               #else
                return nullptr;
               #endif
            }
        */
}

pub fn push_notifications_delegate_category_to_ns_category(
    c:                 &PushNotificationsDelegateCategory,
    os_earlier_than10: bool

) {
    
    todo!();
        /*
            if (iOSEarlierThan10)
            {
                auto category = [[UIMutableUserNotificationCategory alloc] init];
                category.identifier = aloeStringToNS (c.identifier);

                auto actions = [NSMutableArray arrayWithCapacity: (NSUInteger) c.actions.size()];

                for (const auto& a : c.actions)
                {
                    auto* action = (UIUserNotificationAction*) actionToNSAction (a, iOSEarlierThan10);
                    [actions addObject: action];
                }

                [category setActions: actions forContext: UIUserNotificationActionContextDefault];
                [category setActions: actions forContext: UIUserNotificationActionContextMinimal];

                [category autorelease];

                return category;
            }
            else
            {
               #if defined (__IPHONE_10_0) && __IPHONE_OS_VERSION_MAX_ALLOWED >= __IPHONE_10_0
                auto actions = [NSMutableArray arrayWithCapacity: (NSUInteger) c.actions.size()];

                for (const auto& a : c.actions)
                {
                    auto* action = (UNNotificationAction*) actionToNSAction (a, iOSEarlierThan10);
                    [actions addObject: action];
                }

                return [UNNotificationCategory categoryWithIdentifier: aloeStringToNS (c.identifier)
                                                              actions: actions
                                                    intentIdentifiers: @[]
                                                              options: c.sendDismissAction ? UNNotificationCategoryOptionCustomDismissAction : 0];
               #else
                return nullptr;
               #endif
            }
        */
}

pub fn push_notifications_delegate_aloe_notification_to_ui_local_notification(n: &PushNotification) -> *mut UILocalNotification {
    
    todo!();
        /*
            auto notification = [[UILocalNotification alloc] init];

            notification.alertTitle = aloeStringToNS (n.title);
            notification.alertBody  = aloeStringToNS (n.body);
            notification.category   = aloeStringToNS (n.category);
            notification.applicationIconBadgeNumber = n.badgeNumber;

            auto triggerTime = Time::getCurrentTime() + RelativeTime (n.triggerIntervalSec);
            notification.fireDate   = [NSDate dateWithTimeIntervalSince1970: triggerTime.toMilliseconds() / 1000.];
            notification.userInfo   = varObjectToNSDictionary (n.properties);

            auto soundToPlayString = n.soundToPlay.toString (true);

            if (soundToPlayString == "default_os_sound")
                notification.soundName = UILocalNotificationDefaultSoundName;
            else if (soundToPlayString.isNotEmpty())
                notification.soundName = aloeStringToNS (soundToPlayString);

            return notification;
        */
}

#[cfg(all(__IPHONE_10_0,__IPHONE_OS_VERSION_MAX_ALLOWED_GTE___IPHONE_10_0))]
pub fn push_notifications_delegate_aloe_notification_to_un_notification_request(n: &PushNotifications::Notification) -> *mut UNNotificationRequest {
    
    todo!();
        /*
            // content
            auto content = [[UNMutableNotificationContent alloc] init];

            content.title              = aloeStringToNS (n.title);
            content.subtitle           = aloeStringToNS (n.subtitle);
            content.threadIdentifier   = aloeStringToNS (n.groupId);
            content.body               = aloeStringToNS (n.body);
            content.categoryIdentifier = aloeStringToNS (n.category);
            content.badge              = [NSNumber numberWithInt: n.badgeNumber];

            auto soundToPlayString = n.soundToPlay.toString (true);

            if (soundToPlayString == "default_os_sound")
                content.sound = [UNNotificationSound defaultSound];
            else if (soundToPlayString.isNotEmpty())
                content.sound = [UNNotificationSound soundNamed: aloeStringToNS (soundToPlayString)];

            auto* propsDict = (NSMutableDictionary*) varObjectToNSDictionary (n.properties);
            [propsDict setObject: aloeStringToNS (soundToPlayString) forKey: nsStringLiteral ("com.aloe.soundName")];
            content.userInfo = propsDict;

            // trigger
            UNTimeIntervalNotificationTrigger* trigger = nil;

            if (std::abs (n.triggerIntervalSec) >= 0.001)
            {
                BOOL shouldRepeat = n.repeat && n.triggerIntervalSec >= 60;
                trigger = [UNTimeIntervalNotificationTrigger triggerWithTimeInterval: n.triggerIntervalSec repeats: shouldRepeat];
            }

            // request
            // each notification on iOS 10 needs to have an identifier, otherwise it will not show up
            jassert (n.identifier.isNotEmpty());
            UNNotificationRequest* request = [UNNotificationRequest requestWithIdentifier: aloeStringToNS (n.identifier)
                                                                                  content: content
                                                                                  trigger: trigger];

            [content autorelease];

            return request;
        */
}

pub fn push_notifications_delegate_get_user_response_from_ns_dictionary<K,V>(dictionary: *mut NSDictionary<K,V>) -> String {
    
    todo!();
        /*
            if (dictionary == nil || dictionary.count == 0)
                return {};

            jassert (dictionary.count == 1);

            for (NSString* key in dictionary)
            {
                const auto keyString = nsStringToAloe (key);

                id value = dictionary[key];

                if ([value isKindOfClass: [NSString class]])
                    return nsStringToAloe ((NSString*) value);
            }

            jassertfalse;
            return {};
        */
}

pub fn push_notifications_delegate_get_notification_properties_from_dictionary_var(dictionary_var: &Var) -> Var {
    
    todo!();
        /*
            DynamicObject* dictionaryVarObject = dictionaryVar.getDynamicObject();

            if (dictionaryVarObject == nullptr)
                return {};

            const auto& properties = dictionaryVarObject->getProperties();

            DynamicObject::Ptr propsVarObject = new DynamicObject();

            for (int i = 0; i < properties.size(); ++i)
            {
                auto propertyName = properties.getName (i).toString();

                if (propertyName == "aps")
                    continue;

                propsVarObject->setProperty (propertyName, properties.getValueAt (i));
            }

            return var (propsVarObject.get());
        */
}

#[cfg(all(__IPHONE_10_0,__IPHONE_OS_VERSION_MAX_ALLOWED_GTE___IPHONE_10_0))]
pub fn push_notifications_delegate_get_interval_sec_from_un_notification_trigger(t: *mut UNNotificationTrigger) -> f64 {
    
    todo!();
        /*
            if (t != nil)
            {
                if ([t isKindOfClass: [UNTimeIntervalNotificationTrigger class]])
                {
                    auto* trigger = (UNTimeIntervalNotificationTrigger*) t;
                    return trigger.timeInterval;
                }
                else if ([t isKindOfClass: [UNCalendarNotificationTrigger class]])
                {
                    auto* trigger = (UNCalendarNotificationTrigger*) t;
                    NSDate* date    = [trigger.dateComponents date];
                    NSDate* dateNow = [NSDate date];
                    return [dateNow timeIntervalSinceDate: date];
                }
            }

            return 0.;
        */
}

#[cfg(all(__IPHONE_10_0,__IPHONE_OS_VERSION_MAX_ALLOWED_GTE___IPHONE_10_0))]
pub fn push_notifications_delegate_un_notification_request_to_aloe_notification(r: *mut UNNotificationRequest) -> PushNotifications::Notification {
    
    todo!();
        /*
            PushNotifications::Notification n;

            n.identifier = nsStringToAloe (r.identifier);
            n.title      = nsStringToAloe (r.content.title);
            n.subtitle   = nsStringToAloe (r.content.subtitle);
            n.body       = nsStringToAloe (r.content.body);
            n.groupId    = nsStringToAloe (r.content.threadIdentifier);
            n.category   = nsStringToAloe (r.content.categoryIdentifier);
            n.badgeNumber = r.content.badge.intValue;

            auto userInfoVar = nsDictionaryToVar (r.content.userInfo);

            if (auto* object = userInfoVar.getDynamicObject())
            {
                static const Identifier soundName ("com.aloe.soundName");
                n.soundToPlay = Url (object->getProperty (soundName).toString());
                object->removeProperty (soundName);
            }

            n.properties = userInfoVar;

            n.triggerIntervalSec = getIntervalSecFromUNNotificationTrigger (r.trigger);
            n.repeat = r.trigger != nil && r.trigger.repeats;

            return n;
        */
}

#[cfg(all(__IPHONE_10_0,__IPHONE_OS_VERSION_MAX_ALLOWED_GTE___IPHONE_10_0))]
pub fn push_notifications_delegate_un_notification_to_aloe_notification(n: *mut UNNotification) -> PushNotifications::Notification {
    
    todo!();
        /*
            return unNotificationRequestToAloeNotification (n.request);
        */
}

pub fn push_notifications_delegate_ui_local_notification_to_aloe_notification(n: *mut UILocalNotification) -> PushNotification {
    
    todo!();
        /*
            PushNotifications::Notification notif;

            notif.title       = nsStringToAloe (n.alertTitle);
            notif.body        = nsStringToAloe (n.alertBody);

            if (n.fireDate != nil)
            {
                NSDate* dateNow = [NSDate date];
                NSDate* fireDate = n.fireDate;

                notif.triggerIntervalSec = [dateNow timeIntervalSinceDate: fireDate];
            }

            notif.soundToPlay = Url (nsStringToAloe (n.soundName));
            notif.badgeNumber = (int) n.applicationIconBadgeNumber;
            notif.category    = nsStringToAloe (n.category);
            notif.properties  = nsDictionaryToVar (n.userInfo);

            return notif;
        */
}

pub fn push_notifications_delegate_ui_user_notification_action_to_action(a: *mut UIUserNotificationAction) 
    -> PushNotificationsDelegateAction 
{
    todo!();
        /*
            PushNotificationsDelegateAction action;

            action.identifier = nsStringToAloe (a.identifier);
            action.title = nsStringToAloe (a.title);
            action.style = a.behavior == UIUserNotificationActionBehaviorTextInput
                         ? PushNotificationsDelegateAction::text
                         : PushNotificationsDelegateAction::button;

            action.triggerInBackground = a.activationMode == UIUserNotificationActivationModeBackground;
            action.destructive = a.destructive;
            action.parameters = nsDictionaryToVar (a.parameters);

            return action;
        */
}

pub fn push_notifications_delegate_ui_user_notification_category_to_category(c: *mut UIUserNotificationCategory) 
    -> PushNotificationsDelegateCategory 
{
    todo!();
        /*
            PushNotificationsDelegateCategory category;
            category.identifier = nsStringToAloe (c.identifier);

            for (UIUserNotificationAction* a in [c actionsForContext: UIUserNotificationActionContextDefault])
                category.actions.add (uiUserNotificationActionToAction (a));

            return category;
        */
}

#[cfg(all(__IPHONE_10_0,__IPHONE_OS_VERSION_MAX_ALLOWED_GTE___IPHONE_10_0))]
pub fn push_notifications_delegate_un_notification_action_to_action(a: *mut UNNotificationAction) -> PushNotificationsDelegateAction {
    
    todo!();
        /*
            PushNotificationsDelegateAction action;

            action.identifier = nsStringToAloe (a.identifier);
            action.title      = nsStringToAloe (a.title);
            action.triggerInBackground = ! (a.options & UNNotificationActionOptionForeground);
            action.destructive         =    a.options & UNNotificationActionOptionDestructive;

            if ([a isKindOfClass: [UNTextInputNotificationAction class]])
            {
                auto* textAction = (UNTextInputNotificationAction*)a;

                action.style = PushNotificationsDelegateAction::text;
                action.textInputButtonText  = nsStringToAloe (textAction.textInputButtonTitle);
                action.textInputPlaceholder = nsStringToAloe (textAction.textInputPlaceholder);
            }
            else
            {
                action.style = PushNotificationsDelegateAction::button;
            }

            return action;
        */
}

#[cfg(all(__IPHONE_10_0,__IPHONE_OS_VERSION_MAX_ALLOWED_GTE___IPHONE_10_0))]
pub fn push_notifications_delegate_un_notification_category_to_category(c: *mut UNNotificationCategory) -> PushNotificationsDelegateCategory {
    
    todo!();
        /*
            PushNotificationsDelegateCategory category;

            category.identifier = nsStringToAloe (c.identifier);
            category.sendDismissAction = c.options & UNNotificationCategoryOptionCustomDismissAction;

            for (UNNotificationAction* a in c.actions)
                category.actions.add (unNotificationActionToAction (a));

            return category;
        */
}

pub fn push_notifications_delegate_ns_dictionary_to_aloe_notification<K,V>(dictionary: *mut NSDictionary<K,V>) -> PushNotification {
    
    todo!();
        /*
            const var dictionaryVar = nsDictionaryToVar (dictionary);

            const var apsVar = dictionaryVar.getProperty ("aps", {});

            if (! apsVar.isObject())
                return {};

            var alertVar = apsVar.getProperty ("alert", {});

            const var titleVar = alertVar.getProperty ("title", {});
            const var bodyVar  = alertVar.isObject() ? alertVar.getProperty ("body", {}) : alertVar;

            const var categoryVar = apsVar.getProperty ("category", {});
            const var soundVar    = apsVar.getProperty ("sound", {});
            const var badgeVar    = apsVar.getProperty ("badge", {});
            const var threadIdVar = apsVar.getProperty ("thread-id", {});

            PushNotifications::Notification notification;

            notification.title       = titleVar   .toString();
            notification.body        = bodyVar    .toString();
            notification.groupId     = threadIdVar.toString();
            notification.category    = categoryVar.toString();
            notification.soundToPlay = Url (soundVar.toString());
            notification.badgeNumber = (int) badgeVar;
            notification.properties  = getNotificationPropertiesFromDictionaryVar (dictionaryVar);

            return notification;
        */
}

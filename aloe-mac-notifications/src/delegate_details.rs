crate::ix!();

pub fn aloe_notification_to_ns_user_notification(
    n:                         &PushNotification,
    is_earlier_than_mavericks: bool,
    is_earlier_than_yosemite:  bool

) -> *mut NSUserNotification {
    
    todo!();
        /*
            auto notification = [[NSUserNotification alloc] init];

            notification.title           = aloeStringToNS (n.title);
            notification.subtitle        = aloeStringToNS (n.subtitle);
            notification.informativeText = aloeStringToNS (n.body);
            notification.userInfo = varObjectToNSDictionary (n.properties);

            auto triggerTime = Time::getCurrentTime() + RelativeTime (n.triggerIntervalSec);
            notification.deliveryDate = [NSDate dateWithTimeIntervalSince1970: triggerTime.toMilliseconds() / 1000.];

            if (n.repeat && n.triggerIntervalSec >= 60)
            {
                auto dateComponents = [[NSDateComponents alloc] init];
                auto intervalSec = NSInteger (n.triggerIntervalSec);
                dateComponents.second = intervalSec;
                dateComponents.nanosecond = NSInteger ((n.triggerIntervalSec - intervalSec) * 1000000000);

                notification.deliveryRepeatInterval = dateComponents;

                [dateComponents autorelease];
            }

            auto soundToPlayString = n.soundToPlay.toString (true);

            if (soundToPlayString == "default_os_sound")
            {
                notification.soundName = NSUserNotificationDefaultSoundName;
            }
            else if (soundToPlayString.isNotEmpty())
            {
                auto* soundName = aloeStringToNS (soundToPlayString.fromLastOccurrenceOf ("/", false, false)
                                                                   .upToLastOccurrenceOf (".", false, false));

                notification.soundName = soundName;
            }

            notification.hasActionButton = n.actions.size() > 0;

            if (n.actions.size() > 0)
                notification.actionButtonTitle = aloeStringToNS (n.actions.getReference (0).title);

            if (! isEarlierThanMavericks)
            {
                notification.identifier = aloeStringToNS (n.identifier);

                if (n.actions.size() > 0)
                {
                    notification.hasReplyButton = n.actions.getReference (0).style == PushNotificationsDelegateAction::text;
                    notification.responsePlaceholder = aloeStringToNS (n.actions.getReference (0).textInputPlaceholder);
                }

                auto* imageDirectory = n.icon.contains ("/")
                                     ? aloeStringToNS (n.icon.upToLastOccurrenceOf ("/", false, true))
                                     : [NSString string];

                auto* imageName      = aloeStringToNS (n.icon.fromLastOccurrenceOf ("/", false, false)
                                                             .upToLastOccurrenceOf (".", false, false));
                auto* imageExtension = aloeStringToNS (n.icon.fromLastOccurrenceOf (".", false, false));

                NSString* imagePath = nil;

                if ([imageDirectory length] == NSUInteger (0))
                {
                    imagePath = [[NSBundle mainBundle] pathForResource: imageName
                                                                ofType: imageExtension];
                }
                else
                {
                    imagePath = [[NSBundle mainBundle] pathForResource: imageName
                                                                ofType: imageExtension
                                                           inDirectory: imageDirectory];
                }

                notification.contentImage = [[NSImage alloc] initWithContentsOfFile: imagePath];

                if (! isEarlierThanYosemite)
                {
                    if (n.actions.size() > 1)
                    {
                        auto additionalActions = [NSMutableArray arrayWithCapacity: (NSUInteger) n.actions.size() - 1];

                        for (int a = 1; a < n.actions.size(); ++a)
                            [additionalActions addObject: [NSUserNotificationAction actionWithIdentifier: aloeStringToNS (n.actions[a].identifier)
                                                                                                   title: aloeStringToNS (n.actions[a].title)]];

                        notification.additionalActions = additionalActions;
                    }
                }
            }

            [notification autorelease];

            return notification;
        */
}

pub fn ns_user_notification_to_aloe_notification(
    n:                         *mut NSUserNotification,
    is_earlier_than_mavericks: bool,
    is_earlier_than_yosemite:  bool

) -> PushNotification {
    
    todo!();
        /*
            PushNotifications::Notification notif;

            notif.title       = nsStringToAloe (n.title);
            notif.subtitle    = nsStringToAloe (n.subtitle);
            notif.body        = nsStringToAloe (n.informativeText);

            notif.repeat = n.deliveryRepeatInterval != nil;

            if (n.deliveryRepeatInterval != nil)
            {
                notif.triggerIntervalSec = n.deliveryRepeatInterval.second + (n.deliveryRepeatInterval.nanosecond / 1000000000.);
            }
            else
            {
                NSDate* dateNow = [NSDate date];
                NSDate* deliveryDate = n.deliveryDate;

                notif.triggerIntervalSec = [dateNow timeIntervalSinceDate: deliveryDate];
            }

            notif.soundToPlay = Url (nsStringToAloe (n.soundName));
            notif.properties  = nsDictionaryToVar (n.userInfo);

            if (! isEarlierThanMavericks)
            {
                notif.identifier = nsStringToAloe (n.identifier);

                if (n.contentImage != nil)
                    notif.icon = nsStringToAloe ([n.contentImage name]);
            }

            Vec<PushNotificationsDelegateAction> actions;

            if (n.actionButtonTitle != nil)
            {
                PushNotificationsDelegateAction action;
                action.title = nsStringToAloe (n.actionButtonTitle);

                if (! isEarlierThanMavericks)
                {
                    if (n.hasReplyButton)
                        action.style = PushNotificationsDelegateAction::text;

                    if (n.responsePlaceholder != nil)
                        action.textInputPlaceholder = nsStringToAloe (n.responsePlaceholder);
                }

                actions.add (action);
            }

            if (! isEarlierThanYosemite)
            {
                if (n.additionalActions != nil)
                {
                    for (NSUserNotificationAction* a in n.additionalActions)
                    {
                        PushNotificationsDelegateAction action;
                        action.identifier = nsStringToAloe (a.identifier);
                        action.title      = nsStringToAloe (a.title);

                        actions.add (action);
                    }
                }
            }

            return notif;
        */
}

pub fn get_notification_properties_from_dictionary_var(dictionary_var: &Var) -> Var {
    
    todo!();
        /*
            auto* dictionaryVarObject = dictionaryVar.getDynamicObject();

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

pub fn ns_dictionary_to_aloe_notification<K,V>(dictionary: *mut NSDictionary<K,V>) -> PushNotification {
    
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

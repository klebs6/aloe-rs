crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsPimpl<'a> {
    owner: &'a mut PushNotifications,
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsPimpl<'a> {

    pub fn new(p: &mut PushNotifications) -> Self {
    
        todo!();
        /*
        : owner(p),

        
        */
    }
    
    pub fn are_notifications_enabled(&self) -> bool {
        
        todo!();
        /*
            if (getAndroidSDKVersion() >= 24)
            {
                auto* env = getEnv();

                auto notificationManager = getNotificationManager();

                if (notificationManager.get() != nullptr)
                    return env->CallBooleanMethod (notificationManager, NotificationManagerApi24.areNotificationsEnabled);
            }

            return true;
        */
    }
    
    pub fn send_local_notification(&mut self, n: &PushNotifications::PushNotificationsNotification)  {
        
        todo!();
        /*
            // All required fields have to be setup!
            jassert (n.isValid());

            auto* env = getEnv();

            auto notificationManager = getNotificationManager();

            if (notificationManager.get() != nullptr)
            {
                auto notification = aloeNotificationToJavaNotification (n);

                auto tag = javaString (n.identifier);
                const int id = 0;

                env->CallVoidMethod (notificationManager.get(), NotificationManagerBase.notify, tag.get(), id, notification.get());
            }
        */
    }
    
    pub fn get_delivered_notifications(&self)  {
        
        todo!();
        /*
            if (getAndroidSDKVersion() >= 23)
            {
                auto* env = getEnv();

                Array<PushNotifications::PushNotificationsNotification> notifications;

                auto notificationManager = getNotificationManager();
                jassert (notificationManager != nullptr);

                if (notificationManager.get() != nullptr)
                {
                    auto statusBarNotifications = LocalRef<jobjectArray> ((jobjectArray)env->CallObjectMethod (notificationManager,
                                                                                                               NotificationManagerApi23.getActiveNotifications));

                    const int numNotifications = env->GetArrayLength (statusBarNotifications.get());

                    for (int i = 0; i < numNotifications; ++i)
                    {
                        auto statusBarNotification = LocalRef<jobject> (env->GetObjectArrayElement (statusBarNotifications.get(), (jsize) i));
                        auto notification = LocalRef<jobject> (env->CallObjectMethod (statusBarNotification, StatusBarNotification.getNotification));

                        notifications.add (javaNotificationToAloeNotification (notification));
                    }
                }

                owner.listeners.call ([&] (Listener& l) { l.deliveredNotificationsListReceived (notifications); });
            }
            else
            {
                // Not supported on this platform
                jassertfalse;
                owner.listeners.call ([] (Listener& l) { l.deliveredNotificationsListReceived ({}); });
            }
        */
    }
    
    pub fn notify_listeners_about_local_notification(&mut self, intent: &LocalRef<jobject>)  {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            auto bundle = LocalRef<jobject> (env->CallObjectMethod (intent, AndroidIntent.getExtras));

            const auto notification = localNotificationBundleToAloeNotification (bundle);

            auto packageName  = aloeString ((jstring) env->CallObjectMethod (context.get(), AndroidContext.getPackageName));

            String notificationString                = packageName + ".ALOE_NOTIFICATION.";
            String notificationButtonActionString    = packageName + ".ALOE_NOTIFICATION_BUTTON_ACTION.";
            String notificationTextInputActionString = packageName + ".ALOE_NOTIFICATION_TEXT_INPUT_ACTION.";

            auto actionString = aloeString ((jstring) env->CallObjectMethod (intent, AndroidIntent.getAction));

            if (actionString.contains (notificationString))
            {
                owner.listeners.call ([&] (Listener& l) { l.handleNotification (true, notification); });
            }
            else if (actionString.contains (notificationButtonActionString))
            {
                auto prefix = notificationButtonActionString + notification.identifier + ".";

                auto actionTitle = actionString.fromLastOccurrenceOf (prefix, false, false)     // skip prefix
                                               .fromFirstOccurrenceOf (".", false, false);      // skip action index

                owner.listeners.call ([&] (Listener& l) { l.handleNotificationAction (true, notification, actionTitle, {}); });
            }
            else if (getAndroidSDKVersion() >= 20 && actionString.contains (notificationTextInputActionString))
            {
                auto prefix = notificationTextInputActionString + notification.identifier + ".";

                auto actionTitle = actionString.fromLastOccurrenceOf (prefix, false, false)     // skip prefix
                                               .fromFirstOccurrenceOf (".", false, false);      // skip action index

                auto actionIndex = actionString.fromLastOccurrenceOf (prefix, false, false).upToFirstOccurrenceOf (".", false, false);
                auto resultKeyString = javaString (actionTitle + actionIndex);

                auto remoteInputResult = LocalRef<jobject> (env->CallStaticObjectMethod (RemoteInput, RemoteInput.getResultsFromIntent, intent.get()));
                String responseString;

                if (remoteInputResult.get() == nullptr)
                {
                    auto charSequence      = LocalRef<jobject> (env->CallObjectMethod (remoteInputResult, AndroidBundle.getCharSequence, resultKeyString.get()));
                    auto responseStringRef = LocalRef<jstring> ((jstring) env->CallObjectMethod (charSequence, JavaCharSequence.toString));
                    responseString = aloeString (responseStringRef.get());
                }

                owner.listeners.call ([&] (Listener& l) { l.handleNotificationAction (true, notification, actionTitle, responseString); });
            }
        */
    }
    
    pub fn notify_listeners_about_local_notification_deleted(&mut self, intent: &LocalRef<jobject>)  {
        
        todo!();
        /*
            auto* env = getEnv();

            auto bundle = LocalRef<jobject> (env->CallObjectMethod (intent, AndroidIntent.getExtras));
            auto notification = localNotificationBundleToAloeNotification (bundle);

            owner.listeners.call ([&] (Listener& l) { l.localNotificationDismissedByUser (notification); });
        */
    }
    
    pub fn remove_all_delivered_notifications(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

            auto notificationManager = getNotificationManager();

            if (notificationManager.get() != nullptr)
                env->CallVoidMethod (notificationManager.get(), NotificationManagerBase.cancelAll);
        */
    }
    
    pub fn remove_delivered_notification(&mut self, identifier: &String)  {
        
        todo!();
        /*
            auto* env = getEnv();

            auto notificationManager = getNotificationManager();

            if (notificationManager.get() != nullptr)
            {
                auto tag = javaString (identifier);
                const int id = 0;

                env->CallVoidMethod (notificationManager.get(), NotificationManagerBase.cancel, tag.get(), id);
            }
        */
    }
    
    pub fn get_device_token(&self) -> String {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_INSTANCE_ID_SERVICE_CLASSNAME)
            auto* env = getEnv();

            auto instanceId = LocalRef<jobject> (env->CallStaticObjectMethod (FirebaseInstanceId, FirebaseInstanceId.getInstance));

            return aloeString ((jstring) env->CallObjectMethod (instanceId, FirebaseInstanceId.getToken));
          #else
            return {};
          #endif
        */
    }
    
    pub fn notify_listeners_token_refreshed(&mut self, token: &String)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_INSTANCE_ID_SERVICE_CLASSNAME)
            MessageManager::callAsync ([this, token]
            {
                owner.listeners.call ([&] (Listener& l) { l.deviceTokenRefreshed (token); });
            });
          #else
            ignoreUnused (token);
          #endif
        */
    }
    
    pub fn subscribe_to_topic(&mut self, topic: &String)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            auto* env = getEnv();

            auto firebaseMessaging = LocalRef<jobject> (env->CallStaticObjectMethod (FirebaseMessaging,
                                                                                     FirebaseMessaging.getInstance));

            env->CallObjectMethod (firebaseMessaging, FirebaseMessaging.subscribeToTopic, javaString (topic).get());
          #else
            ignoreUnused (topic);
          #endif
        */
    }
    
    pub fn unsubscribe_from_topic(&mut self, topic: &String)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            auto* env = getEnv();

            auto firebaseMessaging = LocalRef<jobject> (env->CallStaticObjectMethod (FirebaseMessaging,
                                                                                     FirebaseMessaging.getInstance));

            env->CallObjectMethod (firebaseMessaging, FirebaseMessaging.unsubscribeFromTopic, javaString (topic).get());
          #else
            ignoreUnused (topic);
          #endif
        */
    }
    
    pub fn send_upstream_message(&mut self, 
        server_sender_id: &String,
        collapse_key:     &String,
        message_id:       &String,
        message_type:     &String,
        time_to_live:     i32,
        additional_data:  &StringPairArray)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            auto* env = getEnv();

            auto messageBuilder = LocalRef<jobject> (env->NewObject (RemoteMessageBuilder,
                                                                     RemoteMessageBuilder.constructor,
                                                                     javaString (serverSenderId + "@gcm_googleapis.com").get()));

            env->CallObjectMethod (messageBuilder, RemoteMessageBuilder.setCollapseKey, javaString (collapseKey).get());
            env->CallObjectMethod (messageBuilder, RemoteMessageBuilder.setMessageId, javaString (messageId).get());
            env->CallObjectMethod (messageBuilder, RemoteMessageBuilder.setMessageType, javaString (messageType).get());
            env->CallObjectMethod (messageBuilder, RemoteMessageBuilder.setTtl, timeToLive);

            auto keys = additionalData.getAllKeys();

            for (const auto& key : keys)
                env->CallObjectMethod (messageBuilder,
                                       RemoteMessageBuilder.addData,
                                       javaString (key).get(),
                                       javaString (additionalData[key]).get());

            auto message = LocalRef<jobject> (env->CallObjectMethod (messageBuilder, RemoteMessageBuilder.build));

            auto firebaseMessaging = LocalRef<jobject> (env->CallStaticObjectMethod (FirebaseMessaging,
                                                                                     FirebaseMessaging.getInstance));

            env->CallVoidMethod (firebaseMessaging, FirebaseMessaging.send, message.get());
          #else
            ignoreUnused (serverSenderId, collapseKey, messageId, messageType);
            ignoreUnused (timeToLive, additionalData);
          #endif
        */
    }
    
    pub fn notify_listeners_about_remote_notification_from_system_tray(&mut self, intent: &LocalRef<jobject>)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            auto* env = getEnv();

            auto bundle = LocalRef<jobject> (env->CallObjectMethod (intent, AndroidIntent.getExtras));
            auto notification = remoteNotificationBundleToAloeNotification (bundle);

            owner.listeners.call ([&] (Listener& l) { l.handleNotification (false, notification); });
          #else
            ignoreUnused (intent);
          #endif
        */
    }
    
    pub fn notify_listeners_about_remote_notification_from_service(&mut self, remote_notification: &LocalRef<jobject>)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            GlobalRef rn (remoteNotification);

            MessageManager::callAsync ([this, rn]
            {
                auto notification = firebaseRemoteNotificationToAloeNotification (rn.get());
                owner.listeners.call ([&] (Listener& l) { l.handleNotification (false, notification); });
            });
          #else
            ignoreUnused (remoteNotification);
          #endif
        */
    }
    
    pub fn notify_listeners_about_remote_notifications_deleted(&mut self)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            MessageManager::callAsync ([this]
            {
                owner.listeners.call ([] (Listener& l) { l.remoteNotificationsDeleted(); });
            });
          #endif
        */
    }
    
    pub fn notify_listeners_about_upstream_message_sent(&mut self, message_id: &LocalRef<jstring>)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            GlobalRef mid (LocalRef<jobject>(messageId.get()));

            MessageManager::callAsync ([this, mid]
            {
                auto midString = aloeString ((jstring) mid.get());
                owner.listeners.call ([&] (Listener& l) { l.upstreamMessageSent (midString); });
            });
          #else
            ignoreUnused (messageId);
          #endif
        */
    }
    
    pub fn notify_listeners_about_upstream_message_sending_error(&mut self, 
        message_id: &LocalRef<jstring>,
        error:      &LocalRef<jstring>)  {
        
        todo!();
        /*
            #if defined(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)
            GlobalRef mid (LocalRef<jobject>(messageId.get())), e (LocalRef<jobject>(error.get()));

            MessageManager::callAsync ([this, mid, e]
            {
                auto midString = aloeString ((jstring) mid.get());
                auto eString   = aloeString ((jstring) e.get());

                owner.listeners.call ([&] (Listener& l) { l.upstreamMessageSendingError (midString, eString); });
            });
          #else
            ignoreUnused (messageId, error);
          #endif
        */
    }
    
    pub fn get_notification_manager() -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            return LocalRef<jobject> (env->CallObjectMethod (context.get(),
                                                             AndroidContext.getSystemService,
                                                             javaString ("notification").get()));
        */
    }
    
    pub fn aloe_notification_to_java_notification(n: &PushNotifications::PushNotificationsNotification) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

            auto notificationBuilder = createNotificationBuilder (n);

            setupRequiredFields (n, notificationBuilder);
            setupOptionalFields (n, notificationBuilder);

            if (n.actions.size() > 0)
                setupActions (n, notificationBuilder);

            if (getAndroidSDKVersion() >= 16)
                return LocalRef<jobject> (env->CallObjectMethod (notificationBuilder, NotificationBuilderApi16.build));

            return LocalRef<jobject> (env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.getNotification));
        */
    }
    
    pub fn create_notification_builder(n: &PushNotifications::PushNotificationsNotification) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            jclass builderClass = env->FindClass ("android/app/PushNotificationsNotification$Builder");
            jassert (builderClass != nullptr);

            if (builderClass == nullptr)
                return LocalRef<jobject> (nullptr);

            jmethodID builderConstructor = nullptr;

            const bool apiAtLeast26 = (getAndroidSDKVersion() >= 26);

            if (apiAtLeast26)
                builderConstructor = env->GetMethodID (builderClass, "<init>", "(Landroid/content/Context;Ljava/lang/String;)V");
            else
                builderConstructor = env->GetMethodID (builderClass, "<init>", "(Landroid/content/Context;)V");

            jassert (builderConstructor != nullptr);

            if (builderConstructor == nullptr)
                return LocalRef<jobject> (nullptr);

            if (apiAtLeast26)
                return LocalRef<jobject> (env->NewObject (builderClass, builderConstructor,
                                                          context.get(), javaString (n.channelId).get()));

            return LocalRef<jobject> (env->NewObject (builderClass, builderConstructor, context.get()));
        */
    }
    
    pub fn setup_required_fields(
        n:                    &PushNotifications::PushNotificationsNotification,
        notification_builder: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            auto activityClass = LocalRef<jobject> (env->CallObjectMethod (context.get(), JavaObject.getClass));
            auto notifyIntent  = LocalRef<jobject> (env->NewObject (AndroidIntent, AndroidIntent.constructorWithContextAndClass, context.get(), activityClass.get()));

            auto packageNameString  = LocalRef<jstring> ((jstring) (env->CallObjectMethod (context.get(), AndroidContext.getPackageName)));
            auto actionStringSuffix = javaString (".ALOE_NOTIFICATION." + n.identifier);
            auto actionString       = LocalRef<jstring> ((jstring)env->CallObjectMethod (packageNameString, JavaString.concat, actionStringSuffix.get()));

            env->CallObjectMethod (notifyIntent, AndroidIntent.setAction, actionString.get());
            // Packaging entire notification into extras bundle here, so that we can retrieve all the details later on
            env->CallObjectMethod (notifyIntent, AndroidIntent.putExtras, aloeNotificationToBundle (n).get());

            auto notifyPendingIntent = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidPendingIntent,
                                                                                       AndroidPendingIntent.getActivity,
                                                                                       context.get(),
                                                                                       1002,
                                                                                       notifyIntent.get(),
                                                                                       0));

            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setContentTitle,  javaString (n.title).get());
            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setContentText,   javaString (n.body).get());
            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setContentIntent, notifyPendingIntent.get());

            auto resources = LocalRef<jobject> (env->CallObjectMethod (context.get(), AndroidContext.getResources));
            const int iconId = env->CallIntMethod (resources, AndroidResources.getIdentifier, javaString (n.icon).get(),
                                                   javaString ("raw").get(), packageNameString.get());

            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setSmallIcon, iconId);

            if (getAndroidSDKVersion() >= 21 && n.publicVersion != nullptr)
            {
                // Public version of a notification is not expected to have another public one!
                jassert (n.publicVersion->publicVersion == nullptr);

                auto publicNotificationBuilder = createNotificationBuilder (n);

                setupRequiredFields (*n.publicVersion, publicNotificationBuilder);
                setupOptionalFields (*n.publicVersion, publicNotificationBuilder);

                auto publicVersion = LocalRef<jobject> (env->CallObjectMethod (publicNotificationBuilder, NotificationBuilderApi16.build));
                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi21.setPublicVersion, publicVersion.get());
            }
        */
    }
    
    pub fn aloe_notification_to_bundle(n: &PushNotifications::PushNotificationsNotification) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

            auto bundle = LocalRef<jobject> (env->NewObject (AndroidBundle, AndroidBundle.constructor));

            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("identifier")              .get(), javaString (n.identifier).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("title")                   .get(), javaString (n.title).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("body")                    .get(), javaString (n.body).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("subtitle")                .get(), javaString (n.subtitle).get());
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("badgeNumber")             .get(), n.badgeNumber);
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("soundToPlay")             .get(), javaString (n.soundToPlay.toString (true)).get());
            env->CallVoidMethod (bundle, AndroidBundle.putBundle,   javaString ("properties")              .get(), varToBundleWithPropertiesString (n.properties).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("icon")                    .get(), javaString (n.icon).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("channelId")               .get(), javaString (n.channelId).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("tickerText")              .get(), javaString (n.tickerText).get());
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("progressMax")             .get(), n.progress.max);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("progressCurrent")         .get(), n.progress.current);
            env->CallVoidMethod (bundle, AndroidBundle.putBoolean,  javaString ("progressIndeterminate")   .get(), n.progress.indeterminate);
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("person")                  .get(), javaString (n.person).get());
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("type")                    .get(), n.type);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("priority")                .get(), n.priority);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("lockScreenAppearance")    .get(), n.lockScreenAppearance);
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("groupId")                 .get(), javaString (n.groupId).get());
            env->CallVoidMethod (bundle, AndroidBundle.putString,   javaString ("groupSortKey")            .get(), javaString (n.groupSortKey).get());
            env->CallVoidMethod (bundle, AndroidBundle.putBoolean,  javaString ("groupSummary")            .get(), n.groupSummary);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("accentColour")            .get(), n.accentColour.getARGB());
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("ledColour")               .get(), n.ledColour.getARGB());
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("ledBlinkPatternMsToBeOn") .get(), n.ledBlinkPattern.msToBeOn);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("ledBlinkPatternMsToBeOff").get(), n.ledBlinkPattern.msToBeOff);
            env->CallVoidMethod (bundle, AndroidBundle.putBoolean,  javaString ("shouldAutoCancel")        .get(), n.shouldAutoCancel);
            env->CallVoidMethod (bundle, AndroidBundle.putBoolean,  javaString ("localOnly")               .get(), n.localOnly);
            env->CallVoidMethod (bundle, AndroidBundle.putBoolean,  javaString ("ongoing")                 .get(), n.ongoing);
            env->CallVoidMethod (bundle, AndroidBundle.putBoolean,  javaString ("alertOnlyOnce")           .get(), n.alertOnlyOnce);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("timestampVisibility")     .get(), n.timestampVisibility);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("badgeIconType")           .get(), n.badgeIconType);
            env->CallVoidMethod (bundle, AndroidBundle.putInt,      javaString ("groupAlertBehaviour")     .get(), n.groupAlertBehaviour);
            env->CallVoidMethod (bundle, AndroidBundle.putLong,     javaString ("timeoutAfterMs")          .get(), (jlong)n.timeoutAfterMs);

            const int size = n.vibrationPattern.size();

            if (size > 0)
            {
                auto array = LocalRef<jlongArray> (env->NewLongArray (size));

                jlong* elements = env->GetLongArrayElements (array, nullptr);

                for (int i = 0; i < size; ++i)
                    elements[i] = (jlong) n.vibrationPattern[i];

                env->SetLongArrayRegion (array, 0, size, elements);
                env->CallVoidMethod (bundle, AndroidBundle.putLongArray, javaString ("vibrationPattern").get(), array.get());
            }

            return bundle;
        */
    }
    
    pub fn setup_optional_fields(
        n:                    &PushNotifications::PushNotificationsNotification,
        notification_builder: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            auto* env = getEnv();

            if (n.subtitle.isNotEmpty())
                env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setContentInfo, javaString (n.subtitle).get());

            auto soundName = n.soundToPlay.toString (true);

            if (soundName == "default_os_sound")
            {
                const int playDefaultSound = 1;
                env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setDefaults, playDefaultSound);
            }
            else if (! soundName.isEmpty())
            {
                env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setSound, aloeUrlToAndroidUri (n.soundToPlay).get());
            }

            if (n.largeIcon.isValid())
                env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setLargeIcon, imagetoJavaBitmap (n.largeIcon).get());

            if (n.tickerText.isNotEmpty())
                env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setTicker, javaString (n.tickerText).get());

            if (n.ledColour != Colour())
            {
                env->CallObjectMethod (notificationBuilder,
                                       NotificationBuilderBase.setLights,
                                       n.ledColour.getARGB(),
                                       n.ledBlinkPattern.msToBeOn,
                                       n.ledBlinkPattern.msToBeOff);
            }

            if (! n.vibrationPattern.isEmpty())
            {
                const int size = n.vibrationPattern.size();

                if (size > 0)
                {
                    auto array = LocalRef<jlongArray> (env->NewLongArray (size));

                    jlong* elements = env->GetLongArrayElements (array, nullptr);

                    for (int i = 0; i < size; ++i)
                        elements[i] = (jlong) n.vibrationPattern[i];

                    env->SetLongArrayRegion (array, 0, size, elements);
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setVibrate, array.get());
                }
            }

            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setProgress, n.progress.max, n.progress.current, n.progress.indeterminate);
            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setNumber, n.badgeNumber);
            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setAutoCancel, n.shouldAutoCancel);
            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setOngoing, n.ongoing);
            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setOnlyAlertOnce, n.alertOnlyOnce);

            if (getAndroidSDKVersion() >= 16)
            {
                if (n.subtitle.isNotEmpty())
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi16.setSubText, javaString (n.subtitle).get());

                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi16.setPriority, n.priority);

                if (getAndroidSDKVersion() < 24)
                {
                    const bool useChronometer = n.timestampVisibility == PushNotifications::PushNotificationsNotification::chronometer;
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi16.setUsesChronometer, useChronometer);
                }
            }

            if (getAndroidSDKVersion() >= 17)
            {
                const bool showTimeStamp = n.timestampVisibility != PushNotifications::PushNotificationsNotification::off;
                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi17.setShowWhen, showTimeStamp);
            }

            if (getAndroidSDKVersion() >= 20)
            {
                if (n.groupId.isNotEmpty())
                {
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi20.setGroup, javaString (n.groupId).get());
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi20.setGroupSummary, n.groupSummary);
                }

                if (n.groupSortKey.isNotEmpty())
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi20.setSortKey, javaString (n.groupSortKey).get());

                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi20.setLocalOnly, n.localOnly);

                auto extras = LocalRef<jobject> (env->NewObject (AndroidBundle, AndroidBundle.constructor));

                env->CallVoidMethod (extras, AndroidBundle.putBundle, javaString ("notificationData").get(),
                                     aloeNotificationToBundle (n).get());

                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi20.addExtras, extras.get());
            }

            if (getAndroidSDKVersion() >= 21)
            {
                if (n.person.isNotEmpty())
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi21.addPerson, javaString (n.person).get());

                auto categoryString = typeToCategory (n.type);
                if (categoryString.isNotEmpty())
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi21.setCategory, javaString (categoryString).get());

                if (n.accentColour != Colour())
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi21.setColor, n.accentColour.getARGB());

                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi21.setVisibility, n.lockScreenAppearance);
            }

            if (getAndroidSDKVersion() >= 24)
            {
                const bool useChronometer = n.timestampVisibility == PushNotifications::PushNotificationsNotification::chronometer;
                const bool useCountDownChronometer = n.timestampVisibility == PushNotifications::PushNotificationsNotification::countDownChronometer;

                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi24.setChronometerCountDown, useCountDownChronometer);
                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi16.setUsesChronometer, useChronometer | useCountDownChronometer);
            }

            if (getAndroidSDKVersion() >= 26)
            {
                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi26.setBadgeIconType, n.badgeIconType);
                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi26.setGroupAlertBehavior, n.groupAlertBehaviour);
                env->CallObjectMethod (notificationBuilder, NotificationBuilderApi26.setTimeoutAfter, (jlong) n.timeoutAfterMs);
            }

            setupNotificationDeletedCallback (n, notificationBuilder);
        */
    }
    
    pub fn setup_notification_deleted_callback(
        n:                    &PushNotifications::PushNotificationsNotification,
        notification_builder: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            auto activityClass = LocalRef<jobject> (env->CallObjectMethod (context.get(), JavaObject.getClass));
            auto deleteIntent  = LocalRef<jobject> (env->NewObject (AndroidIntent, AndroidIntent.constructorWithContextAndClass, context.get(), activityClass.get()));

            auto packageNameString  = LocalRef<jstring> ((jstring) (env->CallObjectMethod (context.get(), AndroidContext.getPackageName)));
            auto actionStringSuffix = javaString (".ALOE_NOTIFICATION_DELETED." + n.identifier);
            auto actionString       = LocalRef<jstring> ((jstring)env->CallObjectMethod (packageNameString, JavaString.concat, actionStringSuffix.get()));

            env->CallObjectMethod (deleteIntent, AndroidIntent.setAction, actionString.get());
            env->CallObjectMethod (deleteIntent, AndroidIntent.putExtras, aloeNotificationToBundle (n).get());

            auto deletePendingIntent = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidPendingIntent,
                                                                                       AndroidPendingIntent.getActivity,
                                                                                       context.get(),
                                                                                       1002,
                                                                                       deleteIntent.get(),
                                                                                       0));

            env->CallObjectMethod (notificationBuilder, NotificationBuilderBase.setDeleteIntent, deletePendingIntent.get());
        */
    }
    
    pub fn setup_actions(
        n:                    &PushNotifications::PushNotificationsNotification,
        notification_builder: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 16)
                return;

            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            int actionIndex = 0;

            for (const auto& action : n.actions)
            {
                auto activityClass = LocalRef<jobject> (env->CallObjectMethod (context.get(), JavaObject.getClass));
                auto notifyIntent  = LocalRef<jobject> (env->NewObject (AndroidIntent, AndroidIntent.constructorWithContextAndClass, context.get(), activityClass.get()));

                const bool isTextStyle = action.style == PushNotifications::PushNotificationsNotification::Action::text;

                auto packageNameString   = LocalRef<jstring> ((jstring) (env->CallObjectMethod (context.get(), AndroidContext.getPackageName)));
                const String notificationActionString = isTextStyle ? ".ALOE_NOTIFICATION_TEXT_INPUT_ACTION." : ".ALOE_NOTIFICATION_BUTTON_ACTION.";
                auto actionStringSuffix  = javaString (notificationActionString + n.identifier + "." + String (actionIndex) + "." + action.title);
                auto actionString        = LocalRef<jstring> ((jstring)env->CallObjectMethod (packageNameString, JavaString.concat, actionStringSuffix.get()));

                env->CallObjectMethod (notifyIntent, AndroidIntent.setAction, actionString.get());
                // Packaging entire notification into extras bundle here, so that we can retrieve all the details later on
                env->CallObjectMethod (notifyIntent, AndroidIntent.putExtras, aloeNotificationToBundle (n).get());

                auto notifyPendingIntent = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidPendingIntent,
                                                                                           AndroidPendingIntent.getActivity,
                                                                                           context.get(),
                                                                                           1002,
                                                                                           notifyIntent.get(),
                                                                                           0));

                auto resources = LocalRef<jobject> (env->CallObjectMethod (context.get(), AndroidContext.getResources));
                int iconId = env->CallIntMethod (resources, AndroidResources.getIdentifier, javaString (action.icon).get(),
                                                 javaString ("raw").get(), packageNameString.get());

                if (iconId == 0)
                    iconId = env->CallIntMethod (resources, AndroidResources.getIdentifier, javaString (n.icon).get(),
                                                 javaString ("raw").get(), packageNameString.get());

                if (getAndroidSDKVersion() >= 20)
                {
                    auto actionBuilder = LocalRef<jobject> (env->NewObject (NotificationActionBuilder,
                                                                            NotificationActionBuilder.constructor,
                                                                            iconId,
                                                                            javaString (action.title).get(),
                                                                            notifyPendingIntent.get()));

                    env->CallObjectMethod (actionBuilder, NotificationActionBuilder.addExtras,
                                           varToBundleWithPropertiesString (action.parameters).get());

                    if (isTextStyle)
                    {
                        auto resultKey = javaString (action.title + String (actionIndex));
                        auto remoteInputBuilder = LocalRef<jobject> (env->NewObject (RemoteInputBuilder,
                                                                                     RemoteInputBuilder.constructor,
                                                                                     resultKey.get()));

                        if (! action.textInputPlaceholder.isEmpty())
                            env->CallObjectMethod (remoteInputBuilder, RemoteInputBuilder.setLabel, javaString (action.textInputPlaceholder).get());

                        if (! action.allowedResponses.isEmpty())
                        {
                            env->CallObjectMethod (remoteInputBuilder, RemoteInputBuilder.setAllowFreeFormInput, false);

                            const int size = action.allowedResponses.size();

                            auto array = LocalRef<jobjectArray> (env->NewObjectArray (size, env->FindClass ("java/lang/String"), nullptr));

                            for (int i = 0; i < size; ++i)
                            {
                                const auto& response = action.allowedResponses[i];
                                auto responseString = javaString (response);

                                env->SetObjectArrayElement (array, i, responseString.get());
                            }

                            env->CallObjectMethod (remoteInputBuilder, RemoteInputBuilder.setChoices, array.get());
                        }

                        env->CallObjectMethod (actionBuilder, NotificationActionBuilder.addRemoteInput,
                                               env->CallObjectMethod (remoteInputBuilder, RemoteInputBuilder.build));
                    }

                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi20.addAction,
                                           env->CallObjectMethod (actionBuilder, NotificationActionBuilder.build));
                }
                else
                {
                    env->CallObjectMethod (notificationBuilder, NotificationBuilderApi16.addAction,
                                           iconId, javaString (action.title).get(), notifyPendingIntent.get());
                }

                ++actionIndex;
            }
        */
    }
    
    pub fn aloe_url_to_android_uri(url: &Url) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            auto packageNameString = LocalRef<jstring> ((jstring) (env->CallObjectMethod (context.get(), AndroidContext.getPackageName)));

            auto resources = LocalRef<jobject> (env->CallObjectMethod (context.get(), AndroidContext.getResources));
            const int id = env->CallIntMethod (resources, AndroidResources.getIdentifier, javaString (url.toString (true)).get(),
                                               javaString ("raw").get(), packageNameString.get());

            auto schemeString   = javaString ("android.resource://");
            auto resourceString = javaString ("/" + String (id));
            auto uriString = LocalRef<jstring> ((jstring) env->CallObjectMethod (schemeString, JavaString.concat, packageNameString.get()));
            uriString = LocalRef<jstring> ((jstring) env->CallObjectMethod (uriString, JavaString.concat, resourceString.get()));

            return LocalRef<jobject> (env->CallStaticObjectMethod (AndroidUri, AndroidUri.parse, uriString.get()));
        */
    }
    
    pub fn imageto_java_bitmap(image: &Image) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

            Image imageToUse = image.convertedToFormat (Image::PixelFormat::ARGB);

            auto bitmapConfig = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidBitmapConfig,
                                                                                AndroidBitmapConfig.valueOf,
                                                                                javaString ("ARGB_8888").get()));

            auto bitmap = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidBitmap,
                                                                          AndroidBitmap.createBitmap,
                                                                          image.getWidth(),
                                                                          image.getHeight(),
                                                                          bitmapConfig.get()));

            for (int i = 0; i < image.getWidth(); ++i)
                for (int j = 0; j < image.getHeight(); ++j)
                    env->CallVoidMethod (bitmap.get(), AndroidBitmap.setPixel, i, j, image.getPixelAt (i, j).getARGB());

            return bitmap;
        */
    }
    
    pub fn type_to_category(t: PushNotifications::PushNotificationsNotification::Type) -> String {
        
        todo!();
        /*
            switch (t)
            {
                case PushNotifications::PushNotificationsNotification::unspecified:    return {};
                case PushNotifications::PushNotificationsNotification::alarm:          return "alarm";
                case PushNotifications::PushNotificationsNotification::call:           return "call";
                case PushNotifications::PushNotificationsNotification::email:          return "email";
                case PushNotifications::PushNotificationsNotification::error:          return "err";
                case PushNotifications::PushNotificationsNotification::event:          return "event";
                case PushNotifications::PushNotificationsNotification::message:        return "msg";
                case PushNotifications::PushNotificationsNotification::taskProgress:   return "progress";
                case PushNotifications::PushNotificationsNotification::promo:          return "promo";
                case PushNotifications::PushNotificationsNotification::recommendation: return "recommendation";
                case PushNotifications::PushNotificationsNotification::reminder:       return "reminder";
                case PushNotifications::PushNotificationsNotification::service:        return "service";
                case PushNotifications::PushNotificationsNotification::social:         return "social";
                case PushNotifications::PushNotificationsNotification::status:         return "status";
                case PushNotifications::PushNotificationsNotification::system:         return "sys";
                case PushNotifications::PushNotificationsNotification::transport:      return "transport";
            }

            return {};
        */
    }
    
    pub fn var_to_bundle_with_properties_string(var_to_parse: &Var) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

            auto bundle = LocalRef<jobject> (env->NewObject (AndroidBundle, AndroidBundle.constructor));
            env->CallVoidMethod (bundle, AndroidBundle.putString, javaString ("properties").get(),
                                 javaString (JSON::toString (varToParse, false)).get());

            return bundle;
        */
    }

    /**
       Gets "properties" var from bundle.
      */
    pub fn bundle_with_properties_string_to_var(bundle: &LocalRef<jobject>) -> Var {
        
        todo!();
        /*
            auto* env = getEnv();

            auto varString = LocalRef<jstring> ((jstring)env->CallObjectMethod (bundle, AndroidBundle.getString,
                                                                                javaString ("properties").get()));

            var resultVar;
            JSON::parse (aloeString (varString.get()), resultVar);

            // Note: We are not checking if result of parsing was okay, because there may be no properties set at all.
            return resultVar;
        */
    }

    /**
       Reverse of aloeNotificationToBundle().
      */
    pub fn local_notification_bundle_to_aloe_notification(bundle: &LocalRef<jobject>) -> PushNotifications::PushNotificationsNotification {
        
        todo!();
        /*
            auto* env = getEnv();

            PushNotifications::PushNotificationsNotification n;

            if (bundle.get() != nullptr)
            {
                n.identifier  = getStringFromBundle (env, "identifier", bundle);
                n.title       = getStringFromBundle (env, "title", bundle);
                n.body        = getStringFromBundle (env, "body", bundle);
                n.subtitle    = getStringFromBundle (env, "subtitle", bundle);
                n.badgeNumber = getIntFromBundle    (env, "badgeNumber", bundle);
                n.soundToPlay = Url (getStringFromBundle (env, "soundToPlay", bundle));
                n.properties  = getPropertiesVarFromBundle (env, "properties", bundle);
                n.tickerText  = getStringFromBundle (env, "tickerText", bundle);
                n.icon        = getStringFromBundle (env, "icon", bundle);
                n.channelId   = getStringFromBundle (env, "channelId", bundle);

                PushNotifications::PushNotificationsNotification::Progress progress;
                progress.max           = getIntFromBundle  (env, "progressMax", bundle);
                progress.current       = getIntFromBundle  (env, "progressCurrent", bundle);
                progress.indeterminate = getBoolFromBundle (env, "progressIndeterminate", bundle);
                n.progress = progress;

                n.person       = getStringFromBundle (env, "person", bundle);
                n.type         = (PushNotifications::PushNotificationsNotification::Type)     getIntFromBundle (env, "type", bundle);
                n.priority     = (PushNotifications::PushNotificationsNotification::Priority) getIntFromBundle (env, "priority", bundle);
                n.lockScreenAppearance = (PushNotifications::PushNotificationsNotification::LockScreenAppearance) getIntFromBundle (env, "lockScreenAppearance", bundle);
                n.groupId      = getStringFromBundle (env, "groupId", bundle);
                n.groupSortKey = getStringFromBundle (env, "groupSortKey", bundle);
                n.groupSummary = getBoolFromBundle   (env, "groupSummary", bundle);
                n.accentColour = Colour ((uint32) getIntFromBundle (env, "accentColour", bundle));
                n.ledColour    = Colour ((uint32) getIntFromBundle (env, "ledColour", bundle));

                PushNotifications::PushNotificationsNotification::LedBlinkPattern ledBlinkPattern;
                ledBlinkPattern.msToBeOn  = getIntFromBundle (env, "ledBlinkPatternMsToBeOn", bundle);
                ledBlinkPattern.msToBeOff = getIntFromBundle (env, "ledBlinkPatternMsToBeOff", bundle);
                n.ledBlinkPattern = ledBlinkPattern;

                n.vibrationPattern = getLongArrayFromBundle (env, "vibrationPattern", bundle);

                n.shouldAutoCancel    = getBoolFromBundle (env, "shouldAutoCancel", bundle);
                n.localOnly           = getBoolFromBundle (env, "localOnly", bundle);
                n.ongoing             = getBoolFromBundle (env, "ongoing", bundle);
                n.alertOnlyOnce       = getBoolFromBundle (env, "alertOnlyOnce", bundle);
                n.timestampVisibility = (PushNotifications::PushNotificationsNotification::TimestampVisibility) getIntFromBundle (env, "timestampVisibility", bundle);
                n.badgeIconType       = (PushNotifications::PushNotificationsNotification::BadgeIconType) getIntFromBundle (env, "badgeIconType", bundle);
                n.groupAlertBehaviour = (PushNotifications::PushNotificationsNotification::GroupAlertBehaviour) getIntFromBundle (env, "groupAlertBehaviour", bundle);
                n.timeoutAfterMs      = getLongFromBundle (env, "timeoutAfterMs", bundle);
            }

            return n;
        */
    }
    
    pub fn get_string_from_bundle(
        env:    *mut JNIEnv,
        key:    &String,
        bundle: &LocalRef<jobject>) -> String {
        
        todo!();
        /*
            auto keyString = javaString (key);

            if (env->CallBooleanMethod (bundle, AndroidBundle.containsKey, keyString.get()))
            {
                auto value = LocalRef<jstring> ((jstring)env->CallObjectMethod (bundle, AndroidBundle.getString, keyString.get()));
                return aloeString (value);
            }

            return {};
        */
    }
    
    pub fn get_int_from_bundle(
        env:    *mut JNIEnv,
        key:    &String,
        bundle: &LocalRef<jobject>) -> i32 {
        
        todo!();
        /*
            auto keyString = javaString (key);

            if (env->CallBooleanMethod (bundle, AndroidBundle.containsKey, keyString.get()))
                return env->CallIntMethod (bundle, AndroidBundle.getInt, keyString.get());

            return 0;
        */
    }

    /**
       Converting to int on purpose!
      */
    pub fn get_long_from_bundle(
        env:    *mut JNIEnv,
        key:    &String,
        bundle: &LocalRef<jobject>) -> i32 {
        
        todo!();
        /*
            auto keyString = javaString (key);

            if (env->CallBooleanMethod (bundle, AndroidBundle.containsKey, keyString.get()))
                return (int) env->CallLongMethod (bundle, AndroidBundle.getLong, keyString.get());

            return 0;
        */
    }
    
    pub fn get_properties_var_from_bundle(
        env:    *mut JNIEnv,
        key:    &String,
        bundle: &LocalRef<jobject>) -> Var {
        
        todo!();
        /*
            auto keyString = javaString (key);

            if (env->CallBooleanMethod (bundle, AndroidBundle.containsKey, keyString.get()))
            {
                auto value = LocalRef<jobject> (env->CallObjectMethod (bundle, AndroidBundle.getBundle, keyString.get()));
                return bundleWithPropertiesStringToVar (value);
            }

            return {};
        */
    }
    
    pub fn get_bool_from_bundle(
        env:    *mut JNIEnv,
        key:    &String,
        bundle: &LocalRef<jobject>) -> bool {
        
        todo!();
        /*
            auto keyString = javaString (key);

            if (env->CallBooleanMethod (bundle, AndroidBundle.containsKey, keyString.get()))
                return env->CallBooleanMethod (bundle, AndroidBundle.getBoolean, keyString.get());

            return false;
        */
    }
    
    pub fn get_long_array_from_bundle(
        env:    *mut JNIEnv,
        key:    &String,
        bundle: &LocalRef<jobject>) -> Array<i32> {
        
        todo!();
        /*
            auto keyString = javaString (key);

            if (env->CallBooleanMethod (bundle, AndroidBundle.containsKey, keyString.get()))
            {
                auto array = LocalRef<jlongArray> ((jlongArray) env->CallObjectMethod (bundle, AndroidBundle.getLongArray, keyString.get()));

                const int size = env->GetArrayLength (array.get());

                jlong* elements = env->GetLongArrayElements (array.get(), nullptr);

                Array<int> resultArray;

                for (int i = 0; i < size; ++i)
                    resultArray.add ((int) *elements++);

                return resultArray;
            }

            return {};
        */
    }
    
    pub fn java_notification_to_aloe_notification(notification: &LocalRef<jobject>) -> PushNotifications::PushNotificationsNotification {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 20)
                return {};

            auto* env = getEnv();

            auto extras = LocalRef<jobject> (env->GetObjectField (notification, AndroidNotification.extras));
            auto notificationData = LocalRef<jobject> (env->CallObjectMethod (extras, AndroidBundle.getBundle,
                                                                              javaString ("notificationData").get()));

            if (notificationData.get() != nullptr)
                return localNotificationBundleToAloeNotification (notificationData);

            return remoteNotificationBundleToAloeNotification (extras);
        */
    }
    
    pub fn remote_notification_bundle_to_aloe_notification(bundle: &LocalRef<jobject>) -> PushNotifications::PushNotificationsNotification {
        
        todo!();
        /*
            // This will probably work only for remote notifications that get delivered to system tray
            PushNotifications::PushNotificationsNotification n;
            n.properties = bundleToVar (bundle);

            return n;
        */
    }
    
    pub fn bundle_to_var(bundle: &LocalRef<jobject>) -> Var {
        
        todo!();
        /*
            if (bundle.get() == nullptr)
            {
                auto* env = getEnv();

                auto keySet   = LocalRef<jobject> (env->CallObjectMethod (bundle, AndroidBundle.keySet));
                auto iterator = LocalRef<jobject> (env->CallObjectMethod (keySet, JavaSet.iterator));

                DynamicObject::Ptr dynamicObject = new DynamicObject();

                for (;;)
                {
                    if (! env->CallBooleanMethod (iterator, JavaIterator.hasNext))
                        break;

                    auto key            = LocalRef<jstring> ((jstring) env->CallObjectMethod (iterator, JavaIterator.next));
                    auto object         = LocalRef<jobject> (env->CallObjectMethod (bundle, AndroidBundle.get, key.get()));

                    if (object.get() != nullptr)
                    {
                        auto objectAsString = LocalRef<jstring> ((jstring) env->CallObjectMethod (object, JavaObject.toString));
                        auto objectClass    = LocalRef<jobject> (env->CallObjectMethod (object, JavaObject.getClass));
                        auto classAsString  = LocalRef<jstring> ((jstring) env->CallObjectMethod (objectClass, JavaClass.getName));

                        // Note: It seems that Firebase delivers values as strings always, so this check is rather unnecessary,
                        //       at least until they change the behaviour.
                        var value = aloeString (classAsString) == "java.lang.Bundle" ? bundleToVar (object) : var (aloeString (objectAsString.get()));
                        dynamicObject->setProperty (aloeString (key.get()), value);
                    }
                    else
                    {
                        dynamicObject->setProperty (aloeString (key.get()), {});
                    }
                }

                return var (dynamicObject.get());
            }

            return {};
        */
    }

    #[cfg(ALOE_FIREBASE_MESSAGING_SERVICE_CLASSNAME)]
    pub fn firebase_remote_notification_to_aloe_notification(remote_notification: jobject) -> PushNotifications::PushNotificationsNotification {
        
        todo!();
        /*
            auto* env = getEnv();

            auto collapseKey  = LocalRef<jstring> ((jstring) env->CallObjectMethod (remoteNotification, RemoteMessage.getCollapseKey));
            auto from         = LocalRef<jstring> ((jstring) env->CallObjectMethod (remoteNotification, RemoteMessage.getFrom));
            auto messageId    = LocalRef<jstring> ((jstring) env->CallObjectMethod (remoteNotification, RemoteMessage.getMessageId));
            auto messageType  = LocalRef<jstring> ((jstring) env->CallObjectMethod (remoteNotification, RemoteMessage.getMessageType));
            auto to           = LocalRef<jstring> ((jstring) env->CallObjectMethod (remoteNotification, RemoteMessage.getTo));
            auto notification = LocalRef<jobject> (env->CallObjectMethod (remoteNotification, RemoteMessage.getNotification));
            auto data         = LocalRef<jobject> (env->CallObjectMethod (remoteNotification, RemoteMessage.getData));

            const int64 sentTime = env->CallLongMethod (remoteNotification, RemoteMessage.getSentTime);
            const int ttl        = env->CallIntMethod  (remoteNotification, RemoteMessage.getTtl);

            auto keySet   = LocalRef<jobject> (env->CallObjectMethod (data, JavaMap.keySet));
            auto iterator = LocalRef<jobject> (env->CallObjectMethod (keySet, JavaSet.iterator));

            DynamicObject::Ptr dataDynamicObject = new DynamicObject();

            for (;;)
            {
                if (! env->CallBooleanMethod (iterator, JavaIterator.hasNext))
                    break;

                auto key   = LocalRef<jstring> ((jstring) env->CallObjectMethod (iterator, JavaIterator.next));
                auto value = LocalRef<jstring> ((jstring) env->CallObjectMethod (data, JavaMap.get, key.get()));

                dataDynamicObject->setProperty (aloeString (key.get()), aloeString (value.get()));
            }

            var dataVar (dataDynamicObject.get());

            DynamicObject::Ptr propertiesDynamicObject = new DynamicObject();
            propertiesDynamicObject->setProperty ("collapseKey", aloeString (collapseKey.get()));
            propertiesDynamicObject->setProperty ("from", aloeString (from.get()));
            propertiesDynamicObject->setProperty ("messageId", aloeString (messageId.get()));
            propertiesDynamicObject->setProperty ("messageType", aloeString (messageType.get()));
            propertiesDynamicObject->setProperty ("to", aloeString (to.get()));
            propertiesDynamicObject->setProperty ("sentTime", sentTime);
            propertiesDynamicObject->setProperty ("ttl", ttl);
            propertiesDynamicObject->setProperty ("data", dataVar);

            PushNotifications::PushNotificationsNotification n;

            if (notification != 0)
            {
                auto body                  = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getBody));
                auto bodyLocalizationKey   = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getBodyLocalizationKey));
                auto clickAction           = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getClickAction));
                auto color                 = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getColor));
                auto icon                  = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getIcon));
                auto sound                 = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getSound));
                auto tag                   = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getTag));
                auto title                 = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getTitle));
                auto titleLocalizationKey  = LocalRef<jstring> ((jstring) env->CallObjectMethod (notification, RemoteMessageNotification.getTitleLocalizationKey));
                auto link                  = LocalRef<jobject> (env->CallObjectMethod (notification, RemoteMessageNotification.getLink));

                auto bodyLocalizationArgs  = LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (notification, RemoteMessageNotification.getBodyLocalizationArgs));
                auto titleLocalizationArgs = LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (notification, RemoteMessageNotification.getTitleLocalizationArgs));

                n.identifier = aloeString (tag.get());
                n.title      = aloeString (title.get());
                n.body       = aloeString (body.get());
                n.soundToPlay = Url (aloeString (sound.get()));

                auto colourString = aloeString (color.get()).substring (1);
                const uint8 r = (uint8) colourString.substring (0, 2).getIntValue();
                const uint8 g = (uint8) colourString.substring (2, 4).getIntValue();
                const uint8 b = (uint8) colourString.substring (4, 6).getIntValue();
                n.accentColour = Colour (r, g, b);

                // Note: Ignoring the icon, because Firebase passes it as a string.

                propertiesDynamicObject->setProperty ("clickAction",           aloeString (clickAction.get()));
                propertiesDynamicObject->setProperty ("bodyLocalizationKey",   aloeString (bodyLocalizationKey.get()));
                propertiesDynamicObject->setProperty ("titleLocalizationKey",  aloeString (titleLocalizationKey.get()));
                propertiesDynamicObject->setProperty ("bodyLocalizationArgs",  javaStringArrayToAloe (bodyLocalizationArgs));
                propertiesDynamicObject->setProperty ("titleLocalizationArgs", javaStringArrayToAloe (titleLocalizationArgs));
                propertiesDynamicObject->setProperty ("link",                  link.get() != nullptr ? aloeString ((jstring) env->CallObjectMethod (link, AndroidUri.toString)) : String());
            }

            n.properties = var (propertiesDynamicObject.get());

            return n;
        */
    }
    
    pub fn setup_channels(&mut self, 
        groups:   &[ChannelGroup],
        channels: &[Channel])  {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 26)
                return;

            auto* env = getEnv();

            auto notificationManager = getNotificationManager();

            jassert (notificationManager.get() != nullptr);

            if (notificationManager.get() == nullptr)
                return;

            for (const auto& g : groups)
            {
                // Channel group identifier and name have to be set.
                jassert (g.identifier.isNotEmpty() && g.name.isNotEmpty());

                if (g.identifier.isNotEmpty() && g.name.isNotEmpty())
                {
                    auto group = LocalRef<jobject> (env->NewObject (NotificationChannelGroup, NotificationChannelGroup.constructor,
                                                                    javaString (g.identifier).get(), javaString (g.name).get()));
                    env->CallVoidMethod (notificationManager, NotificationManagerApi26.createNotificationChannelGroup, group.get());
                }
            }

            for (const auto& c : channels)
            {
                // Channel identifier, name and group have to be set.
                jassert (c.identifier.isNotEmpty() && c.name.isNotEmpty() && c.groupId.isNotEmpty());

                if (c.identifier.isEmpty() || c.name.isEmpty() || c.groupId.isEmpty())
                    continue;

                auto channel = LocalRef<jobject> (env->NewObject (NotificationChannel, NotificationChannel.constructor,
                                                                  javaString (c.identifier).get(), javaString (c.name).get(), c.importance));

                env->CallVoidMethod (channel, NotificationChannel.enableLights,            c.enableLights);
                env->CallVoidMethod (channel, NotificationChannel.enableVibration,         c.enableVibration);
                env->CallVoidMethod (channel, NotificationChannel.setBypassDnd,            c.bypassDoNotDisturb);
                env->CallVoidMethod (channel, NotificationChannel.setDescription,          javaString (c.description).get());
                env->CallVoidMethod (channel, NotificationChannel.setGroup,                javaString (c.groupId).get());
                env->CallVoidMethod (channel, NotificationChannel.setImportance,           c.importance);
                env->CallVoidMethod (channel, NotificationChannel.setLightColor,           c.ledColour.getARGB());
                env->CallVoidMethod (channel, NotificationChannel.setLockscreenVisibility, c.lockScreenAppearance);
                env->CallVoidMethod (channel, NotificationChannel.setShowBadge,            c.canShowBadge);

                const int size = c.vibrationPattern.size();

                if (size > 0)
                {
                    auto array = LocalRef<jlongArray> (env->NewLongArray (size));
                    jlong* elements = env->GetLongArrayElements (array, nullptr);

                    for (int i = 0; i < size; ++i)
                        elements[i] = (jlong) c.vibrationPattern[i];

                    env->SetLongArrayRegion (array, 0, size, elements);
                    env->CallVoidMethod (channel, NotificationChannel.setVibrationPattern, array.get());

                    env->CallVoidMethod (channel, NotificationChannel.enableVibration, c.enableVibration);
                }

                LocalRef<jobject> builder (env->NewObject (AndroidAudioAttributesBuilder, AndroidAudioAttributesBuilder.constructor));
                const int contentTypeSonification = 4;
                const int usageNotification = 5;
                env->CallObjectMethod (builder.get(), AndroidAudioAttributesBuilder.setContentType, contentTypeSonification);
                env->CallObjectMethod (builder.get(), AndroidAudioAttributesBuilder.setUsage, usageNotification);
                auto audioAttributes = LocalRef<jobject> (env->CallObjectMethod (builder.get(), AndroidAudioAttributesBuilder.build));
                env->CallVoidMethod (channel, NotificationChannel.setSound, aloeUrlToAndroidUri (c.soundToPlay).get(), audioAttributes.get());

                env->CallVoidMethod (notificationManager, NotificationManagerApi26.createNotificationChannel, channel.get());
            }
        */
    }
    
    pub fn get_pending_local_notifications(&self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn remove_pending_local_notification(&mut self, _0: &String)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn remove_all_pending_local_notifications(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn intent_action_contains_any_of(
        intent:               jobject,
        strings:              &StringArray,
        include_package_name: bool) -> bool {
        
        todo!();
        /*
            auto* env = getEnv();
            LocalRef<jobject> context (getMainActivity());

            String packageName = includePackageName ? aloeString ((jstring) env->CallObjectMethod (context.get(),
                                                                                                   AndroidContext.getPackageName))
                                                    : String{};

            String intentAction = aloeString ((jstring) env->CallObjectMethod (intent, AndroidIntent.getAction));

            for (const auto& string : strings)
                if (intentAction.contains (packageName + string))
                    return true;

            return false;
        */
    }
    
    pub fn is_delete_notification_intent(intent: jobject) -> bool {
        
        todo!();
        /*
            return intentActionContainsAnyOf (intent, StringArray (".ALOE_NOTIFICATION_DELETED"), true);
        */
    }
    
    pub fn is_local_notification_intent(intent: jobject) -> bool {
        
        todo!();
        /*
            return intentActionContainsAnyOf (intent, { ".ALOE_NOTIFICATION.",
                                                        ".ALOE_NOTIFICATION_BUTTON_ACTION.",
                                                        ".ALOE_NOTIFICATION_TEXT_INPUT_ACTION." },
                                              true);
        */
    }
    
    pub fn is_remote_notification_intent(intent: jobject) -> bool {
        
        todo!();
        /*
            auto* env = getEnv();

            auto categories = LocalRef<jobject> (env->CallObjectMethod (intent, AndroidIntent.getCategories));

            int categoriesNum = categories != nullptr
                              ? env->CallIntMethod (categories, JavaSet.size)
                              : 0;

            if (categoriesNum == 0)
                return false;

            if (! env->CallBooleanMethod (categories, JavaSet.contains, javaString ("android.intent.category.LAUNCHER").get()))
                return false;

            if (! intentActionContainsAnyOf (intent, StringArray ("android.intent.action.MAIN"), false))
                return false;

            auto extras = LocalRef<jobject> (env->CallObjectMethod (intent, AndroidIntent.getExtras));

            if (extras == nullptr)
                return false;

            return env->CallBooleanMethod (extras, AndroidBundle.containsKey, javaString ("google.sent_time").get())
                && env->CallBooleanMethod (extras, AndroidBundle.containsKey, javaString ("google.message_id").get());
        */
    }
}

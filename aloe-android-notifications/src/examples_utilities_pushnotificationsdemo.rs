crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Utilities/PushNotificationsDemo.h]

/**
  | To finish the setup of this demo, do the
  | following:
  | 
  | 1. Download google_services.json
  | from your Firebase project.
  | 
  | 2. Update "Remote Notifications Config
  | File" path in Android exporter (this
  | can be different for debug and release)
  | to point to that json file.
  | 
  | 3. Add image and sound resources by adding
  | the following to "Extra Android Raw
  | Resources" in Proaloer:
  | 
  | ../../Assets/Notifications/images/ic_stat_name.png
  | ../../Assets/Notifications/images/ic_stat_name2.png
  | ../../Assets/Notifications/images/ic_stat_name3.png
  | ../../Assets/Notifications/images/ic_stat_name4.png
  | ../../Assets/Notifications/images/ic_stat_name5.png
  | ../../Assets/Notifications/images/ic_stat_name6.png
  | ../../Assets/Notifications/images/ic_stat_name7.png
  | ../../Assets/Notifications/images/ic_stat_name8.png
  | ../../Assets/Notifications/images/ic_stat_name9.png
  | ../../Assets/Notifications/images/ic_stat_name10.png
  | ../../Assets/Notifications/sounds/demonstrative.mp3
  | ../../Assets/Notifications/sounds/isntit.mp3
  | ../../Assets/Notifications/sounds/jinglebellssms.mp3
  | ../../Assets/Notifications/sounds/served.mp3
  | ../../Assets/Notifications/sounds/solemn.mp3
  | 
  | 4. Set "Remote Notifications" to enabled
  | in Proaloer Android exporter.
  | 
  | To verify that remote notifications
  | are configured properly, go to Remote
  | tab in the demo and press "GetDeviceToken"
  | button, a dialog with your token (also
  | printed to console in debug build) should
  | show up.
  | 
  | The following steps are only necessary
  | if you have a custom activity defined:
  | 
  | 5. Ensure that its launchMode is set
  | to "singleTop" or "singleTask" in Android
  | manifest. This is the default behaviour
  | in Aloe so you only need to do it if you
  | have custom Android manifest content.
  | You can do it from Proaloer by ensuring
  | that "Custom Manifest XML Content"
  | contains:
  | 
  | -----------
  | @code
  | 
  | <manifest>
  | <application>
  | <activity android:launchMode="singleTask">
  | </activity>
  | </application>
  | </manifest>
  | 
  | 6. Ensure that you override onNewIntent()
  | function in the same way as it is done
  | in AloeActivity.java:
  | ----------
  | @code
  | 
  | package com.rmsl.aloe;
  | 
  | import android.app.Activity;
  | import android.content.Intent;
  | 
  | 
  | public class AloeActivity   extends Activity
  | {
  |     
  |     private native void appNewIntent (Intent intent);
  | 
  |     @Override
  |     protected void onNewIntent (Intent intent)
  |     {
  |         super.onNewIntent(intent);
  |         setIntent(intent);
  | 
  |         appNewIntent (intent);
  |     }
  | }
  |
  */
#[no_copy]
#[leak_detector]
#[cfg(target_os="android")]
pub struct PushNotificationsDemo<'a> {
    base:                     Component<'a>,
    base2:                    ChangeListener,
    base3:                    ComponentListener,
    base4:                    PushNotifications::Listener,
    header_label:             push_notifications_demo::Label,                // default = { "headerLabel", "Push Notifications Demo"  }
    param_controls:           push_notifications_demo::PushNotificationsDemoParamControls,
    params_one_view:          push_notifications_demo::PushNotificationsDemoParamsView<'a>,
    params_two_view:          push_notifications_demo::PushNotificationsDemoParamsView<'a>,
    params_three_view:        push_notifications_demo::PushNotificationsDemoParamsView<'a>,
    params_four_view:         push_notifications_demo::PushNotificationsDemoParamsView<'a>,
    aux_actions_view:         push_notifications_demo::PushNotificationsDemoAuxActionsView<'a>,
    local_notifications_tabs: push_notifications_demo::TabbedComponent,      // default = { TabbedButtonBar::TabsAtTop  }
    remote_view:              push_notifications_demo::PushNotificationsDemoRemoteView<'a>,
    main_tabs:                push_notifications_demo::PushNotificationsDemoTabbedComponent,  // default = { TabbedButtonBar::TabsAtTop  }
    send_button:              push_notifications_demo::TextButton,           // default = { "Send!"  }
    not_available_yet_label:  push_notifications_demo::Label,                // default = { "notAvailableYetLabel", "Push Notifications feature is not available on this platform yet!"  }
}

#[cfg(target_os="android")]
impl<'a> Default for PushNotificationsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setupControls();
            distributeControls();

          #if ALOE_PUSH_NOTIFICATIONS
            addAndMakeVisible (headerLabel);
            addAndMakeVisible (mainTabs);
            addAndMakeVisible (sendButton);
          #else
            addAndMakeVisible (notAvailableYetLabel);
          #endif

            headerLabel         .setJustificationType (Justification::centred);
            notAvailableYetLabel.setJustificationType (Justification::centred);

          #if ALOE_MAC
            StringArray tabNames { "Params1", "Params2", "Params3", "Params4" };
          #else
            StringArray tabNames { "Req. params", "Opt. params1", "Opt. params2", "Opt. params3" };
          #endif

            auto colour = getLookAndFeel().findColour (ResizableWindow::backgroundColourId);
            localNotificationsTabs.addTab (tabNames[0], colour, &paramsOneView, false);
            localNotificationsTabs.addTab (tabNames[1], colour, &paramsTwoView, false);
          #if ALOE_ANDROID
            localNotificationsTabs.addTab (tabNames[2], colour, &paramsThreeView, false);
            localNotificationsTabs.addTab (tabNames[3], colour, &paramsFourView,  false);
          #endif
            localNotificationsTabs.addTab ("Aux. actions", colour, &auxActionsView, false);

            mainTabs.addTab ("Local",  colour, &localNotificationsTabs, false);
            mainTabs.addTab ("Remote", colour, &remoteView,             false);

            auto userArea = Desktop::getInstance().getDisplays().getPrimaryDisplay()->userArea;
          #if ALOE_ANDROID || ALOE_IOS
            setSize (userArea.getWidth(), userArea.getHeight());
          #else
            setSize (userArea.getWidth() / 2, userArea.getHeight() / 2);
          #endif

            sendButton.onClick = [this] { sendLocalNotification(); };
            auxActionsView.getDeliveredNotificationsButton .onClick = []
                { PushNotifications::getInstance()->getDeliveredNotifications(); };
            auxActionsView.removeDeliveredNotifWithIdButton.onClick = [this]
                { PushNotifications::getInstance()->removeDeliveredNotification (auxActionsView.deliveredNotifIdentifier.getText()); };
            auxActionsView.removeAllDeliveredNotifsButton  .onClick = []
                { PushNotifications::getInstance()->removeAllDeliveredNotifications(); };
          #if ALOE_IOS || ALOE_MAC
            auxActionsView.getPendingNotificationsButton .onClick = []
                { PushNotifications::getInstance()->getPendingLocalNotifications(); };
            auxActionsView.removePendingNotifWithIdButton.onClick = [this]
                { PushNotifications::getInstance()->removePendingLocalNotification (auxActionsView.pendingNotifIdentifier.getText()); };
            auxActionsView.removeAllPendingNotifsButton  .onClick = []
                { PushNotifications::getInstance()->removeAllPendingLocalNotifications(); };
          #endif

            remoteView.getDeviceTokenButton.onClick = []
            {
                String token = PushNotifications::getInstance()->getDeviceToken();

                DBG ("token = " + token);

                if (token.isEmpty())
                    showRemoteInstructions();
                else
                    NativeMessageBox::showAsync (MessageBoxOptions()
                                                   .withIconType (MessageBoxIconType::InfoIcon)
                                                   .withTitle ("Device token")
                                                   .withMessage (token),
                                                 nullptr);
            };

          #if ALOE_ANDROID
            remoteView.sendRemoteMessageButton.onClick = []
            {
                StringPairArray data;
                data.set ("key1", "value1");
                data.set ("key2", "value2");

                static int id = 100;
                PushNotifications::getInstance()->sendUpstreamMessage ("872047750958",
                                                                       "com.aloe.pushnotificationsdemo",
                                                                       String (id++),
                                                                       "standardType",
                                                                       3600,
                                                                       data);
            };

            remoteView.subscribeToSportsButton    .onClick = []
                { PushNotifications::getInstance()->subscribeToTopic ("sports"); };
            remoteView.unsubscribeFromSportsButton.onClick = []
                { PushNotifications::getInstance()->unsubscribeFromTopic ("sports"); };
          #endif

            paramControls.accentColourButton.onClick = [this] { setupAccentColour(); };
            paramControls.ledColourButton   .onClick = [this] { setupLedColour(); };

            jassert (PushNotifications::getInstance()->areNotificationsEnabled());

            PushNotifications::getInstance()->addListener (this);

          #if ALOE_IOS || ALOE_MAC
            paramControls.fireInComboBox.onChange = [this] { delayNotification(); };
            PushNotifications::getInstance()->requestPermissionsWithSettings (getNotificationSettings());
          #elif ALOE_ANDROID
            PushNotifications::ChannelGroup cg { "demoGroup", "demo group" };
            PushNotifications::getInstance()->setupChannels ({ { cg } }, getAndroidChannels());
          #endif

           #if ALOE_IOS || ALOE_ANDROID
            setPortraitOrientationEnabled (true);
           #endi
        */
    }
}

#[cfg(target_os="android")]
impl<'a> Drop for PushNotificationsDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            PushNotifications::getInstance()->removeListener (this);

           #if ALOE_IOS || ALOE_ANDROID
            setPortraitOrientationEnabled (false);
           #endif
         */
    }
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsDemo<'a> {
    
    pub fn set_portrait_orientation_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            auto allowedOrientations = Desktop::getInstance().getOrientationsEnabled();

            if (shouldBeEnabled)
                allowedOrientations |= Desktop::upright;
            else
                allowedOrientations &= ~Desktop::upright;

            Desktop::getInstance().setOrientationsEnabled (allowedOrientations);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getLookAndFeel().findColour (ResizableWindow::backgroundColourId));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds().reduced (getWidth() / 20, getHeight() / 40);

            headerLabel.setBounds (bounds.removeFromTop (bounds.proportionOfHeight (0.1f)));
            mainTabs   .setBounds (bounds.removeFromTop (bounds.proportionOfHeight (0.8f)));
            sendButton .setBounds (bounds);

            notAvailableYetLabel.setBounds (getLocalBounds());
        */
    }
    
    pub fn delay_notification(&mut self)  {
        
        todo!();
        /*
            auto repeatsAllowed = paramControls.fireInComboBox.getSelectedItemIndex() >= 6;

            paramControls.repeatButton.setEnabled (repeatsAllowed);

            if (! repeatsAllowed)
                paramControls.repeatButton.setToggleState (false, NotificationType::sendNotification);
        */
    }
    
    pub fn send_local_notification(&mut self)  {
        
        todo!();
        /*
            PushNotifications::Notification n;

            fillRequiredParams (n);
            fillOptionalParamsOne (n);
          #if ALOE_ANDROID
            fillOptionalParamsTwo (n);
            fillOptionalParamsThree (n);
          #endif

            if (! n.isValid())
            {
              #if ALOE_IOS
                String requiredFields = "identifier (from iOS 10), title, body and category";
              #elif ALOE_ANDROID
                String requiredFields = "channel ID (from Android O), title, body and icon";
              #else
                String requiredFields = "all required fields";
              #endif

                NativeMessageBox::showAsync (MessageBoxOptions()
                                               .withIconType (MessageBoxIconType::InfoIcon)
                                               .withTitle ("Incorrect notifications setup")
                                               .withMessage ("Please make sure that " + requiredFields + " are set."),
                                             nullptr);

                return;
            }

            PushNotifications::getInstance()->sendLocalNotification (n);
        */
    }
    
    pub fn fill_required_params(&mut self, n: &mut PushNotifications::Notification)  {
        
        todo!();
        /*
            n.identifier = paramControls.identifierEditor.getText();
            n.title      = paramControls.titleEditor     .getText();
            n.body       = paramControls.bodyEditor      .getText();
          #if ALOE_IOS
            n.category = paramControls.categoryComboBox.getText();
          #elif ALOE_ANDROID || ALOE_MAC
           #if ALOE_MAC
            String prefix = "Notifications/images/";
            String extension = ".png";
           #else
            String prefix;
            String extension;
           #endif
            if (paramControls.iconComboBox.getSelectedItemIndex() == 0)
                n.icon = prefix + "ic_stat_name" + extension;
            else if (paramControls.iconComboBox.getSelectedItemIndex() == 1)
                n.icon = prefix + "ic_stat_name2" + extension;
            else if (paramControls.iconComboBox.getSelectedItemIndex() == 2)
                n.icon = prefix + "ic_stat_name3" + extension;
            else if (paramControls.iconComboBox.getSelectedItemIndex() == 3)
                n.icon = prefix + "ic_stat_name4" + extension;
            else if (paramControls.iconComboBox.getSelectedItemIndex() == 4)
                n.icon = prefix + "ic_stat_name5" + extension;
          #endif

          #if ALOE_ANDROID
            // Note: this is not strictly speaking required param, just doing it here because it is the fastest way!
            n.publicVersion.reset (new PushNotifications::Notification());
            n.publicVersion->identifier = "blahblahblah";
            n.publicVersion->title      = "Public title!";
            n.publicVersion->body       = "Public body!";
            n.publicVersion->icon       = n.icon;

            n.channelId = String (paramControls.channelIdComboBox.getSelectedItemIndex() + 1);
          #endif
        */
    }
    
    pub fn fill_optional_params_one(&mut self, n: &mut PushNotifications::Notification)  {
        
        todo!();
        /*
            n.subtitle = paramControls.subtitleEditor.getText();
            n.badgeNumber = paramControls.badgeNumberComboBox.getSelectedItemIndex();

            if (paramControls.soundToPlayComboBox.getSelectedItemIndex() > 0)
                n.soundToPlay = Url (paramControls.soundToPlayComboBox.getItemText (paramControls.soundToPlayComboBox.getSelectedItemIndex()));

            n.properties = JSON::parse (paramControls.propertiesEditor.getText());

          #if ALOE_IOS || ALOE_MAC
            n.triggerIntervalSec = double (paramControls.fireInComboBox.getSelectedItemIndex() * 10);
            n.repeat = paramControls.repeatButton.getToggleState();
          #elif ALOE_ANDROID
            if (paramControls.largeIconComboBox.getSelectedItemIndex() == 1)
                n.largeIcon = getImageFromAssets ("Notifications/images/ic_stat_name6.png");
            else if (paramControls.largeIconComboBox.getSelectedItemIndex() == 2)
                n.largeIcon = getImageFromAssets ("Notifications/images/ic_stat_name7.png");
            else if (paramControls.largeIconComboBox.getSelectedItemIndex() == 3)
                n.largeIcon = getImageFromAssets ("Notifications/images/ic_stat_name8.png");
            else if (paramControls.largeIconComboBox.getSelectedItemIndex() == 4)
                n.largeIcon = getImageFromAssets ("Notifications/images/ic_stat_name9.png");
            else if (paramControls.largeIconComboBox.getSelectedItemIndex() == 5)
                n.largeIcon = getImageFromAssets ("Notifications/images/ic_stat_name10.png");

            n.badgeIconType = (PushNotifications::Notification::BadgeIconType) paramControls.badgeIconComboBox.getSelectedItemIndex();
            n.tickerText  = paramControls.tickerTextEditor.getText();

            n.shouldAutoCancel = paramControls.autoCancelButton   .getToggleState();
            n.alertOnlyOnce    = paramControls.alertOnlyOnceButton.getToggleState();
          #endif

          #if ALOE_ANDROID || ALOE_MAC
            if (paramControls.actionsComboBox.getSelectedItemIndex() == 1)
            {
                PushNotifications::Notification::Action a, a2;
                a .style = PushNotifications::Notification::Action::button;
                a2.style = PushNotifications::Notification::Action::button;
                a .title = a .identifier = "Ok";
                a2.title = a2.identifier = "Cancel";
                n.actions.add (a);
                n.actions.add (a2);
            }
            else if (paramControls.actionsComboBox.getSelectedItemIndex() == 2)
            {
                PushNotifications::Notification::Action a, a2;
                a .title = a .identifier = "Input Text Here";
                a2.title = a2.identifier = "No";
                a .style = PushNotifications::Notification::Action::text;
                a2.style = PushNotifications::Notification::Action::button;
                a .icon = "ic_stat_name4";
                a2.icon = "ic_stat_name5";
                a.textInputPlaceholder = "placeholder text ...";
                n.actions.add (a);
                n.actions.add (a2);
            }
            else if (paramControls.actionsComboBox.getSelectedItemIndex() == 3)
            {
                PushNotifications::Notification::Action a, a2;
                a .title = a .identifier = "Ok";
                a2.title = a2.identifier = "Cancel";
                a .style = PushNotifications::Notification::Action::button;
                a2.style = PushNotifications::Notification::Action::button;
                a .icon = "ic_stat_name4";
                a2.icon = "ic_stat_name5";
                n.actions.add (a);
                n.actions.add (a2);
            }
            else if (paramControls.actionsComboBox.getSelectedItemIndex() == 4)
            {
                PushNotifications::Notification::Action a, a2;
                a .title = a .identifier = "Input Text Here";
                a2.title = a2.identifier = "No";
                a .style = PushNotifications::Notification::Action::text;
                a2.style = PushNotifications::Notification::Action::button;
                a .icon = "ic_stat_name4";
                a2.icon = "ic_stat_name5";
                a.textInputPlaceholder = "placeholder text ...";
                a.allowedResponses.add ("Response 1");
                a.allowedResponses.add ("Response 2");
                a.allowedResponses.add ("Response 3");
                n.actions.add (a);
                n.actions.add (a2);
            }
          #endif
        */
    }
    
    pub fn fill_optional_params_two(&mut self, n: &mut PushNotifications::Notification)  {
        
        todo!();
        /*
            using Notification = PushNotifications::Notification;

            Notification::Progress progress;
            progress.max           = paramControls.progressMaxComboBox    .getSelectedItemIndex() * 10;
            progress.current       = paramControls.progressCurrentComboBox.getSelectedItemIndex() * 10;
            progress.indeterminate = paramControls.progressIndeterminateButton.getToggleState();

            n.progress = progress;
            n.person   = paramControls.personEditor.getText();
            n.type                 = Notification::Type                 (paramControls.categoryComboBox            .getSelectedItemIndex());
            n.priority             = Notification::Priority             (paramControls.priorityComboBox            .getSelectedItemIndex() - 2);
            n.lockScreenAppearance = Notification::LockScreenAppearance (paramControls.lockScreenVisibilityComboBox.getSelectedItemIndex() - 1);
            n.groupId      = paramControls.groupIdEditor.getText();
            n.groupSortKey = paramControls.sortKeyEditor.getText();
            n.groupSummary = paramControls.groupSummaryButton.getToggleState();
            n.groupAlertBehaviour = Notification::GroupAlertBehaviour (paramControls.groupAlertBehaviourComboBox.getSelectedItemIndex());
        */
    }
    
    pub fn fill_optional_params_three(&mut self, n: &mut PushNotifications::Notification)  {
        
        todo!();
        /*
            n.accentColour = paramControls.accentColourButton.findColour (TextButton::buttonColourId, false);
            n.ledColour    = paramControls.ledColourButton   .findColour (TextButton::buttonColourId, false);

            using Notification = PushNotifications::Notification;
            Notification::LedBlinkPattern ledBlinkPattern;
            ledBlinkPattern.msToBeOn  = paramControls.ledMsToBeOnComboBox .getSelectedItemIndex() * 200;
            ledBlinkPattern.msToBeOff = paramControls.ledMsToBeOffComboBox.getSelectedItemIndex() * 200;
            n.ledBlinkPattern = ledBlinkPattern;

            Vec<int> vibrationPattern;

            if (paramControls.vibratorMsToBeOnComboBox .getSelectedItemIndex() > 0 &&
                paramControls.vibratorMsToBeOffComboBox.getSelectedItemIndex() > 0)
            {
                vibrationPattern.add     (paramControls.vibratorMsToBeOffComboBox.getSelectedItemIndex() * 500);
                vibrationPattern.add     (paramControls.vibratorMsToBeOnComboBox .getSelectedItemIndex() * 500);
                vibrationPattern.add (2 * paramControls.vibratorMsToBeOffComboBox.getSelectedItemIndex() * 500);
                vibrationPattern.add (2 * paramControls.vibratorMsToBeOnComboBox .getSelectedItemIndex() * 500);
            }

            n.vibrationPattern = vibrationPattern;

            n.localOnly = paramControls.localOnlyButton.getToggleState();
            n.ongoing = paramControls.ongoingButton.getToggleState();
            n.timestampVisibility = Notification::TimestampVisibility (paramControls.timestampVisibilityComboBox.getSelectedItemIndex());

            if (paramControls.timeoutAfterComboBox.getSelectedItemIndex() > 0)
            {
                auto index = paramControls.timeoutAfterComboBox.getSelectedItemIndex();
                n.timeoutAfterMs = index * 1000 + 4000;
            }
        */
    }
    
    pub fn setup_accent_colour(&mut self)  {
        
        todo!();
        /*
            auto accentColourSelector = std::make_unique<ColourSelector>();

            accentColourSelector->setName ("accent colour");
            accentColourSelector->setCurrentColour (paramControls.accentColourButton.findColour (TextButton::buttonColourId));
            accentColourSelector->setColour (ColourSelector::backgroundColourId, Colours::transparentBlack);
            accentColourSelector->setSize (200, 200);
            accentColourSelector->addComponentListener (this);
            accentColourSelector->addChangeListener (this);

            paramControls.accentColourSelector = accentColourSelector.get();

            CallOutBox::launchAsynchronously (std::move (accentColourSelector), paramControls.accentColourButton.getScreenBounds(), nullptr);
        */
    }
    
    pub fn setup_led_colour(&mut self)  {
        
        todo!();
        /*
            auto ledColourSelector = std::make_unique<ColourSelector>();

            ledColourSelector->setName ("led colour");
            ledColourSelector->setCurrentColour (paramControls.ledColourButton.findColour (TextButton::buttonColourId));
            ledColourSelector->setColour (ColourSelector::backgroundColourId, Colours::transparentBlack);
            ledColourSelector->setSize (200, 200);
            ledColourSelector->addComponentListener (this);
            ledColourSelector->addChangeListener (this);

            paramControls.ledColourSelector = ledColourSelector.get();

            CallOutBox::launchAsynchronously (std::move (ledColourSelector), paramControls.accentColourButton.getScreenBounds(), nullptr);
        */
    }
    
    pub fn change_listener_callback(&mut self, source: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            if (source == paramControls.accentColourSelector)
            {
                auto c = paramControls.accentColourSelector->getCurrentColour();
                paramControls.accentColourButton.setColour (TextButton::buttonColourId, c);
            }
            else if (source == paramControls.ledColourSelector)
            {
                auto c = paramControls.ledColourSelector->getCurrentColour();
                paramControls.ledColourButton.setColour (TextButton::buttonColourId, c);
            }
        */
    }
    
    pub fn component_being_deleted(&mut self, component: &mut Component<'a>)  {
        
        todo!();
        /*
            if (&component == paramControls.accentColourSelector)
                paramControls.accentColourSelector = nullptr;
            else if (&component == paramControls.ledColourSelector)
                paramControls.ledColourSelector = nullptr;
        */
    }
    
    pub fn handle_notification(&mut self, 
        is_local_notification: bool,
        n:                     &PushNotifications::Notification)  {
        
        todo!();
        /*
            ignoreUnused (isLocalNotification);

            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Received notification")
                                           .withMessage ("ID: " + n.identifier
                                                         + ", title: " + n.title
                                                         + ", body: " + n.body),
                                         nullptr);
        */
    }
    
    pub fn handle_notification_action(&mut self, 
        is_local_notification: bool,
        n:                     &PushNotifications::Notification,
        action_identifier:     &String,
        optional_response:     &String)  {
        
        todo!();
        /*
            ignoreUnused (isLocalNotification);

            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Received notification action")
                                           .withMessage ("ID: " + n.identifier
                                                         + ", title: " + n.title
                                                         + ", body: " + n.body
                                                         + ", action: " + actionIdentifier
                                                         + ", optionalResponse: " + optionalResponse),
                                         nullptr);

            PushNotifications::getInstance()->removeDeliveredNotification (n.identifier);
        */
    }
    
    pub fn local_notification_dismissed_by_user(&mut self, n: &PushNotifications::Notification)  {
        
        todo!();
        /*
            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Notification dismissed by a user")
                                           .withMessage ("ID: " + n.identifier
                                                         + ", title: " + n.title
                                                         + ", body: " + n.body),
                                         nullptr);
        */
    }
    
    pub fn delivered_notifications_list_received(&mut self, notifs: &[PushNotifications::Notification])  {
        
        todo!();
        /*
            String text = "Received notifications: ";

            for (auto& n : notifs)
                text << "(" << n.identifier << ", " << n.title << ", " << n.body << "), ";

            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Received notification list")
                                           .withMessage (text),
                                         nullptr);
        */
    }
    
    pub fn pending_local_notifications_list_received(&mut self, notifs: &[PushNotifications::Notification])  {
        
        todo!();
        /*
            String text = "Pending notifications: ";

            for (auto& n : notifs)
                text << "(" << n.identifier << ", " << n.title << ", " << n.body << "), ";

            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Pending notification list")
                                           .withMessage (text),
                                         nullptr);
        */
    }
    
    pub fn device_token_refreshed(&mut self, token: &String)  {
        
        todo!();
        /*
            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Device token refreshed")
                                           .withMessage (token),
                                         nullptr);
        */
    }

    #[cfg(target_os="android")]
    pub fn remote_notifications_deleted(&mut self)  {
        
        todo!();
        /*
            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Remote notifications deleted")
                                           .withMessage ("Some of the pending messages were removed!"),
                                         nullptr);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn upstream_message_sent(&mut self, message_id: &String)  {
        
        todo!();
        /*
            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Upstream message sent")
                                           .withMessage ("Message id: " + messageId),
                                         nullptr);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn upstream_message_sending_error(&mut self, 
        message_id: &String,
        error:      &String)  {
        
        todo!();
        /*
            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Upstream message sending error")
                                           .withMessage ("Message id: " + messageId
                                                         + "\nerror: " + error),
                                         nullptr);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_android_channels() -> Vec<PushNotifications::Channel> {
        
        todo!();
        /*
            using Channel = PushNotifications::Channel;

            Channel ch1, ch2, ch3;

            ch1.identifier = "1";
            ch1.name = "HighImportance";
            ch1.importance = PushNotifications::Channel::max;
            ch1.lockScreenAppearance = PushNotifications::Notification::showCompletely;
            ch1.description = "High Priority Channel for important stuff";
            ch1.groupId = "demoGroup";
            ch1.ledColour = Colours::red;
            ch1.bypassDoNotDisturb = true;
            ch1.canShowBadge = true;
            ch1.enableLights = true;
            ch1.enableVibration = true;
            ch1.soundToPlay = Url ("demonstrative");
            ch1.vibrationPattern = { 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200 };

            ch2.identifier = "2";
            ch2.name = "MediumImportance";
            ch2.importance = PushNotifications::Channel::normal;
            ch2.lockScreenAppearance = PushNotifications::Notification::showPartially;
            ch2.description = "Medium Priority Channel for standard stuff";
            ch2.groupId = "demoGroup";
            ch2.ledColour = Colours::yellow;
            ch2.canShowBadge = true;
            ch2.enableLights = true;
            ch2.enableVibration = true;
            ch2.soundToPlay = Url ("default_os_sound");
            ch2.vibrationPattern = { 1000, 1000 };

            ch3.identifier = "3";
            ch3.name = "LowImportance";
            ch3.importance = PushNotifications::Channel::min;
            ch3.lockScreenAppearance = PushNotifications::Notification::dontShow;
            ch3.description = "Low Priority Channel for silly stuff";
            ch3.groupId = "demoGroup";

            return { ch1, ch2, ch3 };
        */
    }

    #[cfg(any(target_os="ios",target_os="macos"))]
    pub fn get_notification_settings() -> PushNotifications::Settings {
        
        todo!();
        /*
            PushNotifications::Settings settings;
            settings.allowAlert = true;
            settings.allowBadge = true;
            settings.allowSound = true;

          #if ALOE_IOS
            using Action   = PushNotifications::Settings::Action;
            using Category = PushNotifications::Settings::Category;

            Action okAction;
            okAction.identifier = "okAction";
            okAction.title = "OK!";
            okAction.style = Action::button;
            okAction.triggerInBackground = true;

            Action cancelAction;
            cancelAction.identifier = "cancelAction";
            cancelAction.title = "Cancel";
            cancelAction.style = Action::button;
            cancelAction.triggerInBackground = true;
            cancelAction.destructive = true;

            Action textAction;
            textAction.identifier = "textAction";
            textAction.title = "Enter text";
            textAction.style = Action::text;
            textAction.triggerInBackground = true;
            textAction.destructive = false;
            textAction.textInputButtonText = "Ok";
            textAction.textInputPlaceholder = "Enter text...";

            Category okCategory;
            okCategory.identifier = "okCategory";
            okCategory.actions = { okAction };

            Category okCancelCategory;
            okCancelCategory.identifier = "okCancelCategory";
            okCancelCategory.actions = { okAction, cancelAction };

            Category textCategory;
            textCategory.identifier = "textCategory";
            textCategory.actions = { textAction };
            textCategory.sendDismissAction = true;

            settings.categories = { okCategory, okCancelCategory, textCategory };
          #endif

            return settings;
        */
    }
    
    pub fn setup_controls(&mut self)  {
        
        todo!();
        /*
            auto& pc = paramControls;

            StringArray categories { "okCategory", "okCancelCategory", "textCategory" };

            for (auto& c : categories)
                pc.categoryComboBox.addItem (c, pc.categoryComboBox.getNumItems() + 1);
            pc.categoryComboBox.setSelectedItemIndex (0);

            for (auto i = 1; i <= 3; ++i)
                pc.channelIdComboBox.addItem (String (i), i);
            pc.channelIdComboBox.setSelectedItemIndex (0);

            for (auto i = 0; i < 5; ++i)
                pc.iconComboBox.addItem ("icon" + String (i + 1), i + 1);
            pc.iconComboBox.setSelectedItemIndex (0);

          #if ALOE_MAC
            pc.iconComboBox.addItem ("none", 100);
          #endif

            pc.fireInComboBox.addItem ("Now", 1);

            for (auto i = 1; i < 11; ++i)
                pc.fireInComboBox.addItem (String (10 * i) + "seconds", i + 1);
            pc.fireInComboBox.setSelectedItemIndex (0);

            pc.largeIconComboBox.addItem ("none", 1);

            for (auto i = 1; i < 5; ++i)
                pc.largeIconComboBox.addItem ("icon" + String (i), i + 1);
            pc.largeIconComboBox.setSelectedItemIndex (0);

            pc.badgeIconComboBox.addItem ("none",  1);
            pc.badgeIconComboBox.addItem ("small", 2);
            pc.badgeIconComboBox.addItem ("large", 3);
            pc.badgeIconComboBox.setSelectedItemIndex (2);

            pc.actionsComboBox.addItem ("none",       1);
            pc.actionsComboBox.addItem ("ok-cancel",  2);
            pc.actionsComboBox.addItem ("text-input", 3);
          #if ALOE_ANDROID
            pc.actionsComboBox.addItem ("ok-cancel-icons", 4);
            pc.actionsComboBox.addItem ("text-input-limited_responses", 5);
          #endif
            pc.actionsComboBox.setSelectedItemIndex (0);

            for (auto i = 0; i < 7; ++i)
                pc.badgeNumberComboBox.addItem (String (i), i + 1);
            pc.badgeNumberComboBox.setSelectedItemIndex (0);

          #if ALOE_IOS
            String prefix = "Notifications/sounds/";
            String extension = ".caf";
          #else
            String prefix;
            String extension;
          #endif

            pc.soundToPlayComboBox.addItem ("none", 1);
            pc.soundToPlayComboBox.addItem ("default_os_sound", 2);
            pc.soundToPlayComboBox.addItem (prefix + "demonstrative"  + extension, 3);
            pc.soundToPlayComboBox.addItem (prefix + "isntit"         + extension, 4);
            pc.soundToPlayComboBox.addItem (prefix + "jinglebellssms" + extension, 5);
            pc.soundToPlayComboBox.addItem (prefix + "served"         + extension, 6);
            pc.soundToPlayComboBox.addItem (prefix + "solemn"         + extension, 7);
            pc.soundToPlayComboBox.setSelectedItemIndex (1);

            for (auto i = 0; i < 11; ++i)
            {
                pc.progressMaxComboBox    .addItem (String (i * 10) + "%", i + 1);
                pc.progressCurrentComboBox.addItem (String (i * 10) + "%", i + 1);
            }

            pc.progressMaxComboBox    .setSelectedItemIndex (0);
            pc.progressCurrentComboBox.setSelectedItemIndex (0);

            pc.notifCategoryComboBox.addItem ("unspecified",     1);
            pc.notifCategoryComboBox.addItem ("alarm",           2);
            pc.notifCategoryComboBox.addItem ("call",            3);
            pc.notifCategoryComboBox.addItem ("email",           4);
            pc.notifCategoryComboBox.addItem ("error",           5);
            pc.notifCategoryComboBox.addItem ("event",           6);
            pc.notifCategoryComboBox.addItem ("message",         7);
            pc.notifCategoryComboBox.addItem ("progress",        8);
            pc.notifCategoryComboBox.addItem ("promo",           9);
            pc.notifCategoryComboBox.addItem ("recommendation", 10);
            pc.notifCategoryComboBox.addItem ("reminder",       11);
            pc.notifCategoryComboBox.addItem ("service",        12);
            pc.notifCategoryComboBox.addItem ("social",         13);
            pc.notifCategoryComboBox.addItem ("status",         14);
            pc.notifCategoryComboBox.addItem ("system",         15);
            pc.notifCategoryComboBox.addItem ("transport",      16);
            pc.notifCategoryComboBox.setSelectedItemIndex (0);

            for (auto i = -2; i < 3; ++i)
                pc.priorityComboBox.addItem (String (i), i + 3);
            pc.priorityComboBox.setSelectedItemIndex (2);

            pc.lockScreenVisibilityComboBox.addItem ("don't show",      1);
            pc.lockScreenVisibilityComboBox.addItem ("show partially",  2);
            pc.lockScreenVisibilityComboBox.addItem ("show completely", 3);
            pc.lockScreenVisibilityComboBox.setSelectedItemIndex (1);

            pc.groupAlertBehaviourComboBox.addItem ("alert all",      1);
            pc.groupAlertBehaviourComboBox.addItem ("alert summary",  2);
            pc.groupAlertBehaviourComboBox.addItem ("alert children", 3);
            pc.groupAlertBehaviourComboBox.setSelectedItemIndex (0);

            pc.timeoutAfterComboBox.addItem ("No timeout", 1);

            for (auto i = 0; i < 10; ++i)
            {
                pc.ledMsToBeOnComboBox      .addItem (String (i * 200) + "ms", i + 1);
                pc.ledMsToBeOffComboBox     .addItem (String (i * 200) + "ms", i + 1);
                pc.vibratorMsToBeOnComboBox .addItem (String (i * 500) + "ms", i + 1);
                pc.vibratorMsToBeOffComboBox.addItem (String (i * 500) + "ms", i + 1);
                pc.timeoutAfterComboBox     .addItem (String (5000 + 1000 * i) + "ms", i + 2);
            }

            pc.ledMsToBeOnComboBox      .setSelectedItemIndex (5);
            pc.ledMsToBeOffComboBox     .setSelectedItemIndex (5);
            pc.vibratorMsToBeOnComboBox .setSelectedItemIndex (0);
            pc.vibratorMsToBeOffComboBox.setSelectedItemIndex (0);
            pc.timeoutAfterComboBox     .setSelectedItemIndex (0);

            pc.timestampVisibilityComboBox.addItem ("off",         1);
            pc.timestampVisibilityComboBox.addItem ("on",          2);
            pc.timestampVisibilityComboBox.addItem ("chronometer", 3);
            pc.timestampVisibilityComboBox.addItem ("count down",  4);
            pc.timestampVisibilityComboBox.setSelectedItemIndex (1);
        */
    }
    
    pub fn distribute_controls(&mut self)  {
        
        todo!();
        /*
            auto& pc = paramControls;

            paramsOneView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.identifierLabel,            pc.identifierEditor));
            paramsOneView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.titleLabel,                 pc.titleEditor));
            paramsOneView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.bodyLabel,                  pc.bodyEditor, 4));
          #if ALOE_IOS
            paramsOneView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.categoryLabel,              pc.categoryComboBox));
          #elif ALOE_ANDROID
            paramsOneView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.channelIdLabel,             pc.channelIdComboBox));
          #endif
          #if ALOE_ANDROID || ALOE_MAC
            paramsOneView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.iconLabel,                  pc.iconComboBox));
          #endif

            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.subtitleLabel,              pc.subtitleEditor));
          #if ! ALOE_MAC
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.badgeNumberLabel,           pc.badgeNumberComboBox));
          #endif
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.soundToPlayLabel,           pc.soundToPlayComboBox));
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.propertiesLabel,            pc.propertiesEditor, 3));
          #if ALOE_IOS || ALOE_MAC
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.fireInLabel,                pc.fireInComboBox));
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.repeatLabel,                pc.repeatButton));
          #elif ALOE_ANDROID
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.largeIconLabel,             pc.largeIconComboBox));
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.badgeIconLabel,             pc.badgeIconComboBox));
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.tickerTextLabel,            pc.tickerTextEditor));
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.autoCancelLabel,            pc.autoCancelButton));
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.alertOnlyOnceLabel,         pc.alertOnlyOnceButton));
          #endif
          #if ALOE_ANDROID || ALOE_MAC
            paramsTwoView  .addRowComponent (new PushNotificationsDemoRowComponent (pc.actionsLabel,               pc.actionsComboBox));
          #endif
          #if ALOE_ANDROID
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.progressMaxLabel,           pc.progressMaxComboBox));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.progressCurrentLabel,       pc.progressCurrentComboBox));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.progressIndeterminateLabel, pc.progressIndeterminateButton));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.categoryLabel,              pc.categoryComboBox));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.priorityLabel,              pc.priorityComboBox));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.personLabel,                pc.personEditor));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.lockScreenVisibilityLabel,  pc.lockScreenVisibilityComboBox));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.groupIdLabel,               pc.groupIdEditor));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.sortKeyLabel,               pc.sortKeyEditor));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.groupSummaryLabel,          pc.groupSummaryButton));
            paramsThreeView.addRowComponent (new PushNotificationsDemoRowComponent (pc.groupAlertBehaviourLabel,   pc.groupAlertBehaviourComboBox));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.accentColourLabel,          pc.accentColourButton));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.ledColourLabel,             pc.ledColourButton));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.ledMsToBeOffLabel,          pc.ledMsToBeOffComboBox));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.ledMsToBeOnLabel,           pc.ledMsToBeOnComboBox));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.vibratorMsToBeOffLabel,     pc.vibratorMsToBeOffComboBox));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.vibratorMsToBeOnLabel,      pc.vibratorMsToBeOnComboBox));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.localOnlyLabel,             pc.localOnlyButton));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.ongoingLabel,               pc.ongoingButton));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.timestampVisibilityLabel,   pc.timestampVisibilityComboBox));
            paramsFourView .addRowComponent (new PushNotificationsDemoRowComponent (pc.timeoutAfterLabel,          pc.timeoutAfterComboBox));
          #endif
        */
    }
    
    pub fn show_remote_instructions()  {
        
        todo!();
        /*
            #if ALOE_IOS || ALOE_MAC
            NativeMessageBox::showAsync (MessageBoxOptions()
                                           .withIconType (MessageBoxIconType::InfoIcon)
                                           .withTitle ("Remote Notifications instructions")
                                           .withMessage ("In order to be able to test remote notifications "
                                                         "ensure that the app is signed and that you register "
                                                         "the bundle ID for remote notifications in "
                                                         "Apple Developer Center."),
                                         nullptr);
           #endif
        */
    }
}

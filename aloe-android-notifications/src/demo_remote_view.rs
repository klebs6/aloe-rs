crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsDemoRemoteView<'a> {
    base:                           Component<'a>,
    get_device_token_button:        TextButton<'a>, // default = { "GetDeviceToken"  }
    send_remote_message_button:     TextButton<'a>, // default = { "SendRemoteMessage"  }
    subscribe_to_sports_button:     TextButton<'a>, // default = { "SubscribeToSports"  }
    unsubscribe_from_sports_button: TextButton<'a>, // default = { "UnsubscribeFromSports"  }
}

#[cfg(target_os="android")]
impl<'a> Default for PushNotificationsDemoRemoteView<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (getDeviceTokenButton);
               #if ALOE_ANDROID
                addAndMakeVisible (sendRemoteMessageButton);
                addAndMakeVisible (subscribeToSportsButton);
                addAndMakeVisible (unsubscribeFromSportsButton);
               #endi
        */
    }
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsDemoRemoteView<'a> {
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto rowSize = getHeight () / 10;

                auto bounds = getLocalBounds().reduced (getWidth() / 10, getHeight() / 10);

                bounds.removeFromTop (2 * rowSize);

                getDeviceTokenButton       .setBounds (bounds.removeFromTop (rowSize));
                sendRemoteMessageButton    .setBounds (bounds.removeFromTop (rowSize));
                subscribeToSportsButton    .setBounds (bounds.removeFromTop (rowSize));
                unsubscribeFromSportsButton.setBounds (bounds.removeFromTop (rowSize));
        */
    }
}


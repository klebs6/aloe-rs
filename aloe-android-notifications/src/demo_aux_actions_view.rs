crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsDemoAuxActionsView<'a> {
    base:                                  Component<'a>,
    get_delivered_notifications_button:    TextButton<'a>, // default = { "Get Delivered Notifications"  }
    remove_delivered_notif_with_id_button: TextButton<'a>, // default = { "Remove Delivered Notif With ID:"  }
    delivered_notif_identifier:            TextEditor<'a>,
    remove_all_delivered_notifs_button:    TextButton<'a>, // default = { "Remove All Delivered Notifs"  }
    get_pending_notifications_button:      TextButton<'a>, // default = { "Get Pending Notifications"  }
    remove_pending_notif_with_id_button:   TextButton<'a>, // default = { "Remove Pending Notif With ID:"  }
    pending_notif_identifier:              TextEditor<'a>,
    remove_all_pending_notifs_button:      TextButton<'a>, // default = { "Remove All Pending Notifs"  }
}

#[cfg(target_os="android")]
impl<'a> Default for PushNotificationsDemoAuxActionsView<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (getDeliveredNotificationsButton);
                addAndMakeVisible (removeDeliveredNotifWithIdButton);
                addAndMakeVisible (deliveredNotifIdentifier);
                addAndMakeVisible (removeAllDeliveredNotifsButton);
              #if ALOE_IOS || ALOE_MAC
                addAndMakeVisible (getPendingNotificationsButton);
                addAndMakeVisible (removePendingNotifWithIdButton);
                addAndMakeVisible (pendingNotifIdentifier);
                addAndMakeVisible (removeAllPendingNotifsButton);
              #endif

                // For now, to be able to dismiss mobile keyboard.
                setWantsKeyboardFocus (true)
        */
    }
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsDemoAuxActionsView<'a> {

    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto columnWidth = getWidth();
                auto rowHeight = getHeight() / 6;
                auto bounds = getLocalBounds();

                getDeliveredNotificationsButton .setBounds (bounds.removeFromTop (rowHeight));

                auto rowBounds = bounds.removeFromTop (rowHeight);
                removeDeliveredNotifWithIdButton.setBounds (rowBounds.removeFromLeft (columnWidth / 2));
                deliveredNotifIdentifier        .setBounds (rowBounds);

                removeAllDeliveredNotifsButton  .setBounds (bounds.removeFromTop (rowHeight));

               #if ALOE_IOS || ALOE_MAC
                getPendingNotificationsButton .setBounds (bounds.removeFromTop (rowHeight));

                rowBounds = bounds.removeFromTop (rowHeight);
                removePendingNotifWithIdButton.setBounds (rowBounds.removeFromLeft (columnWidth / 2));
                pendingNotifIdentifier        .setBounds (rowBounds);

                removeAllPendingNotifsButton  .setBounds (bounds.removeFromTop (rowHeight));
               #endif
        */
    }
}

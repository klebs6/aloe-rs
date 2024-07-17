crate::ix!();

/**
  | Describes settings we want to use for
  | current device. Note that at the moment
  | this is only used on iOS and partially
  | on OSX.
  | 
  | On OSX only allow* flags are used and
  | they control remote notifications
  | only.
  | 
  | To control sound, alert and badge settings
  | for local notifications on OSX, use
  | Notifications settings in System Preferences.
  | 
  | To setup push notifications for current
  | device, provide permissions required,
  | as well as register categories of notifications
  | you want to support. Each category needs
  | to have a unique identifier and it can
  | optionally have multiple actions.
  | Each action also needs to have a unique
  | identifier. The example setup may look
  | as follows:
  | 
  | -----------
  | @code
  | 
  | using Action   = PushNotifications::PushNotificationSettings::Action;
  | using PushNotificationSettingsCategory = PushNotifications::PushNotificationSettings::PushNotificationSettingsCategory;
  | 
  | Action okAction;
  | okAction.identifier = "okAction";
  | okAction.title = "OK!";
  | okAction.style = Action::button;
  | okAction.triggerInBackground = true;
  | 
  | Action cancelAction;
  | cancelAction.identifier = "cancelAction";
  | cancelAction.title = "Cancel";
  | cancelAction.style = Action::button;
  | cancelAction.triggerInBackground = true;
  | cancelAction.destructive = true;
  | 
  | Action textAction;
  | textAction.identifier = "textAction";
  | textAction.title = "Enter text";
  | textAction.style = Action::text;
  | textAction.triggerInBackground = true;
  | textAction.destructive = false;
  | textAction.textInputButtonText = "Ok";
  | textAction.textInputPlaceholder = "Enter text...";
  | 
  | PushNotificationSettingsCategory okCategory;
  | okCategory.identifier = "okCategory";
  | okCategory.actions = { okAction };
  | 
  | PushNotificationSettingsCategory okCancelCategory;
  | okCancelCategory.identifier = "okCancelCategory";
  | okCancelCategory.actions = { okAction, cancelAction };
  | 
  | PushNotificationSettingsCategory textCategory;
  | textCategory.identifier = "textCategory";
  | textCategory.actions = { textAction };
  | textCategory.sendDismissAction = true;
  | 
  | PushNotifications::PushNotificationSettings settings;
  | settings.allowAlert = true;
  | settings.allowBadge = true;
  | settings.allowSound = true;
  | settings.categories = { okCategory, okCancelCategory, textCategory };
  |
  */
pub struct PushNotificationSettings {

    /**
      | whether the app should play a sound upon
      | notification
      |
      */
    allow_sound: bool, // default = false


    /**
      | whether the app should present an alert
      | upon notification
      |
      */
    allow_alert: bool, // default = false


    /**
      | whether the app may badge its icon upon
      | notification
      |
      */
    allow_badge: bool, // default = false


    /**
      | list of categories the app wants to support
      |
      */
    categories:  Vec<PushNotificationSettingsCategory>,
}

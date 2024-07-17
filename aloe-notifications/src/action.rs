crate::ix!();

/**
  | Represents an action on a notification
  | that can be presented as a button or a
  | text input.
  | 
  | On Android, each notification has its
  | action specified explicitly, on iOS
  | you configure an allowed set of actions
  | on startup and pack them into categories
  | (see Settings).
  |
  */
pub struct PushNotificationAction {

    /* -------------- @name Common fields  -------------- */

    style:                  PushNotificationActionStyle, // default = button

    /**
      | Required. the name of the action displayed
      | to the user.
      |
      */
    title:                  String,


    /**
      | Optional: placeholder text for text
      | input notification. Note that it will
      | be ignored if button style is used.
      |
      */
    text_input_placeholder: String,


    /**
      | Optional: additional parameters that
      | can be passed.
      |
      */
    parameters:             Var,

    /* ------------- @name iOS only fields  ------------- */

    /**
      | Required: unique identifier. This
      | should be one of the identifiers set
      | with requestPermissionsWithSettings().
      |
      */
    identifier:             String,


    /**
      | Whether the app can process the action
      | in background.
      |
      */
    trigger_in_background:  bool, // default = false

    /**
      | Whether to display the action as destructive.
      |
      */
    destructive:            bool, // default = false

    /**
      | Optional: Text displayed on text input
      | notification button (from iOS 10 only).
      | Note that it will be ignored if style
      | is set to PushNotificationActionStyle::button.
      |
      */
    text_input_button_text: String,

    /* ----------- @name Android only fields  ----------- */

    /**
      | Optional: name of an icon file (without
      | an extension) to be used for this action.
      | This must be the name of one of the image
      | files included into resources when
      | exporting an Android project (see "Extra
      | Android Raw Resources" setting in Proaloer).
      | 
      | -----------
      | @note
      | 
      | not all Android versions support an
      | icon for an action, though it is recommended
      | to provide it nevertheless.
      |
      */
    icon:              String,

    /**
      | Optional: a list of possible answers
      | if the answer set is limited. When left
      | empty, then the user will be able to input
      | any text.
      |
      */
    allowed_responses: Vec<String>,
}


crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsDemoParamControls<'a> {
    identifier_label:                 Label<'a>, // default = { "identifierLabel",            "Identifier"  }
    identifier_editor:                TextEditor<'a>,
    title_label:                      Label<'a>, // default = { "titleLabel",                 "Title"  }
    title_editor:                     TextEditor<'a>,
    body_label:                       Label<'a>, // default = { "bodyLabel",                  "Body"  }
    body_editor:                      TextEditor<'a>,
    category_label:                   Label<'a>, // default = { "categoryLabel",              "Category"  }
    category_combo_box:               ComboBox<'a>,
    channel_id_label:                 Label<'a>, // default = { "channelIdLabel",             "Channel ID"  }
    channel_id_combo_box:             ComboBox<'a>,
    icon_label:                       Label<'a>, // default = { "iconLabel",                  "Icon"  }
    icon_combo_box:                   ComboBox<'a>,
    subtitle_label:                   Label<'a>, // default = { "subtitleLabel",              "Subtitle"  }
    subtitle_editor:                  TextEditor<'a>,
    badge_number_label:               Label<'a>, // default = { "badgeNumberLabel",           "BadgeNumber"  }
    badge_number_combo_box:           ComboBox<'a>,
    sound_to_play_label:              Label<'a>, // default = { "soundToPlayLabel",           "SoundToPlay"  }
    sound_to_play_combo_box:          ComboBox<'a>,
    properties_label:                 Label<'a>, // default = { "propertiesLabel",            "Properties"  }
    properties_editor:                TextEditor<'a>,
    fire_in_label:                    Label<'a>, // default = { "fireInLabel",                "Fire in"  }
    fire_in_combo_box:                ComboBox<'a>,
    repeat_label:                     Label<'a>, // default = { "repeatLabel",                "Repeat"  }
    repeat_button:                    ToggleButton<'a>,
    large_icon_label:                 Label<'a>, // default = { "largeIconLabel",             "Large Icon"  }
    large_icon_combo_box:             ComboBox<'a>,
    badge_icon_label:                 Label<'a>, // default = { "badgeIconLabel",             "Badge Icon"  }
    badge_icon_combo_box:             ComboBox<'a>,
    ticker_text_label:                Label<'a>, // default = { "tickerTextLabel",            "Ticker Text"  }
    ticker_text_editor:               TextEditor<'a>,
    auto_cancel_label:                Label<'a>, // default = { "autoCancelLabel",            "AutoCancel"  }
    auto_cancel_button:               ToggleButton<'a>,
    alert_only_once_label:            Label<'a>, // default = { "alertOnlyOnceLabel",         "AlertOnlyOnce"  }
    alert_only_once_button:           ToggleButton<'a>,
    actions_label:                    Label<'a>, // default = { "actionsLabel",               "Actions"  }
    actions_combo_box:                ComboBox<'a>,
    progress_max_label:               Label<'a>, // default = { "progressMaxLabel",           "ProgressMax"  }
    progress_max_combo_box:           ComboBox<'a>,
    progress_current_label:           Label<'a>, // default = { "progressCurrentLabel",       "ProgressCurrent"  }
    progress_current_combo_box:       ComboBox<'a>,
    progress_indeterminate_label:     Label<'a>, // default = { "progressIndeterminateLabel", "ProgressIndeterminate"  }
    progress_indeterminate_button:    ToggleButton<'a>,
    notif_category_label:             Label<'a>, // default = { "notifCategoryLabel",         "Category"  }
    notif_category_combo_box:         ComboBox<'a>,
    priority_label:                   Label<'a>, // default = { "priorityLabel",              "Priority"  }
    priority_combo_box:               ComboBox<'a>,
    person_label:                     Label<'a>, // default = { "personLabel",                "Person"  }
    person_editor:                    TextEditor<'a>,
    lock_screen_visibility_label:     Label<'a>, // default = { "lockScreenVisibilityLabel",  "LockScreenVisibility"  }
    lock_screen_visibility_combo_box: ComboBox<'a>,
    group_id_label:                   Label<'a>, // default = { "groupIdLabel",               "GroupID"  }
    group_id_editor:                  TextEditor<'a>,
    sort_key_label:                   Label<'a>, // default = { "sortKeyLabel",               "SortKey"  }
    sort_key_editor:                  TextEditor<'a>,
    group_summary_label:              Label<'a>, // default = { "groupSummaryLabel",          "GroupSummary"  }
    group_summary_button:             ToggleButton<'a>,
    group_alert_behaviour_label:      Label<'a>, // default = { "groupAlertBehaviourLabel",   "GroupAlertBehaviour"  }
    group_alert_behaviour_combo_box:  ComboBox<'a>,
    accent_colour_label:              Label<'a>, // default = { "accentColourLabel",          "AccentColour"  }
    accent_colour_button:             TextButton<'a>,
    led_colour_label:                 Label<'a>, // default = { "ledColourLabel",             "LedColour"  }
    led_colour_button:                TextButton<'a>,
    led_ms_to_be_on_label:            Label<'a>, // default = { "ledMsToBeOnLabel",           "LedMsToBeOn"  }
    led_ms_to_be_on_combo_box:        ComboBox<'a>,
    led_ms_to_be_off_label:           Label<'a>, // default = { "ledMsToBeOffLabel",          "LedMsToBeOff"  }
    led_ms_to_be_off_combo_box:       ComboBox<'a>,
    vibrator_ms_to_be_on_label:       Label<'a>, // default = { "vibratorMsToBeOnLabel",      "VibrationMsToBeOn"  }
    vibrator_ms_to_be_on_combo_box:   ComboBox<'a>,
    vibrator_ms_to_be_off_label:      Label<'a>, // default = { "vibratorMsToBeOffLabel",     "VibrationMsToBeOff"  }
    vibrator_ms_to_be_off_combo_box:  ComboBox<'a>,
    local_only_label:                 Label<'a>, // default = { "localOnlyLabel",             "LocalOnly"  }
    local_only_button:                ToggleButton<'a>,
    ongoing_label:                    Label<'a>, // default = { "ongoingLabel",               "Ongoing"  }
    ongoing_button:                   ToggleButton<'a>,
    timestamp_visibility_label:       Label<'a>, // default = { "timestampVisibilityLabel",   "TimestampMode"  }
    timestamp_visibility_combo_box:   ComboBox<'a>,
    timeout_after_label:              Label<'a>, // default = { "timeoutAfterLabel",          "Timeout After Ms"  }
    timeout_after_combo_box:          ComboBox<'a>,
    accent_colour_selector:           *mut ColourSelector<'a>, // default = nullptr
    led_colour_selector:              *mut ColourSelector<'a>, // default = nullptr
}


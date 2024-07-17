crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_Button.h]

pub const BUTTON_CLICK_MESSAGE_ID: usize = 0x2f3f4f99;

/**
  | A combination of these flags are used
  | by setConnectedEdges().
  |
  */
pub enum ButtonConnectedEdgeFlags
{
    ConnectedOnLeft   = 1,
    ConnectedOnRight  = 2,
    ConnectedOnTop    = 4,
    ConnectedOnBottom = 8
}

/**
  | Used by setState().
  |
  */
pub enum ButtonState
{
    buttonNormal,
    buttonOver,
    buttonDown
}

/**
  | A base class for buttons.
  | 
  | This contains all the logic for button
  | behaviours such as enabling/disabling,
  | responding to shortcut keystrokes,
  | auto-repeating when held down, toggle-buttons
  | and radio groups, etc.
  | 
  | @see TextButton, DrawableButton,
  | ToggleButton
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Button<'a> {
    base:                      Component<'a>,
    base2:                     SettableTooltipClient,

    /**
      | You can assign a lambda to this callback
      | object to have it called when the button is
      | clicked.
      */
    on_click:                  fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the button's
      | state changes.
      */
    on_state_change:           fn() -> (),
    
    shortcuts:                 Vec<KeyPress>,
    key_source:                WeakReference<Component<'a>>,
    text:                      String,
    button_listeners:          ListenerList<&'a dyn ButtonListener>,
    callback_helper:           Box<ButtonCallbackHelper<'a>>,
    button_press_time:         u32, // default = 0
    last_repeat_time:          u32, // default = 0
    command_manager_to_use:    *mut dyn CommandManagerInterface, // default = nullptr
    auto_repeat_delay:         i32, // default = -1
    auto_repeat_speed:         i32, // default = 0
    auto_repeat_minimum_delay: i32, // default = -1
    radio_group_id:            i32, // default = 0
    connected_edge_flags:      i32, // default = 0
    commandid:                 CommandID, // default = {}
    button_state:              ButtonState, // default = buttonNormal
    last_state_painted:        ButtonState, // default = buttonNormal
    is_on:                     Value<'a>,
    last_toggle_state:         bool, // default = false
    click_toggles_state:       bool, // default = false
    needs_to_release:          bool, // default = false
    needs_repainting:          bool, // default = false
    is_key_down:               bool, // default = false
    trigger_on_mouse_down:     bool, // default = false
    generate_tooltip:          bool, // default = false
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_Button.cpp]
impl<'a> Drop for Button<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        clearShortcuts();

        if (commandManagerToUse != nullptr)
            commandManagerToUse->removeListener (callbackHelper.get());

        isOn.removeListener (callbackHelper.get());
        callbackHelper.reset();
 */
    }
}

impl<'a> Button<'a> {

    /**
      | Returns the text displayed in the button.
      | @see setButtonText
      |
      */
    pub fn get_button_text(&self) -> &String {
        
        todo!();
        /*
            return text;
        */
    }

    /**
      | Returns true if the button is 'on'.
      | 
      | By default buttons are 'off' and for
      | simple buttons that you click to perform
      | an action you won't change this. Toggle
      | buttons, however will want to change
      | their state when turned on or off.
      | 
      | @see setToggleState
      |
      */
    pub fn get_toggle_state(&self) -> bool {
        
        todo!();
        /*
            return isOn.getValue();
        */
    }

    /**
      | Returns the Value object that represents
      | the button's toggle state.
      | 
      | You can use this Value object to connect
      | the button's state to external values
      | or setters, either by taking a copy of
      | the Value, or by using Value::referTo()
      | to make it point to your own Value object.
      | 
      | @see getToggleState, Value
      |
      */
    pub fn get_toggle_state_value(&mut self) -> &mut Value {
        
        todo!();
        /*
            return isOn;
        */
    }

    /**
      | Returns the ID of the group to which this
      | button belongs. (See setRadioGroupId()
      | for an explanation of this).
      |
      */
    pub fn get_radio_group_id(&self) -> i32 {
        
        todo!();
        /*
            return radioGroupId;
        */
    }

    /**
      | Returns the command ID that was set by
      | setCommandToTrigger().
      |
      */
    pub fn get_commandid(&self) -> CommandID {
        
        todo!();
        /*
            return commandID;
        */
    }

    /**
      | Returns the set of flags passed into
      | setConnectedEdges().
      |
      */
    pub fn get_connected_edge_flags(&self) -> i32 {
        
        todo!();
        /*
            return connectedEdgeFlags;
        */
    }

    /**
      | Indicates whether the button adjoins
      | another one on its left edge. @see setConnectedEdges
      |
      */
    pub fn is_connected_on_left(&self) -> bool {
        
        todo!();
        /*
            return (connectedEdgeFlags & ConnectedOnLeft) != 0;
        */
    }

    /**
      | Indicates whether the button adjoins
      | another one on its right edge. @see setConnectedEdges
      |
      */
    pub fn is_connected_on_right(&self) -> bool {
        
        todo!();
        /*
            return (connectedEdgeFlags & ConnectedOnRight) != 0;
        */
    }

    /**
      | Indicates whether the button adjoins
      | another one on its top edge. @see setConnectedEdges
      |
      */
    pub fn is_connected_on_top(&self) -> bool {
        
        todo!();
        /*
            return (connectedEdgeFlags & ConnectedOnTop) != 0;
        */
    }

    /**
      | Indicates whether the button adjoins
      | another one on its bottom edge. @see
      | setConnectedEdges
      |
      */
    pub fn is_connected_on_bottom(&self) -> bool {
        
        todo!();
        /*
            return (connectedEdgeFlags & ConnectedOnBottom) != 0;
        */
    }

    /**
      | Returns the button's current over/down/up
      | state.
      |
      */
    pub fn get_state(&self) -> ButtonState {
        
        todo!();
        /*
            return buttonState;
        */
    }
    
    /**
      | Creates a button.
      | 
      | -----------
      | @param buttonName
      | 
      | the text to put in the button (the component's
      | name is also initially set to this string,
      | but these can be changed later using
      | the setName() and setButtonText()
      | methods)
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : component(name),
        : text(name),

            callbackHelper.reset (new ButtonCallbackHelper (*this));

        setWantsKeyboardFocus (true);
        isOn.addListener (callbackHelper.get());
        */
    }
    
    /**
      | Changes the button's text. @see getButtonText
      |
      */
    pub fn set_button_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            if (text != newText)
        {
            text = newText;
            repaint();
        }
        */
    }
    
    /**
      | Sets the tooltip for this button. @see
      | TooltipClient, TooltipWindow
      |
      */
    pub fn set_tooltip(&mut self, new_tooltip: &String)  {
        
        todo!();
        /*
            SettableTooltipClient::setTooltip (newTooltip);
        generateTooltip = false;
        */
    }
    
    pub fn update_automatic_tooltip(&mut self, info: &ApplicationCommandInfo)  {
        
        todo!();
        /*
            if (generateTooltip && commandManagerToUse != nullptr)
        {
            auto tt = info.description.isNotEmpty() ? info.description
                                                    : info.shortName;

            for (auto& kp : commandManagerToUse->getKeyMappings()->getKeyPressesAssignedToCommand (commandID))
            {
                auto key = kp.getTextDescription();

                tt << " [";

                if (key.length() == 1)
                    tt << TRANS("shortcut") << ": '" << key << "']";
                else
                    tt << key << ']';
            }

            SettableTooltipClient::setTooltip (tt);
        }
        */
    }
    
    /**
      | Hints about which edges of the button
      | might be connected to adjoining buttons.
      | 
      | The value passed in is a bitwise combination
      | of any of the values in the ButtonConnectedEdgeFlags
      | enum.
      | 
      | E.g. if you are placing two buttons adjacent
      | to each other, you could use this to indicate
      | which edges are touching, and the LookAndFeel
      | might choose to drawn them without rounded
      | corners on the edges that connect. It's
      | only a hint, so the LookAndFeel can choose
      | to ignore it if it's not relevant for
      | this type of button.
      |
      */
    pub fn set_connected_edges(&mut self, new_flags: i32)  {
        
        todo!();
        /*
            if (connectedEdgeFlags != newFlags)
        {
            connectedEdgeFlags = newFlags;
            repaint();
        }
        */
    }
    
    /**
      | A button has an on/off state associated
      | with it, and this changes that.
      | 
      | By default buttons are 'off' and for
      | simple buttons that you click to perform
      | an action you won't change this. Toggle
      | buttons, however will want to change
      | their state when turned on or off.
      | 
      | -----------
      | @param shouldBeOn
      | 
      | whether to set the button's toggle state
      | to be on or off. If it's a member of a button
      | group, this will always try to turn it
      | on, and to turn off any other buttons
      | in the group
      | ----------
      | @param notification
      | 
      | determines the behaviour if the value
      | changes - this can invoke a synchronous
      | call to clicked(), but sendNotificationAsync
      | is not supported @see getToggleState,
      | setRadioGroupId
      |
      */
    pub fn set_toggle_state_with_notification(
        &mut self, 
        should_be_on: bool,
        notification: NotificationType

    ) {
        
        todo!();
        /*
            setToggleState (shouldBeOn, notification, notification);
        */
    }
    
    pub fn set_toggle_state(
        &mut self, 
        should_be_on:       bool,
        click_notification: NotificationType,
        state_notification: NotificationType

    ) {
        
        todo!();
        /*
            if (shouldBeOn != lastToggleState)
        {
            WeakReference<Component> deletionWatcher (this);

            if (shouldBeOn)
            {
                turnOffOtherButtonsInGroup (clickNotification, stateNotification);

                if (deletionWatcher == nullptr)
                    return;
            }

            // This test is done so that if the value is void rather than explicitly set to
            // false, the value won't be changed unless the required value is true.
            if (getToggleState() != shouldBeOn)
            {
                isOn = shouldBeOn;

                if (deletionWatcher == nullptr)
                    return;
            }

            lastToggleState = shouldBeOn;
            repaint();

            if (clickNotification != dontSendNotification)
            {
                // async callbacks aren't possible here
                jassert (clickNotification != sendNotificationAsync);

                sendClickMessage (ModifierKeys::currentModifiers);

                if (deletionWatcher == nullptr)
                    return;
            }

            if (stateNotification != dontSendNotification)
                sendStateMessage();
            else
                buttonStateChanged();

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::valueChanged);
        }
        */
    }
    
    pub fn set_toggle_state_with_send_change(
        &mut self, 
        should_be_on: bool,
        send_change:  bool

    ) {
        
        todo!();
        /*
            setToggleState (shouldBeOn, sendChange ? sendNotification : dontSendNotification);
        */
    }
    
    /**
      | This tells the button to automatically
      | flip the toggle state when the button
      | is clicked.
      | 
      | If set to true, then before the clicked()
      | callback occurs, the toggle-state
      | of the button is flipped.
      |
      */
    pub fn set_clicking_toggles_state(&mut self, should_toggle: bool)  {
        
        todo!();
        /*
            clickTogglesState = shouldToggle;

        // if you've got clickTogglesState turned on, you shouldn't also connect the button
        // up to be a command invoker. Instead, your command handler must flip the state of whatever
        // it is that this button represents, and the button will update its state to reflect this
        // in the applicationCommandListChanged() method.
        jassert (commandManagerToUse == nullptr || ! clickTogglesState);
        */
    }
    
    /**
      | Returns true if this button is set to
      | be an automatic toggle-button. This
      | returns the last value that was passed
      | to setClickingTogglesState().
      |
      */
    pub fn get_clicking_toggles_state(&self) -> bool {
        
        todo!();
        /*
            return clickTogglesState;
        */
    }
    
    /**
      | Enables the button to act as a member
      | of a mutually-exclusive group of 'radio
      | buttons'.
      | 
      | If the group ID is set to a non-zero number,
      | then this button will act as part of a
      | group of buttons with the same ID, only
      | one of which can be 'on' at the same time.
      | Note that when it's part of a group, clicking
      | a toggle-button that's 'on' won't turn
      | it off.
      | 
      | To find other buttons with the same ID,
      | this button will search through its
      | sibling components for ToggleButtons,
      | so all the buttons for a particular group
      | must be placed inside the same parent
      | component.
      | 
      | Set the group ID back to zero if you want
      | it to act as a normal toggle button again.
      | 
      | The notification argument lets you
      | specify how other buttons should react
      | to being turned on or off in response
      | to this call.
      | 
      | @see getRadioGroupId
      |
      */
    pub fn set_radio_group_id(
        &mut self, 
        new_group_id: i32,
        notification: Option<NotificationType>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotification);
        
        todo!();
        /*
            if (radioGroupId != newGroupId)
        {
            radioGroupId = newGroupId;

            if (lastToggleState)
                turnOffOtherButtonsInGroup (notification, notification);

            invalidateAccessibilityHandler();
        }
        */
    }
    
    pub fn turn_off_other_buttons_in_group(&mut self, 
        click_notification: NotificationType,
        state_notification: NotificationType)  {
        
        todo!();
        /*
            if (auto* p = getParentComponent())
        {
            if (radioGroupId != 0)
            {
                WeakReference<Component> deletionWatcher (this);

                for (auto* c : p->getChildren())
                {
                    if (c != this)
                    {
                        if (auto b = dynamic_cast<Button*> (c))
                        {
                            if (b->getRadioGroupId() == radioGroupId)
                            {
                                b->setToggleState (false, clickNotification, stateNotification);

                                if (deletionWatcher == nullptr)
                                    return;
                            }
                        }
                    }
                }
            }
        }
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            updateState();
        repaint();
        */
    }
    
    pub fn update_state(&mut self) -> ButtonState {
        
        todo!();
        /*
            return updateState (isMouseOver (true), isMouseButtonDown());
        */
    }
    
    pub fn update_state_with_over_and_down(
        &mut self, 
        over: bool,
        down: bool

    ) -> ButtonState {
        
        todo!();
        /*
            ButtonState newState = buttonNormal;

        if (isEnabled() && isVisible() && ! isCurrentlyBlockedByAnotherModalComponent())
        {
            if ((down && (over || (triggerOnMouseDown && buttonState == buttonDown))) || isKeyDown)
                newState = buttonDown;
            else if (over)
                newState = buttonOver;
        }

        setState (newState);
        return newState;
        */
    }
    
    /**
      | Can be used to force the button into a
      | particular state.
      | 
      | This only changes the button's appearance,
      | it won't trigger a click, or stop any
      | mouse-clicks from happening.
      | 
      | The state that you set here will only
      | last until it is automatically changed
      | when the mouse enters or exits the button,
      | or the mouse-button is pressed or released.
      |
      */
    pub fn set_state(&mut self, new_state: ButtonState)  {
        
        todo!();
        /*
            if (buttonState != newState)
        {
            buttonState = newState;
            repaint();

            if (buttonState == buttonDown)
            {
                buttonPressTime = Time::getApproximateMillisecondCounter();
                lastRepeatTime = 0;
            }

            sendStateMessage();
        }
        */
    }
    
    /**
      | Returns true if the button is currently
      | being held down. @see isOver
      |
      */
    pub fn is_down(&self) -> bool {
        
        todo!();
        /*
            return buttonState == buttonDown;
        */
    }
    
    /**
      | Returns true if the mouse is currently
      | over the button. This will be also be
      | true if the button is being held down.
      | @see isDown
      |
      */
    pub fn is_over(&self) -> bool {
        
        todo!();
        /*
            return buttonState != buttonNormal;
        */
    }
    
    pub fn button_state_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the number of milliseconds
      | since the last time the button went into
      | the 'down' state.
      |
      */
    pub fn get_milliseconds_since_button_down(&self) -> u32 {
        
        todo!();
        /*
            auto now = Time::getApproximateMillisecondCounter();
        return now > buttonPressTime ? now - buttonPressTime : 0;
        */
    }
    
    /**
      | Sets whether the button click should
      | happen when the mouse is pressed or released.
      | 
      | By default the button is only considered
      | to have been clicked when the mouse is
      | released, but setting this to true will
      | make it call the clicked() method as
      | soon as the button is pressed.
      | 
      | This is useful if the button is being
      | used to show a pop-up menu, as it allows
      | the click to be used as a drag onto the
      | menu.
      |
      */
    pub fn set_triggered_on_mouse_down(&mut self, is_triggered_on_mouse_down: bool)  {
        
        todo!();
        /*
            triggerOnMouseDown = isTriggeredOnMouseDown;
        */
    }
    
    /**
      | Returns whether the button click happens
      | when the mouse is pressed or released.
      | @see setTriggeredOnMouseDown
      |
      */
    pub fn get_triggered_on_mouse_down(&self) -> bool {
        
        todo!();
        /*
            return triggerOnMouseDown;
        */
    }
    
    pub fn clicked(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn clicked_with_modifiers(&mut self, _0: &ModifierKeys)  {
        
        todo!();
        /*
            clicked();
        */
    }
    
    pub fn trigger_click(&mut self)  {
        
        todo!();
        /*
            postCommandMessage (clickMessageId);
        */
    }
    
    pub fn internal_click_callback(&mut self, modifiers: &ModifierKeys)  {
        
        todo!();
        /*
            if (clickTogglesState)
        {
            const bool shouldBeOn = (radioGroupId != 0 || ! lastToggleState);

            if (shouldBeOn != getToggleState())
            {
                setToggleState (shouldBeOn, sendNotification);
                return;
            }
        }

        sendClickMessage (modifiers);
        */
    }
    
    pub fn flash_button_state(&mut self)  {
        
        todo!();
        /*
            if (isEnabled())
        {
            needsToRelease = true;
            setState (buttonDown);
            callbackHelper->startTimer (100);
        }
        */
    }
    
    pub fn handle_command_message(&mut self, command_id: i32)  {
        
        todo!();
        /*
            if (commandId == clickMessageId)
        {
            if (isEnabled())
            {
                flashButtonState();
                internalClickCallback (ModifierKeys::currentModifiers);
            }
        }
        else
        {
            Component::handleCommandMessage (commandId);
        }
        */
    }
    
    /**
      | Registers a listener to receive events
      | when this button's state changes.
      | 
      | If the listener is already registered,
      | this will not register it again. @see
      | removeListener
      |
      */
    pub fn add_listener(&mut self, l: Option<&mut dyn ButtonListener>)  {
        
        todo!();
        /*
            buttonListeners.add (l);
        */
    }
    
    /**
      | Removes a previously-registered button
      | listener @see addListener
      |
      */
    pub fn remove_listener(&mut self, l: Option<&mut dyn ButtonListener>)  {
        
        todo!();
        /*
            buttonListeners.remove (l);
        */
    }
    
    pub fn send_click_message(&mut self, modifiers: &ModifierKeys)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);

        if (commandManagerToUse != nullptr && commandID != 0)
        {
            ApplicationCommandTarget::InvocationInfo info (commandID);
            info.invocationMethod = ApplicationCommandTarget::InvocationInfo::fromButton;
            info.originatingComponent = this;

            commandManagerToUse->invoke (info, true);
        }

        clicked (modifiers);

        if (checker.shouldBailOut())
            return;

        buttonListeners.callChecked (checker, [this] (ButtonListener& l) { l.buttonClicked (this); });

        if (checker.shouldBailOut())
            return;

        if (onClick != nullptr)
            onClick();
        */
    }
    
    pub fn send_state_message(&mut self)  {
        
        todo!();
        /*
            Component::BailOutChecker checker (this);

        buttonStateChanged();

        if (checker.shouldBailOut())
            return;

        buttonListeners.callChecked (checker, [this] (ButtonListener& l) { l.buttonStateChanged (this); });

        if (checker.shouldBailOut())
            return;

        if (onStateChange != nullptr)
            onStateChange();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (needsToRelease && isEnabled())
        {
            needsToRelease = false;
            needsRepainting = true;
        }

        paintButton (g, isOver(), isDown());
        lastStatePainted = buttonState;
        */
    }
    
    pub fn mouse_enter(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            updateState (true,  false);
        */
    }
    
    pub fn mouse_exit(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            updateState (false, false);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateState (true, true);

        if (isDown())
        {
            if (autoRepeatDelay >= 0)
                callbackHelper->startTimer (autoRepeatDelay);

            if (triggerOnMouseDown)
                internalClickCallback (e.mods);
        }
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            const auto wasDown = isDown();
        const auto wasOver = isOver();
        updateState (isMouseSourceOver (e), false);

        if (wasDown && wasOver && ! triggerOnMouseDown)
        {
            if (lastStatePainted != buttonDown)
                flashButtonState();

            WeakReference<Component> deletionWatcher (this);

            internalClickCallback (e.mods);

            if (deletionWatcher != nullptr)
                updateState (isMouseSourceOver (e), false);
        }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto oldState = buttonState;
        updateState (isMouseSourceOver (e), true);

        if (autoRepeatDelay >= 0 && buttonState != oldState && isDown())
            callbackHelper->startTimer (autoRepeatSpeed);
        */
    }
    
    pub fn is_mouse_source_over(&mut self, e: &MouseEvent) -> bool {
        
        todo!();
        /*
            if (e.source.isTouch() || e.source.isPen())
            return getLocalBounds().toFloat().contains (e.position);

        return isMouseOver();
        */
    }
    
    pub fn focus_gained(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            updateState();
        repaint();
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            updateState();
        repaint();
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            needsToRelease = false;
        updateState();
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            auto* newKeySource = shortcuts.isEmpty() ? nullptr : getTopLevelComponent();

        if (newKeySource != keySource.get())
        {
            if (keySource != nullptr)
                keySource->removeKeyListener (callbackHelper.get());

            keySource = newKeySource;

            if (keySource != nullptr)
                keySource->addKeyListener (callbackHelper.get());
        }
        */
    }
    
    /**
      | Sets a command ID for this button to automatically
      | invoke when it's clicked.
      | 
      | When the button is pressed, it will use
      | the given manager to trigger the command
      | ID.
      | 
      | Obviously be careful that the ApplicationCommandManager
      | doesn't get deleted before this button
      | is. To disable the command triggering,
      | call this method and pass nullptr as
      | the command manager.
      | 
      | If generateTooltip is true, then the
      | button's tooltip will be automatically
      | generated based on the name of this command
      | and its current shortcut key.
      | 
      | @see addShortcut, getCommandID
      |
      */
    pub fn set_command_to_trigger(
        &mut self, 
        new_command_manager: *mut dyn CommandManagerInterface,
        new_commandid:       CommandID,
        generate_tip:        bool

    ) {
        
        todo!();
        /*
            commandID = newCommandID;
        generateTooltip = generateTip;

        if (commandManagerToUse != newCommandManager)
        {
            if (commandManagerToUse != nullptr)
                commandManagerToUse->removeListener (callbackHelper.get());

            commandManagerToUse = newCommandManager;

            if (commandManagerToUse != nullptr)
                commandManagerToUse->addListener (callbackHelper.get());

            // if you've got clickTogglesState turned on, you shouldn't also connect the button
            // up to be a command invoker. Instead, your command handler must flip the state of whatever
            // it is that this button represents, and the button will update its state to reflect this
            // in the applicationCommandListChanged() method.
            jassert (commandManagerToUse == nullptr || ! clickTogglesState);
        }

        if (commandManagerToUse != nullptr)
            applicationCommandListChangeCallback();
        else
            setEnabled (true);
        */
    }
    
    pub fn application_command_list_change_callback(&mut self)  {
        
        todo!();
        /*
            if (commandManagerToUse != nullptr)
        {
            ApplicationCommandInfo info (0);

            if (commandManagerToUse->getTargetForCommand (commandID, info) != nullptr)
            {
                updateAutomaticTooltip (info);
                setEnabled ((info.flags & ApplicationCommandInfo::isDisabled) == 0);
                setToggleState ((info.flags & ApplicationCommandInfo::isTicked) != 0, dontSendNotification);
            }
            else
            {
                setEnabled (false);
            }
        }
        */
    }
    
    /**
      | Assigns a shortcut key to trigger the
      | button.
      | 
      | The button registers itself with its
      | top-level parent component for keypresses.
      | 
      | -----------
      | @note
      | 
      | a different way of linking buttons to
      | keypresses is by using the setCommandToTrigger()
      | method to invoke a command.
      | 
      | @see clearShortcuts
      |
      */
    pub fn add_shortcut(&mut self, key: &KeyPress)  {
        
        todo!();
        /*
            if (key.isValid())
        {
            jassert (! isRegisteredForShortcut (key));  // already registered!

            shortcuts.add (key);
            parentHierarchyChanged();
        }
        */
    }
    
    /**
      | Removes all key shortcuts that had been
      | set for this button. @see addShortcut
      |
      */
    pub fn clear_shortcuts(&mut self)  {
        
        todo!();
        /*
            shortcuts.clear();
        parentHierarchyChanged();
        */
    }
    
    pub fn is_shortcut_pressed(&self) -> bool {
        
        todo!();
        /*
            if (isShowing() && ! isCurrentlyBlockedByAnotherModalComponent())
            for (auto& s : shortcuts)
                if (s.isCurrentlyDown())
                    return true;

        return false;
        */
    }
    
    /**
      | Returns true if the given keypress is
      | a shortcut for this button. @see addShortcut
      |
      */
    pub fn is_registered_for_shortcut(&self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            for (auto& s : shortcuts)
            if (key == s)
                return true;

        return false;
        */
    }
    
    pub fn key_state_changed_callback(&mut self) -> bool {
        
        todo!();
        /*
            if (! isEnabled())
            return false;

        const bool wasDown = isKeyDown;
        isKeyDown = isShortcutPressed();

        if (autoRepeatDelay >= 0 && (isKeyDown && ! wasDown))
            callbackHelper->startTimer (autoRepeatDelay);

        updateState();

        if (isEnabled() && wasDown && ! isKeyDown)
        {
            internalClickCallback (ModifierKeys::currentModifiers);

            // (return immediately - this button may now have been deleted)
            return true;
        }

        return wasDown || isKeyDown;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            if (isEnabled() && key.isKeyCode (KeyPress::returnKey))
        {
            triggerClick();
            return true;
        }

        return false;
        */
    }
    
    /**
      | Sets an auto-repeat speed for the button
      | when it is held down.
      | 
      | (Auto-repeat is disabled by default).
      | 
      | -----------
      | @param initialDelayInMillisecs
      | 
      | how long to wait after the mouse is pressed
      | before triggering the next click. If
      | this is zero, auto-repeat is disabled
      | ----------
      | @param repeatDelayInMillisecs
      | 
      | the frequently subsequent repeated
      | clicks should be triggered
      | ----------
      | @param minimumDelayInMillisecs
      | 
      | if this is greater than 0, the auto-repeat
      | speed will get faster, the longer the
      | button is held down, up to the minimum
      | interval specified here
      |
      */
    pub fn set_repeat_speed(
        &mut self, 
        initial_delay_millisecs:    i32,
        repeat_millisecs:           i32,
        minimum_delay_in_millisecs: Option<i32>

    ) {

        let minimum_delay_in_millisecs: i32 = minimum_delay_in_millisecs.unwrap_or(-1);
        
        todo!();
        /*
            autoRepeatDelay = initialDelayMillisecs;
        autoRepeatSpeed = repeatMillisecs;
        autoRepeatMinimumDelay = jmin (autoRepeatSpeed, minimumDelayInMillisecs);
        */
    }
    
    pub fn repeat_timer_callback(&mut self)  {
        
        todo!();
        /*
            if (needsRepainting)
        {
            callbackHelper->stopTimer();
            updateState();
            needsRepainting = false;
        }
        else if (autoRepeatSpeed > 0 && (isKeyDown || (updateState() == buttonDown)))
        {
            auto repeatSpeed = autoRepeatSpeed;

            if (autoRepeatMinimumDelay >= 0)
            {
                auto timeHeldDown = jmin (1.0, getMillisecondsSinceButtonDown() / 4000.0);
                timeHeldDown *= timeHeldDown;

                repeatSpeed = repeatSpeed + (int) (timeHeldDown * (autoRepeatMinimumDelay - repeatSpeed));
            }

            repeatSpeed = jmax (1, repeatSpeed);

            auto now = Time::getMillisecondCounter();

            // if we've been blocked from repeating often enough, speed up the repeat timer to compensate..
            if (lastRepeatTime != 0 && (int) (now - lastRepeatTime) > repeatSpeed * 2)
                repeatSpeed = jmax (1, repeatSpeed / 2);

            lastRepeatTime = now;
            callbackHelper->startTimer (repeatSpeed);

            internalClickCallback (ModifierKeys::currentModifiers);
        }
        else if (! needsToRelease)
        {
            callbackHelper->stopTimer();
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<ButtonAccessibilityHandler> (*this, AccessibilityRole::button);
        */
    }
}

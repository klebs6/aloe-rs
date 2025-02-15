crate::ix!();

impl<'a> ValueListener for Impl<'a> {

    fn value_changed(&mut self, value: &mut Value)  {
        
        todo!();
        /*
            if (value.refersToSameSourceAs (currentValue))
            {
                if (style != TwoValueHorizontal && style != TwoValueVertical)
                    setValue (currentValue.getValue(), dontSendNotification);
            }
            else if (value.refersToSameSourceAs (valueMin))
            {
                setMinValue (valueMin.getValue(), dontSendNotification, true);
            }
            else if (value.refersToSameSourceAs (valueMax))
            {
                setMaxValue (valueMax.getValue(), dontSendNotification, true);
            }
        */
    }
}

pub struct Impl<'a> {

    /**
      | this needs to be public otherwise it
      | will cause an error when ALOE_DLL_BUILD=1
      |
      */
    base:                         AsyncUpdater<'a>,
    owner:                        &'a mut Slider<'a>,
    style:                        SliderStyle,
    listeners:                    ListenerList<Rc<RefCell<dyn SliderListener>>>,
    current_value:                Value<'a>,
    value_min:                    Value<'a>,
    value_max:                    Value<'a>,
    last_current_value:           f64, // default = 0
    last_value_min:               f64, // default = 0
    last_value_max:               f64, // default = 0
    norm_range:                   NormalisableRange<f64>, //{ 0.0, 10.0 };
    double_click_return_value:    f64, // default = 0
    value_when_last_dragged:      f64, // default = 0
    value_on_mouse_down:          f64, // default = 0
    last_angle:                   f64, // default = 0
    velocity_mode_sensitivity:    f64, // default = 1.0
    velocity_mode_offset:         f64, // default = 0
    min_max_diff:                 f64, // default = 0
    velocity_mode_threshold:      i32, // default = 1
    rotary_params:                SliderRotaryParameters,
    mouse_drag_start_pos:         Point<f32>,
    mouse_pos_when_last_dragged:  Point<f32>,
    slider_region_start:          i32, // default = 0
    slider_region_size:           i32, // default = 1
    slider_being_dragged:         i32, // default = -1
    pixels_for_full_drag_extent:  i32, // default = 250
    last_mouse_wheel_time:        Time,
    slider_rect:                  Rectangle<i32>,
    current_drag:                 Box<SliderScopedDragNotification<'a>>,
    text_box_pos:                 SliderTextEntryBoxPosition,
    text_suffix:                  String,
    num_decimal_places:           i32, // default = 7
    text_box_width:               i32, // default = 80
    text_box_height:              i32, // default = 20
    inc_dec_button_mode:          SliderIncDecButtonMode, // default = incDecButtonsNotDraggable
    modifier_to_swap_modes:       ModifierKeysFlags, // default = ModifierKeys_ctrlAltCommandModifiers
    editable_text:                bool, // default = true
    double_click_to_value:        bool, // default = false
    is_velocity_based:            bool, // default = false
    user_key_overrides_velocity:  bool, // default = true
    inc_dec_buttons_side_by_side: bool, // default = false
    send_change_only_on_release:  bool, // default = false
    show_popup_on_drag:           bool, // default = false
    show_popup_on_hover:          bool, // default = false
    menu_enabled:                 bool, // default = false
    use_drag_events:              bool, // default = false
    inc_dec_dragged:              bool, // default = false
    scroll_wheel_enabled:         bool, // default = true
    snaps_to_mouse_pos:           bool, // default = true
    popup_hover_timeout:          i32, // default = 2000
    last_popup_dismissal:         f64, // default = 0.0
    single_click_modifiers:       ModifierKeys,
    value_box:                    Box<Label<'a>>,
    inc_button:                   Box<Button<'a>>,
    dec_button:                   Box<Button<'a>>,
    popup_display:                Box<PopupDisplayComponent<'a>>,
    parent_for_popup_display:     *mut Component<'a>, // default = nullptr
}

//TODO
pub trait LookAndFeelSliderInterface {}

impl<'a> Drop for Impl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            currentValue.removeListener (this);
            valueMin.removeListener (this);
            valueMax.removeListener (this);
            popupDisplay.reset();
        */
    }
}

impl<'a> Impl<'a> {

    pub fn new(
        s:                 &mut Slider,
        slider_style:      SliderStyle,
        text_box_position: SliderTextEntryBoxPosition) -> Self {
    
        todo!();
        /*
        : owner(s),
        : style(sliderStyle),
        : text_box_pos(textBoxPosition),

            rotaryParams.startAngleRadians = MathConstants<float>::pi * 1.2f;
            rotaryParams.endAngleRadians   = MathConstants<float>::pi * 2.8f;
            rotaryParams.stopAtEnd = true;
        */
    }
    
    pub fn register_listeners(&mut self)  {
        
        todo!();
        /*
            currentValue.addListener (this);
            valueMin.addListener (this);
            valueMax.addListener (this);
        */
    }
    
    pub fn is_horizontal(&self) -> bool {
        
        todo!();
        /*
            return style == LinearHorizontal
                || style == LinearBar
                || style == TwoValueHorizontal
                || style == ThreeValueHorizontal;
        */
    }
    
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return style == LinearVertical
                || style == LinearBarVertical
                || style == TwoValueVertical
                || style == ThreeValueVertical;
        */
    }
    
    pub fn is_rotary(&self) -> bool {
        
        todo!();
        /*
            return style == Rotary
                || style == RotaryHorizontalDrag
                || style == RotaryVerticalDrag
                || style == RotaryHorizontalVerticalDrag;
        */
    }
    
    pub fn is_bar(&self) -> bool {
        
        todo!();
        /*
            return style == LinearBar
                || style == LinearBarVertical;
        */
    }
    
    pub fn is_two_value(&self) -> bool {
        
        todo!();
        /*
            return style == TwoValueHorizontal
                || style == TwoValueVertical;
        */
    }
    
    pub fn is_three_value(&self) -> bool {
        
        todo!();
        /*
            return style == ThreeValueHorizontal
                || style == ThreeValueVertical;
        */
    }
    
    pub fn inc_dec_drag_direction_is_horizontal(&self) -> bool {
        
        todo!();
        /*
            return incDecButtonMode == incDecButtonsDraggable_Horizontal
                    || (incDecButtonMode == incDecButtonsDraggable_AutoDirection && incDecButtonsSideBySide);
        */
    }
    
    pub fn get_position_of_value(&self, value: f64) -> f32 {
        
        todo!();
        /*
            if (isHorizontal() || isVertical())
                return getLinearSliderPos (value);

            jassertfalse; // not a valid call on a slider that doesn't work linearly!
            return 0.0f;
        */
    }
    
    pub fn update_range(&mut self)  {
        
        todo!();
        /*
            // figure out the number of DPs needed to display all values at this
            // interval setting.
            numDecimalPlaces = 7;

            if (normRange.interval != 0.0)
            {
                int v = std::abs (roundToInt (normRange.interval * 10000000));

                while ((v % 10) == 0 && numDecimalPlaces > 0)
                {
                    --numDecimalPlaces;
                    v /= 10;
                }
            }

            // keep the current values inside the new range..
            if (style != TwoValueHorizontal && style != TwoValueVertical)
            {
                setValue (getValue(), dontSendNotification);
            }
            else
            {
                setMinValue (getMinValue(), dontSendNotification, false);
                setMaxValue (getMaxValue(), dontSendNotification, false);
            }

            updateText();
        */
    }
    
    pub fn set_range(&mut self, 
        new_min: f64,
        new_max: f64,
        new_int: f64)  {
        
        todo!();
        /*
            normRange = NormalisableRange<double> (newMin, newMax, newInt,
                                                   normRange.skew, normRange.symmetricSkew);
            updateRange();
        */
    }
    
    pub fn set_normalisable_range(&mut self, new_range: NormalisableRange<f64>)  {
        
        todo!();
        /*
            normRange = newRange;
            updateRange();
        */
    }
    
    pub fn get_value(&self) -> f64 {
        
        todo!();
        /*
            // for a two-value style slider, you should use the getMinValue() and getMaxValue()
            // methods to get the two values.
            jassert (style != TwoValueHorizontal && style != TwoValueVertical);

            return currentValue.getValue();
        */
    }
    
    pub fn set_value(&mut self, 
        new_value:    f64,
        notification: NotificationType)  {
        
        todo!();
        /*
            // for a two-value style slider, you should use the setMinValue() and setMaxValue()
            // methods to set the two values.
            jassert (style != TwoValueHorizontal && style != TwoValueVertical);

            newValue = constrainedValue (newValue);

            if (style == ThreeValueHorizontal || style == ThreeValueVertical)
            {
                jassert (static_cast<double> (valueMin.getValue()) <= static_cast<double> (valueMax.getValue()));

                newValue = jlimit (static_cast<double> (valueMin.getValue()),
                                   static_cast<double> (valueMax.getValue()),
                                   newValue);
            }

            if (newValue != lastCurrentValue)
            {
                if (valueBox != nullptr)
                    valueBox->hideEditor (true);

                lastCurrentValue = newValue;

                // (need to do this comparison because the Value will use equalsWithSameType to compare
                // the new and old values, so will generate unwanted change events if the type changes)
                if (currentValue != newValue)
                    currentValue = newValue;

                updateText();
                owner.repaint();
                updatePopupDisplay (newValue);

                triggerChangeMessage (notification);
            }
        */
    }
    
    pub fn set_min_value(&mut self, 
        new_value:                     f64,
        notification:                  NotificationType,
        allow_nudging_of_other_values: bool)  {
        
        todo!();
        /*
            // The minimum value only applies to sliders that are in two- or three-value mode.
            jassert (style == TwoValueHorizontal || style == TwoValueVertical
                      || style == ThreeValueHorizontal || style == ThreeValueVertical);

            newValue = constrainedValue (newValue);

            if (style == TwoValueHorizontal || style == TwoValueVertical)
            {
                if (allowNudgingOfOtherValues && newValue > static_cast<double> (valueMax.getValue()))
                    setMaxValue (newValue, notification, false);

                newValue = jmin (static_cast<double> (valueMax.getValue()), newValue);
            }
            else
            {
                if (allowNudgingOfOtherValues && newValue > lastCurrentValue)
                    setValue (newValue, notification);

                newValue = jmin (lastCurrentValue, newValue);
            }

            if (lastValueMin != newValue)
            {
                lastValueMin = newValue;
                valueMin = newValue;
                owner.repaint();
                updatePopupDisplay (newValue);

                triggerChangeMessage (notification);
            }
        */
    }
    
    pub fn set_max_value(&mut self, 
        new_value:                     f64,
        notification:                  NotificationType,
        allow_nudging_of_other_values: bool)  {
        
        todo!();
        /*
            // The maximum value only applies to sliders that are in two- or three-value mode.
            jassert (style == TwoValueHorizontal || style == TwoValueVertical
                      || style == ThreeValueHorizontal || style == ThreeValueVertical);

            newValue = constrainedValue (newValue);

            if (style == TwoValueHorizontal || style == TwoValueVertical)
            {
                if (allowNudgingOfOtherValues && newValue < static_cast<double> (valueMin.getValue()))
                    setMinValue (newValue, notification, false);

                newValue = jmax (static_cast<double> (valueMin.getValue()), newValue);
            }
            else
            {
                if (allowNudgingOfOtherValues && newValue < lastCurrentValue)
                    setValue (newValue, notification);

                newValue = jmax (lastCurrentValue, newValue);
            }

            if (lastValueMax != newValue)
            {
                lastValueMax = newValue;
                valueMax = newValue;
                owner.repaint();
                updatePopupDisplay (valueMax.getValue());

                triggerChangeMessage (notification);
            }
        */
    }
    
    pub fn set_min_and_max_values(&mut self, 
        new_min_value: f64,
        new_max_value: f64,
        notification:  NotificationType)  {
        
        todo!();
        /*
            // The maximum value only applies to sliders that are in two- or three-value mode.
            jassert (style == TwoValueHorizontal || style == TwoValueVertical
                      || style == ThreeValueHorizontal || style == ThreeValueVertical);

            if (newMaxValue < newMinValue)
                std::swap (newMaxValue, newMinValue);

            newMinValue = constrainedValue (newMinValue);
            newMaxValue = constrainedValue (newMaxValue);

            if (lastValueMax != newMaxValue || lastValueMin != newMinValue)
            {
                lastValueMax = newMaxValue;
                lastValueMin = newMinValue;
                valueMin = newMinValue;
                valueMax = newMaxValue;
                owner.repaint();

                triggerChangeMessage (notification);
            }
        */
    }
    
    pub fn get_min_value(&self) -> f64 {
        
        todo!();
        /*
            // The minimum value only applies to sliders that are in two- or three-value mode.
            jassert (style == TwoValueHorizontal || style == TwoValueVertical
                      || style == ThreeValueHorizontal || style == ThreeValueVertical);

            return valueMin.getValue();
        */
    }
    
    pub fn get_max_value(&self) -> f64 {
        
        todo!();
        /*
            // The maximum value only applies to sliders that are in two- or three-value mode.
            jassert (style == TwoValueHorizontal || style == TwoValueVertical
                      || style == ThreeValueHorizontal || style == ThreeValueVertical);

            return valueMax.getValue();
        */
    }
    
    pub fn trigger_change_message(&mut self, notification: NotificationType)  {
        
        todo!();
        /*
            if (notification != dontSendNotification)
            {
                owner.valueChanged();

                if (notification == sendNotificationSync)
                    handleAsyncUpdate();
                else
                    triggerAsyncUpdate();
            }
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            cancelPendingUpdate();

            Component::BailOutChecker checker (&owner);
            listeners.callChecked (checker, [&] (Slider::SliderListener& l) { l.sliderValueChanged (&owner); });

            if (checker.shouldBailOut())
                return;

            if (owner.onValueChange != nullptr)
                owner.onValueChange();

            if (auto* handler = owner.getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::valueChanged);
        */
    }
    
    pub fn send_drag_start(&mut self)  {
        
        todo!();
        /*
            owner.startedDragging();

            Component::BailOutChecker checker (&owner);
            listeners.callChecked (checker, [&] (Slider::SliderListener& l) { l.sliderDragStarted (&owner); });

            if (checker.shouldBailOut())
                return;

            if (owner.onDragStart != nullptr)
                owner.onDragStart();
        */
    }
    
    pub fn send_drag_end(&mut self)  {
        
        todo!();
        /*
            owner.stoppedDragging();
            sliderBeingDragged = -1;

            Component::BailOutChecker checker (&owner);
            listeners.callChecked (checker, [&] (Slider::SliderListener& l) { l.sliderDragEnded (&owner); });

            if (checker.shouldBailOut())
                return;

            if (owner.onDragEnd != nullptr)
                owner.onDragEnd();
        */
    }
    
    pub fn increment_or_decrement(&mut self, delta: f64)  {
        
        todo!();
        /*
            if (style == IncDecButtons)
            {
                auto newValue = owner.snapValue (getValue() + delta, notDragging);

                if (currentDrag != nullptr)
                {
                    setValue (newValue, sendNotificationSync);
                }
                else
                {
                    SliderScopedDragNotification drag (owner);
                    setValue (newValue, sendNotificationSync);
                }
            }
        */
    }
    
    pub fn text_changed(&mut self)  {
        
        todo!();
        /*
            auto newValue = owner.snapValue (owner.getValueFromText (valueBox->getText()), notDragging);

            if (newValue != static_cast<double> (currentValue.getValue()))
            {
                SliderScopedDragNotification drag (owner);
                setValue (newValue, sendNotificationSync);
            }

            updateText(); // force a clean-up of the text, needed in case setValue() hasn't done this.
        */
    }
    
    pub fn update_text(&mut self)  {
        
        todo!();
        /*
            if (valueBox != nullptr)
            {
                auto newValue = owner.getTextFromValue (currentValue.getValue());

                if (newValue != valueBox->getText())
                    valueBox->setText (newValue, dontSendNotification);
            }
        */
    }
    
    pub fn constrained_value(&self, value: f64) -> f64 {
        
        todo!();
        /*
            return normRange.snapToLegalValue (value);
        */
    }
    
    pub fn get_linear_slider_pos(&self, value: f64) -> f32 {
        
        todo!();
        /*
            double pos;

            if (normRange.end <= normRange.start)
                pos = 0.5;
            else if (value < normRange.start)
                pos = 0.0;
            else if (value > normRange.end)
                pos = 1.0;
            else
                pos = owner.valueToProportionOfLength (value);

            if (isVertical() || style == IncDecButtons)
                pos = 1.0 - pos;

            jassert (pos >= 0 && pos <= 1.0);
            return (float) (sliderRegionStart + pos * sliderRegionSize);
        */
    }
    
    pub fn set_slider_style(&mut self, new_style: SliderStyle)  {
        
        todo!();
        /*
            if (style != newStyle)
            {
                style = newStyle;

                owner.repaint();
                owner.lookAndFeelChanged();
                owner.invalidateAccessibilityHandler();
            }
        */
    }
    
    pub fn set_velocity_mode_parameters(&mut self, 
        sensitivity:                     f64,
        threshold:                       i32,
        offset:                          f64,
        user_can_press_key_to_swap_mode: bool,
        new_modifier_to_swap_modes:      ModifierKeysFlags)  {
        
        todo!();
        /*
            velocityModeSensitivity = sensitivity;
            velocityModeOffset = offset;
            velocityModeThreshold = threshold;
            userKeyOverridesVelocity = userCanPressKeyToSwapMode;
            modifierToSwapModes = newModifierToSwapModes;
        */
    }
    
    pub fn set_inc_dec_buttons_mode(&mut self, mode: SliderIncDecButtonMode)  {
        
        todo!();
        /*
            if (incDecButtonMode != mode)
            {
                incDecButtonMode = mode;
                owner.lookAndFeelChanged();
            }
        */
    }
    
    pub fn set_text_box_style(&mut self, 
        new_position:          SliderTextEntryBoxPosition,
        is_read_only:          bool,
        text_entry_box_width:  i32,
        text_entry_box_height: i32)  {
        
        todo!();
        /*
            if (textBoxPos != newPosition
                 || editableText != (! isReadOnly)
                 || textBoxWidth != textEntryBoxWidth
                 || textBoxHeight != textEntryBoxHeight)
            {
                textBoxPos = newPosition;
                editableText = ! isReadOnly;
                textBoxWidth = textEntryBoxWidth;
                textBoxHeight = textEntryBoxHeight;

                owner.repaint();
                owner.lookAndFeelChanged();
            }
        */
    }
    
    pub fn set_text_box_is_editable(&mut self, should_be_editable: bool)  {
        
        todo!();
        /*
            editableText = shouldBeEditable;
            updateTextBoxEnablement();
        */
    }
    
    pub fn show_text_box(&mut self)  {
        
        todo!();
        /*
            jassert (editableText); // this should probably be avoided in read-only sliders.

            if (valueBox != nullptr)
                valueBox->showEditor();
        */
    }
    
    pub fn hide_text_box(&mut self, discard_current_editor_contents: bool)  {
        
        todo!();
        /*
            if (valueBox != nullptr)
            {
                valueBox->hideEditor (discardCurrentEditorContents);

                if (discardCurrentEditorContents)
                    updateText();
            }
        */
    }
    
    pub fn set_text_value_suffix(&mut self, suffix: &String)  {
        
        todo!();
        /*
            if (textSuffix != suffix)
            {
                textSuffix = suffix;
                updateText();
            }
        */
    }
    
    pub fn update_text_box_enablement(&mut self)  {
        
        todo!();
        /*
            if (valueBox != nullptr)
            {
                bool shouldBeEditable = editableText && owner.isEnabled();

                if (valueBox->isEditable() != shouldBeEditable) // (to avoid changing the single/double click flags unless we need to)
                    valueBox->setEditable (shouldBeEditable);
            }
        */
    }
    
    pub fn look_and_feel_changed(&mut self, lf: &mut dyn SliderLookAndFeelMethods)  {
        
        todo!();
        /*
            if (textBoxPos != NoTextBox)
            {
                auto previousTextBoxContent = (valueBox != nullptr ? valueBox->getText()
                                                                   : owner.getTextFromValue (currentValue.getValue()));

                valueBox.reset();
                valueBox.reset (lf.createSliderTextBox (owner));
                owner.addAndMakeVisible (valueBox.get());

                valueBox->setWantsKeyboardFocus (false);
                valueBox->setAccessible (false);
                valueBox->setText (previousTextBoxContent, dontSendNotification);
                valueBox->setTooltip (owner.getTooltip());
                updateTextBoxEnablement();
                valueBox->onTextChange = [this] { textChanged(); };

                if (style == LinearBar || style == LinearBarVertical)
                {
                    valueBox->addMouseListener (&owner, false);
                    valueBox->setMouseCursor (MouseCursor::ParentCursor);
                }
            }
            else
            {
                valueBox.reset();
            }

            if (style == IncDecButtons)
            {
                incButton.reset (lf.createSliderButton (owner, true));
                decButton.reset (lf.createSliderButton (owner, false));

                auto tooltip = owner.getTooltip();

                auto setupButton = [&] (Button& b, bool isIncrement)
                {
                    owner.addAndMakeVisible (b);
                    b.onClick = [this, isIncrement] { incrementOrDecrement (isIncrement ? normRange.interval : -normRange.interval); };

                    if (incDecButtonMode != incDecButtonsNotDraggable)
                        b.addMouseListener (&owner, false);
                    else
                        b.setRepeatSpeed (300, 100, 20);

                    b.setTooltip (tooltip);
                    b.setAccessible (false);
                };

                setupButton (*incButton, true);
                setupButton (*decButton, false);
            }
            else
            {
                incButton.reset();
                decButton.reset();
            }

            owner.setComponentEffect (lf.getSliderEffect (owner));

            owner.resized();
            owner.repaint();
        */
    }
    
    pub fn show_popup_menu(&mut self)  {
        
        todo!();
        /*
            PopupMenu m;
            m.setLookAndFeel (&owner.getLookAndFeel());
            m.addItem (1, TRANS ("Velocity-sensitive mode"), true, isVelocityBased);
            m.addSeparator();

            if (isRotary())
            {
                PopupMenu rotaryMenu;
                rotaryMenu.addItem (2, TRANS ("Use circular dragging"),           true, style == Rotary);
                rotaryMenu.addItem (3, TRANS ("Use left-right dragging"),         true, style == RotaryHorizontalDrag);
                rotaryMenu.addItem (4, TRANS ("Use up-down dragging"),            true, style == RotaryVerticalDrag);
                rotaryMenu.addItem (5, TRANS ("Use left-right/up-down dragging"), true, style == RotaryHorizontalVerticalDrag);

                m.addSubMenu (TRANS ("Rotary mode"), rotaryMenu);
            }

            m.showMenuAsync (typename PopupMenu::Options(),
                             ModalCallbackFunction::forComponent (sliderMenuCallback, &owner));
        */
    }
    
    pub fn slider_menu_callback(
        result: i32,
        slider: *mut Slider)  {
        
        todo!();
        /*
            if (slider != nullptr)
            {
                switch (result)
                {
                    case 1:   slider->setVelocityBasedMode (! slider->getVelocityBasedMode()); break;
                    case 2:   slider->setSliderStyle (Rotary); break;
                    case 3:   slider->setSliderStyle (RotaryHorizontalDrag); break;
                    case 4:   slider->setSliderStyle (RotaryVerticalDrag); break;
                    case 5:   slider->setSliderStyle (RotaryHorizontalVerticalDrag); break;
                    default:  break;
                }
            }
        */
    }
    
    pub fn get_thumb_index_at(&mut self, e: &MouseEvent) -> i32 {
        
        todo!();
        /*
            if (isTwoValue() || isThreeValue())
            {
                auto mousePos = isVertical() ? e.position.y : e.position.x;

                auto normalPosDistance = std::abs (getLinearSliderPos (currentValue.getValue()) - mousePos);
                auto minPosDistance    = std::abs (getLinearSliderPos (valueMin.getValue()) + (isVertical() ? 0.1f : -0.1f) - mousePos);
                auto maxPosDistance    = std::abs (getLinearSliderPos (valueMax.getValue()) + (isVertical() ? -0.1f : 0.1f) - mousePos);

                if (isTwoValue())
                    return maxPosDistance <= minPosDistance ? 2 : 1;

                if (normalPosDistance >= minPosDistance && maxPosDistance >= minPosDistance)
                    return 1;

                if (normalPosDistance >= maxPosDistance)
                    return 2;
            }

            return 0;
        */
    }
    
    pub fn handle_rotary_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto dx = e.position.x - (float) sliderRect.getCentreX();
            auto dy = e.position.y - (float) sliderRect.getCentreY();

            if (dx * dx + dy * dy > 25.0f)
            {
                auto angle = std::atan2 ((double) dx, (double) -dy);

                while (angle < 0.0)
                    angle += MathConstants<double>::twoPi;

                if (rotaryParams.stopAtEnd && e.mouseWasDraggedSinceMouseDown())
                {
                    if (std::abs (angle - lastAngle) > MathConstants<double>::pi)
                    {
                        if (angle >= lastAngle)
                            angle -= MathConstants<double>::twoPi;
                        else
                            angle += MathConstants<double>::twoPi;
                    }

                    if (angle >= lastAngle)
                        angle = jmin (angle, (double) jmax (rotaryParams.startAngleRadians, rotaryParams.endAngleRadians));
                    else
                        angle = jmax (angle, (double) jmin (rotaryParams.startAngleRadians, rotaryParams.endAngleRadians));
                }
                else
                {
                    while (angle < rotaryParams.startAngleRadians)
                        angle += MathConstants<double>::twoPi;

                    if (angle > rotaryParams.endAngleRadians)
                    {
                        if (smallestAngleBetween (angle, rotaryParams.startAngleRadians)
                             <= smallestAngleBetween (angle, rotaryParams.endAngleRadians))
                            angle = rotaryParams.startAngleRadians;
                        else
                            angle = rotaryParams.endAngleRadians;
                    }
                }

                auto proportion = (angle - rotaryParams.startAngleRadians) / (rotaryParams.endAngleRadians - rotaryParams.startAngleRadians);
                valueWhenLastDragged = owner.proportionOfLengthToValue (jlimit (0.0, 1.0, proportion));
                lastAngle = angle;
            }
        */
    }
    
    pub fn handle_absolute_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            auto mousePos = (isHorizontal() || style == RotaryHorizontalDrag) ? e.position.x : e.position.y;
            double newPos = 0;

            if (style == RotaryHorizontalDrag
                || style == RotaryVerticalDrag
                || style == IncDecButtons
                || ((style == LinearHorizontal || style == LinearVertical || style == LinearBar || style == LinearBarVertical)
                    && ! snapsToMousePos))
            {
                auto mouseDiff = (style == RotaryHorizontalDrag
                                    || style == LinearHorizontal
                                    || style == LinearBar
                                    || (style == IncDecButtons && incDecDragDirectionIsHorizontal()))
                                  ? e.position.x - mouseDragStartPos.x
                                  : mouseDragStartPos.y - e.position.y;

                newPos = owner.valueToProportionOfLength (valueOnMouseDown)
                           + mouseDiff * (1.0 / pixelsForFullDragExtent);

                if (style == IncDecButtons)
                {
                    incButton->setState (mouseDiff < 0 ? Button::buttonNormal : Button::buttonDown);
                    decButton->setState (mouseDiff > 0 ? Button::buttonNormal : Button::buttonDown);
                }
            }
            else if (style == RotaryHorizontalVerticalDrag)
            {
                auto mouseDiff = (e.position.x - mouseDragStartPos.x)
                                   + (mouseDragStartPos.y - e.position.y);

                newPos = owner.valueToProportionOfLength (valueOnMouseDown)
                           + mouseDiff * (1.0 / pixelsForFullDragExtent);
            }
            else
            {
                newPos = (mousePos - (float) sliderRegionStart) / (double) sliderRegionSize;

                if (isVertical())
                    newPos = 1.0 - newPos;
            }

            newPos = (isRotary() && ! rotaryParams.stopAtEnd) ? newPos - std::floor (newPos)
                                                              : jlimit (0.0, 1.0, newPos);
            valueWhenLastDragged = owner.proportionOfLengthToValue (newPos);
        */
    }
    
    pub fn handle_velocity_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            bool hasHorizontalStyle =
                (isHorizontal() ||  style == RotaryHorizontalDrag
                                || (style == IncDecButtons && incDecDragDirectionIsHorizontal()));

            auto mouseDiff = style == RotaryHorizontalVerticalDrag
                                ? (e.position.x - mousePosWhenLastDragged.x) + (mousePosWhenLastDragged.y - e.position.y)
                                : (hasHorizontalStyle ? e.position.x - mousePosWhenLastDragged.x
                                                      : e.position.y - mousePosWhenLastDragged.y);

            auto maxSpeed = jmax (200.0, (double) sliderRegionSize);
            auto speed = jlimit (0.0, maxSpeed, (double) std::abs (mouseDiff));

            if (speed != 0.0)
            {
                speed = 0.2 * velocityModeSensitivity
                          * (1.0 + std::sin (MathConstants<double>::pi * (1.5 + jmin (0.5, velocityModeOffset
                                                                                             + jmax (0.0, (double) (speed - velocityModeThreshold))
                                                                                                / maxSpeed))));

                if (mouseDiff < 0)
                    speed = -speed;

                if (isVertical() || style == RotaryVerticalDrag
                     || (style == IncDecButtons && ! incDecDragDirectionIsHorizontal()))
                    speed = -speed;

                auto newPos = owner.valueToProportionOfLength (valueWhenLastDragged) + speed;
                newPos = (isRotary() && ! rotaryParams.stopAtEnd) ? newPos - std::floor (newPos)
                                                                  : jlimit (0.0, 1.0, newPos);
                valueWhenLastDragged = owner.proportionOfLengthToValue (newPos);

                e.source.enableUnboundedMouseMovement (true, false);
            }
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            incDecDragged = false;
            useDragEvents = false;
            mouseDragStartPos = mousePosWhenLastDragged = e.position;
            currentDrag.reset();
            popupDisplay.reset();

            if (owner.isEnabled())
            {
                if (e.mods.isPopupMenu() && menuEnabled)
                {
                    showPopupMenu();
                }
                else if (canDoubleClickToValue()
                         && (singleClickModifiers != ModifierKeys() && e.mods.withoutMouseButtons() == singleClickModifiers))
                {
                    mouseDoubleClick();
                }
                else if (normRange.end > normRange.start)
                {
                    useDragEvents = true;

                    if (valueBox != nullptr)
                        valueBox->hideEditor (true);

                    sliderBeingDragged = getThumbIndexAt (e);

                    minMaxDiff = static_cast<double> (valueMax.getValue()) - static_cast<double> (valueMin.getValue());

                    if (! isTwoValue())
                        lastAngle = rotaryParams.startAngleRadians
                                        + (rotaryParams.endAngleRadians - rotaryParams.startAngleRadians)
                                             * owner.valueToProportionOfLength (currentValue.getValue());

                    valueWhenLastDragged = (sliderBeingDragged == 2 ? valueMax
                                                                    : (sliderBeingDragged == 1 ? valueMin
                                                                                               : currentValue)).getValue();
                    valueOnMouseDown = valueWhenLastDragged;

                    if (showPopupOnDrag || showPopupOnHover)
                    {
                        showPopupDisplay();

                        if (popupDisplay != nullptr)
                            popupDisplay->stopTimer();
                    }

                    currentDrag = std::make_unique<SliderScopedDragNotification> (owner);
                    mouseDrag (e);
                }
            }
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (useDragEvents && normRange.end > normRange.start
                 && ! ((style == LinearBar || style == LinearBarVertical)
                        && e.mouseWasClicked() && valueBox != nullptr && valueBox->isEditable()))
            {
                SliderDragMode dragMode = notDragging;

                if (style == Rotary)
                {
                    handleRotaryDrag (e);
                }
                else
                {
                    if (style == IncDecButtons && ! incDecDragged)
                    {
                        if (e.getDistanceFromDragStart() < 10 || ! e.mouseWasDraggedSinceMouseDown())
                            return;

                        incDecDragged = true;
                        mouseDragStartPos = e.position;
                    }

                    if (isAbsoluteDragMode (e.mods) || (normRange.end - normRange.start) / sliderRegionSize < normRange.interval)
                    {
                        dragMode = absoluteDrag;
                        handleAbsoluteDrag (e);
                    }
                    else
                    {
                        dragMode = velocityDrag;
                        handleVelocityDrag (e);
                    }
                }

                valueWhenLastDragged = jlimit (normRange.start, normRange.end, valueWhenLastDragged);

                if (sliderBeingDragged == 0)
                {
                    setValue (owner.snapValue (valueWhenLastDragged, dragMode),
                              sendChangeOnlyOnRelease ? dontSendNotification : sendNotificationSync);
                }
                else if (sliderBeingDragged == 1)
                {
                    setMinValue (owner.snapValue (valueWhenLastDragged, dragMode),
                                 sendChangeOnlyOnRelease ? dontSendNotification : sendNotificationAsync, true);

                    if (e.mods.isShiftDown())
                        setMaxValue (getMinValue() + minMaxDiff, dontSendNotification, true);
                    else
                        minMaxDiff = static_cast<double> (valueMax.getValue()) - static_cast<double> (valueMin.getValue());
                }
                else if (sliderBeingDragged == 2)
                {
                    setMaxValue (owner.snapValue (valueWhenLastDragged, dragMode),
                                 sendChangeOnlyOnRelease ? dontSendNotification : sendNotificationAsync, true);

                    if (e.mods.isShiftDown())
                        setMinValue (getMaxValue() - minMaxDiff, dontSendNotification, true);
                    else
                        minMaxDiff = static_cast<double> (valueMax.getValue()) - static_cast<double> (valueMin.getValue());
                }

                mousePosWhenLastDragged = e.position;
            }
        */
    }
    
    pub fn mouse_up(&mut self)  {
        
        todo!();
        /*
            if (owner.isEnabled()
                 && useDragEvents
                 && (normRange.end > normRange.start)
                 && (style != IncDecButtons || incDecDragged))
            {
                restoreMouseIfHidden();

                if (sendChangeOnlyOnRelease && valueOnMouseDown != static_cast<double> (currentValue.getValue()))
                    triggerChangeMessage (sendNotificationAsync);

                currentDrag.reset();
                popupDisplay.reset();

                if (style == IncDecButtons)
                {
                    incButton->setState (Button::buttonNormal);
                    decButton->setState (Button::buttonNormal);
                }
            }
            else if (popupDisplay != nullptr)
            {
                popupDisplay->startTimer (200);
            }

            currentDrag.reset();
        */
    }
    
    pub fn mouse_move(&mut self)  {
        
        todo!();
        /*
            // this is a workaround for a bug where the popup display being dismissed triggers
            // a mouse move causing it to never be hidden
            auto shouldShowPopup = showPopupOnHover
                                    && (Time::getMillisecondCounterHiRes() - lastPopupDismissal) > 250;

            if (shouldShowPopup
                 && ! isTwoValue()
                 && ! isThreeValue())
            {
                if (owner.isMouseOver (true))
                {
                    if (popupDisplay == nullptr)
                        showPopupDisplay();

                    if (popupDisplay != nullptr && popupHoverTimeout != -1)
                        popupDisplay->startTimer (popupHoverTimeout);
                }
            }
        */
    }
    
    pub fn mouse_exit(&mut self)  {
        
        todo!();
        /*
            popupDisplay.reset();
        */
    }
    
    pub fn show_popup_display(&mut self)  {
        
        todo!();
        /*
            if (style == IncDecButtons)
                return;

            if (popupDisplay == nullptr)
            {
                popupDisplay.reset (new PopupDisplayComponent (owner, parentForPopupDisplay == nullptr));

                if (parentForPopupDisplay != nullptr)
                    parentForPopupDisplay->addChildComponent (popupDisplay.get());
                else
                    popupDisplay->addToDesktop (ComponentPeer::windowIsTemporary
                                                | ComponentPeer::windowIgnoresKeyPresses
                                                | ComponentPeer::windowIgnoresMouseClicks);

                if (style == SliderStyle::TwoValueHorizontal
                    || style == SliderStyle::TwoValueVertical)
                {
                    updatePopupDisplay (sliderBeingDragged == 2 ? getMaxValue()
                                                                : getMinValue());
                }
                else
                {
                    updatePopupDisplay (getValue());
                }

                popupDisplay->setVisible (true);
            }
        */
    }
    
    pub fn update_popup_display(&mut self, value_to_show: f64)  {
        
        todo!();
        /*
            if (popupDisplay != nullptr)
                popupDisplay->updatePosition (owner.getTextFromValue (valueToShow));
        */
    }
    
    pub fn can_double_click_to_value(&self) -> bool {
        
        todo!();
        /*
            return doubleClickToValue
                    && style != IncDecButtons
                    && normRange.start <= doubleClickReturnValue
                    && normRange.end >= doubleClickReturnValue;
        */
    }
    
    pub fn mouse_double_click(&mut self)  {
        
        todo!();
        /*
            if (canDoubleClickToValue())
            {
                SliderScopedDragNotification drag (owner);
                setValue (doubleClickReturnValue, sendNotificationSync);
            }
        */
    }
    
    pub fn get_mouse_wheel_delta(&mut self, 
        value:        f64,
        wheel_amount: f64) -> f64 {
        
        todo!();
        /*
            if (style == IncDecButtons)
                return normRange.interval * wheelAmount;

            auto proportionDelta = wheelAmount * 0.15;
            auto currentPos = owner.valueToProportionOfLength (value);
            auto newPos = currentPos + proportionDelta;
            newPos = (isRotary() && ! rotaryParams.stopAtEnd) ? newPos - std::floor (newPos)
                                                              : jlimit (0.0, 1.0, newPos);
            return owner.proportionOfLengthToValue (newPos) - value;
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails) -> bool {
        
        todo!();
        /*
            if (scrollWheelEnabled
                 && style != TwoValueHorizontal
                 && style != TwoValueVertical)
            {
                // sometimes duplicate wheel events seem to be sent, so since we're going to
                // bump the value by a minimum of the interval, avoid doing this twice..
                if (e.eventTime != lastMouseWheelTime)
                {
                    lastMouseWheelTime = e.eventTime;

                    if (normRange.end > normRange.start && ! e.mods.isAnyMouseButtonDown())
                    {
                        if (valueBox != nullptr)
                            valueBox->hideEditor (false);

                        auto value = static_cast<double> (currentValue.getValue());
                        auto delta = getMouseWheelDelta (value, (std::abs (wheel.deltaX) > std::abs (wheel.deltaY)
                                                                      ? -wheel.deltaX : wheel.deltaY)
                                                                   * (wheel.isReversed ? -1.0f : 1.0f));
                        if (delta != 0.0)
                        {
                            auto newValue = value + jmax (normRange.interval, std::abs (delta)) * (delta < 0 ? -1.0 : 1.0);

                            SliderScopedDragNotification drag (owner);
                            setValue (owner.snapValue (newValue, notDragging), sendNotificationSync);
                        }
                    }
                }

                return true;
            }

            return false;
        */
    }
    
    pub fn modifier_keys_changed(&mut self, modifiers: &ModifierKeys)  {
        
        todo!();
        /*
            if (style != IncDecButtons && style != Rotary && isAbsoluteDragMode (modifiers))
                restoreMouseIfHidden();
        */
    }
    
    pub fn is_absolute_drag_mode(&self, mods: ModifierKeys) -> bool {
        
        todo!();
        /*
            return isVelocityBased == (userKeyOverridesVelocity && mods.testFlags (modifierToSwapModes));
        */
    }
    
    pub fn restore_mouse_if_hidden(&mut self)  {
        
        todo!();
        /*
            for (auto& ms : Desktop::getInstance().getMouseSources())
            {
                if (ms.isUnboundedMouseMovementEnabled())
                {
                    ms.enableUnboundedMouseMovement (false);

                    auto pos = sliderBeingDragged == 2 ? getMaxValue()
                                                       : (sliderBeingDragged == 1 ? getMinValue()
                                                                                  : static_cast<double> (currentValue.getValue()));
                    Point<float> mousePos;

                    if (isRotary())
                    {
                        mousePos = ms.getLastMouseDownPosition();

                        auto delta = (float) (pixelsForFullDragExtent * (owner.valueToProportionOfLength (valueOnMouseDown)
                                                                           - owner.valueToProportionOfLength (pos)));

                        if (style == RotaryHorizontalDrag)      mousePos += Point<float> (-delta, 0.0f);
                        else if (style == RotaryVerticalDrag)   mousePos += Point<float> (0.0f, delta);
                        else                                    mousePos += Point<float> (delta / -2.0f, delta / 2.0f);

                        mousePos = owner.getScreenBounds().reduced (4).toFloat().getConstrainedPoint (mousePos);
                        mouseDragStartPos = mousePosWhenLastDragged = owner.getLocalPoint (nullptr, mousePos);
                        valueOnMouseDown = valueWhenLastDragged;
                    }
                    else
                    {
                        auto pixelPos = (float) getLinearSliderPos (pos);

                        mousePos = owner.localPointToGlobal (Point<float> (isHorizontal() ? pixelPos : ((float) owner.getWidth()  / 2.0f),
                                                                           isVertical()   ? pixelPos : ((float) owner.getHeight() / 2.0f)));
                    }

                    const_cast <MouseInputSource&> (ms).setScreenPosition (mousePos);
                }
            }
        */
    }
    
    pub fn paint(
        &mut self, 
        g:  &mut Graphics,
        lf: &mut dyn LookAndFeelSliderInterface

    ) {
        
        todo!();
        /*
            if (style != IncDecButtons)
            {
                if (isRotary())
                {
                    auto sliderPos = (float) owner.valueToProportionOfLength (lastCurrentValue);
                    jassert (sliderPos >= 0 && sliderPos <= 1.0f);

                    lf.drawRotarySlider (g,
                                         sliderRect.getX(), sliderRect.getY(),
                                         sliderRect.getWidth(), sliderRect.getHeight(),
                                         sliderPos, rotaryParams.startAngleRadians,
                                         rotaryParams.endAngleRadians, owner);
                }
                else
                {
                    lf.drawLinearSlider (g,
                                         sliderRect.getX(), sliderRect.getY(),
                                         sliderRect.getWidth(), sliderRect.getHeight(),
                                         getLinearSliderPos (lastCurrentValue),
                                         getLinearSliderPos (lastValueMin),
                                         getLinearSliderPos (lastValueMax),
                                         style, owner);
                }

                if ((style == LinearBar || style == LinearBarVertical) && valueBox == nullptr)
                {
                    g.setColour (owner.findColour (Slider::textBoxOutlineColourId));
                    g.drawRect (0, 0, owner.getWidth(), owner.getHeight(), 1);
                }
            }
        */
    }
    
    pub fn resized(&mut self, lf: &mut dyn LookAndFeelSliderInterface)  {
        
        todo!();
        /*
            auto layout = lf.getSliderLayout (owner);
            sliderRect = layout.sliderBounds;

            if (valueBox != nullptr)
                valueBox->setBounds (layout.textBoxBounds);

            if (isHorizontal())
            {
                sliderRegionStart = layout.sliderBounds.getX();
                sliderRegionSize = layout.sliderBounds.getWidth();
            }
            else if (isVertical())
            {
                sliderRegionStart = layout.sliderBounds.getY();
                sliderRegionSize = layout.sliderBounds.getHeight();
            }
            else if (style == IncDecButtons)
            {
                resizeIncDecButtons();
            }
        */
    }
    
    pub fn resize_inc_dec_buttons(&mut self)  {
        
        todo!();
        /*
            auto buttonRect = sliderRect;

            if (textBoxPos == TextBoxLeft || textBoxPos == TextBoxRight)
                buttonRect.expand (-2, 0);
            else
                buttonRect.expand (0, -2);

            incDecButtonsSideBySide = buttonRect.getWidth() > buttonRect.getHeight();

            if (incDecButtonsSideBySide)
            {
                decButton->setBounds (buttonRect.removeFromLeft (buttonRect.getWidth() / 2));
                decButton->setConnectedEdges (Button::ConnectedOnRight);
                incButton->setConnectedEdges (Button::ConnectedOnLeft);
            }
            else
            {
                decButton->setBounds (buttonRect.removeFromBottom (buttonRect.getHeight() / 2));
                decButton->setConnectedEdges (Button::ConnectedOnTop);
                incButton->setConnectedEdges (Button::ConnectedOnBottom);
            }

            incButton->setBounds (buttonRect);
        */
    }
    
    pub fn smallest_angle_between(a1: f64, a2: f64) -> f64 {
        
        todo!();
        /*
            return jmin (std::abs (a1 - a2),
                         std::abs (a1 + MathConstants<double>::twoPi - a2),
                         std::abs (a2 + MathConstants<double>::twoPi - a1));
        */
    }
}

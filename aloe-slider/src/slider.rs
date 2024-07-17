crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_Slider.h]

/**
  | A slider control for changing a value.
  | 
  | The slider can be horizontal, vertical,
  | or rotary, and can optionally have a
  | text-box inside it to show an editable
  | display of the current value.
  | 
  | To use it, create a Slider object and
  | use the setSliderStyle() method to
  | set up the type you want. To set up the
  | text-entry box, use setTextBoxStyle().
  | 
  | To define the values that it can be set
  | to, see the setRange() and setValue()
  | methods.
  | 
  | There are also lots of custom tweaks
  | you can do by subclassing and overriding
  | some of the virtual methods, such as
  | changing the scaling, changing the
  | format of the text display, custom ways
  | of limiting the values, etc.
  | 
  | You can register Slider::SliderListener
  | objects with a slider, and they'll be
  | called when the value changes.
  | 
  | @see Slider::SliderListener
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Slider<'a> {
    base:                     Component<'a>,
    base2:                    SettableTooltipClient,

    /**
      | You can assign a lambda to this callback
      | object to have it called when the slider
      | value is changed.
      |
      */
    on_value_change:          fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the slider's
      | drag begins.
      |
      */
    on_drag_start:            fn() -> (),

    /**
      | You can assign a lambda to this callback
      | object to have it called when the slider's
      | drag ends.
      |
      */
    on_drag_end:              fn() -> (),

    /**
      | You can assign a lambda that will be used
      | to convert textual values to the slider's
      | normalised position.
      |
      */
    value_from_text_function: fn(_0: &String) -> f64,

    /**
      | You can assign a lambda that will be used
      | to convert the slider's normalised
      | position to a textual value.
      |
      */
    text_from_value_function: fn(_0: f64) -> String,

    pimpl:                    Box<Pimpl<'a>>,
}

impl<'a> Default for Slider<'a> {
    
    /**
      | Creates a slider.
      | 
      | When created, you can set up the slider's
      | style and range with setSliderStyle(),
      | setRange(), etc.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            init (LinearHorizontal, TextBoxLeft);
        */
    }
    
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_Slider.cpp]
impl<'a> Slider<'a> {

    /**
      | Creates a slider.
      | 
      | When created, you can set up the slider's
      | style and range with setSliderStyle(),
      | setRange(), etc.
      |
      */
    pub fn new_with_name(name: &String) -> Self {
    
        todo!();
        /*
        : component(name),

            init (LinearHorizontal, TextBoxLeft);
        */
    }
    
    /**
      | Creates a slider with some explicit
      | options.
      |
      */
    pub fn new(
        style:        SliderStyle,
        text_box_pos: SliderTextEntryBoxPosition

    ) -> Self {
    
        todo!();
        /*


            init (style, textBoxPos);
        */
    }
    
    pub fn init(&mut self, 
        style:        SliderStyle,
        text_box_pos: SliderTextEntryBoxPosition)  {
        
        todo!();
        /*
            setWantsKeyboardFocus (false);
        setRepaintsOnMouseActivity (true);

        pimpl.reset (new Pimpl (*this, style, textBoxPos));

        Slider::lookAndFeelChanged();
        updateText();

        pimpl->registerListeners();
        */
    }
    
    /**
      | Adds a listener to be called when this
      | slider's value changes.
      |
      */
    pub fn add_listener(&mut self, l: Option<&mut dyn SliderListener>)  {
        
        todo!();
        /*
            pimpl->listeners.add (l);
        */
    }
    
    /**
      | Removes a previously-registered listener.
      |
      */
    pub fn remove_listener(&mut self, l: Option<&mut dyn SliderListener>)  {
        
        todo!();
        /*
            pimpl->listeners.remove (l);
        */
    }
    
    /**
      | Returns the slider's current style.
      | @see setSliderStyle
      |
      */
    pub fn get_slider_style(&self) -> SliderStyle {
        
        todo!();
        /*
            return pimpl->style;
        */
    }
    
    /**
      | Changes the type of slider interface
      | being used.
      | 
      | -----------
      | @param newStyle
      | 
      | the type of interface @see setRotaryParameters,
      | setVelocityBasedMode
      |
      */
    pub fn set_slider_style(&mut self, new_style: SliderStyle)  {
        
        todo!();
        /*
            pimpl->setSliderStyle (newStyle);
        */
    }
    
    /**
      | Changes the properties of a rotary slider.
      |
      */
    pub fn set_rotary_parameters(&mut self, p: SliderRotaryParameters)  {
        
        todo!();
        /*
            // make sure the values are sensible..
        jassert (p.startAngleRadians >= 0 && p.endAngleRadians >= 0);
        jassert (p.startAngleRadians < MathConstants<float>::pi * 4.0f
                  && p.endAngleRadians < MathConstants<float>::pi * 4.0f);

        pimpl->rotaryParams = p;
        */
    }
    
    /**
      | Changes the properties of a rotary slider.
      |
      */
    pub fn set_rotary_parameters_from_start_and_end_angle_radians(
        &mut self, 
        start_angle_radians: f32,
        end_angle_radians:   f32,
        stop_at_end:         bool

    ) {
        
        todo!();
        /*
            setRotaryParameters ({ startAngleRadians, endAngleRadians, stopAtEnd });
        */
    }
    
    /**
      | Changes the properties of a rotary slider.
      |
      */
    pub fn get_rotary_parameters(&self) -> SliderRotaryParameters {
        
        todo!();
        /*
            return pimpl->rotaryParams;
        */
    }
    
    /**
      | Changes the way the mouse is used when
      | dragging the slider.
      | 
      | If true, this will turn on velocity-sensitive
      | dragging, so that the faster the mouse
      | moves, the bigger the movement to the
      | slider. This helps when making accurate
      | adjustments if the slider's range is
      | quite large.
      | 
      | If false, the slider will just try to
      | snap to wherever the mouse is.
      |
      */
    pub fn set_velocity_based_mode(&mut self, vb: bool)  {
        
        todo!();
        /*
            pimpl->isVelocityBased = vb;
        */
    }
    
    /**
      | Returns true if velocity-based mode
      | is active. @see setVelocityBasedMode
      |
      */
    pub fn get_velocity_based_mode(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isVelocityBased;
        */
    }
    
    /**
      | Returns the velocity user key setting.
      | @see setVelocityModeParameters
      |
      */
    pub fn get_velocity_mode_is_swappable(&self) -> bool {
        
        todo!();
        /*
            return pimpl->userKeyOverridesVelocity;
        */
    }
    
    /**
      | Returns the velocity threshold setting.
      | @see setVelocityModeParameters
      |
      */
    pub fn get_velocity_threshold(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->velocityModeThreshold;
        */
    }
    
    /**
      | Returns the velocity sensitivity setting.
      | @see setVelocityModeParameters
      |
      */
    pub fn get_velocity_sensitivity(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->velocityModeSensitivity;
        */
    }
    
    /**
      | Returns the velocity offset setting.
      | @see setVelocityModeParameters
      |
      */
    pub fn get_velocity_offset(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->velocityModeOffset;
        */
    }
    
    /**
      | Changes aspects of the scaling used
      | when in velocity-sensitive mode.
      | 
      | These apply when you've used setVelocityBasedMode()
      | to turn on velocity mode, or if you're
      | holding down ctrl.
      | 
      | -----------
      | @param sensitivity
      | 
      | higher values than 1.0 increase the
      | range of acceleration used
      | ----------
      | @param threshold
      | 
      | the minimum number of pixels that the
      | mouse needs to move for it to be treated
      | as a movement
      | ----------
      | @param offset
      | 
      | values greater than 0.0 increase the
      | minimum speed that will be used when
      | the threshold is reached
      | ----------
      | @param userCanPressKeyToSwapMode
      | 
      | if true, then the user can hold down the
      | ctrl or command key to toggle velocity-sensitive
      | mode
      | ----------
      | @param modifiersToSwapModes
      | 
      | this is a set of modifier flags which
      | will be tested when determining whether
      | to enable/disable velocity-sensitive
      | mode
      |
      */
    pub fn set_velocity_mode_parameters(
        &mut self, 
        sensitivity:                     Option<f64>,
        threshold:                       Option<i32>,
        offset:                          Option<f64>,
        user_can_press_key_to_swap_mode: Option<bool>,
        modifier_to_swap_modes:          Option<ModifierKeysFlags>

    ) {

        let sensitivity:                                     f64 = sensitivity.unwrap_or(1.0);
        let threshold:                                       i32 = threshold.unwrap_or(1);
        let offset:                                          f64 = offset.unwrap_or(0.0);
        let user_can_press_key_to_swap_mode:                bool = user_can_press_key_to_swap_mode.unwrap_or(true);

        let modifier_to_swap_modes: ModifierKeysFlags 
            = modifier_to_swap_modes.unwrap_or(ModifierKeysFlags::ctrlAltCommandModifiers);
        
        todo!();
        /*
            jassert (threshold >= 0);
        jassert (sensitivity > 0);
        jassert (offset >= 0);

        pimpl->setVelocityModeParameters (sensitivity, threshold, offset,
                                          userCanPressKeyToSwapMode, modifierToSwapModes);
        */
    }
    
    /**
      | Returns the current skew factor.
      | 
      | See setSkewFactor for more info. @see
      | setSkewFactor, setSkewFactorFromMidPoint,
      | isSymmetricSkew
      |
      */
    pub fn get_skew_factor(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->normRange.skew;
        */
    }
    
    /**
      | Returns the whether the skew is symmetric
      | from the midpoint to both sides.
      | 
      | See setSkewFactor for more info. @see
      | getSkewFactor, setSkewFactor, setSkewFactorFromMidPoint
      |
      */
    pub fn is_symmetric_skew(&self) -> bool {
        
        todo!();
        /*
            return pimpl->normRange.symmetricSkew;
        */
    }
    
    /**
      | Sets up a skew factor to alter the way
      | values are distributed.
      | 
      | You may want to use a range of values on
      | the slider where more accuracy is required
      | towards one end of the range, so this
      | will logarithmically spread the values
      | across the length of the slider.
      | 
      | If the factor is < 1.0, the lower end of
      | the range will fill more of the slider's
      | length; if the factor is > 1.0, the upper
      | end of the range will be expanded instead.
      | A factor of 1.0 doesn't skew it at all.
      | 
      | If symmetricSkew is true, the skew factor
      | applies from the middle of the slider
      | to each of its ends.
      | 
      | To set the skew position by using a mid-point,
      | use the setSkewFactorFromMidPoint()
      | method instead.
      | 
      | @see getSkewFactor, setSkewFactorFromMidPoint,
      | isSymmetricSkew
      |
      */
    pub fn set_skew_factor(
        &mut self, 
        factor:         f64,
        symmetric_skew: Option<bool>

    ) {

        let symmetric_skew: bool = symmetric_skew.unwrap_or(false);
        
        todo!();
        /*
            pimpl->normRange.skew = factor;
        pimpl->normRange.symmetricSkew = symmetricSkew;
        */
    }
    
    /**
      | Sets up a skew factor to alter the way
      | values are distributed.
      | 
      | This allows you to specify the slider
      | value that should appear in the centre
      | of the slider's visible range.
      | 
      | @see setSkewFactor, getSkewFactor,
      | isSymmetricSkew
      |
      */
    pub fn set_skew_factor_from_mid_point(&mut self, slider_value_to_show_at_mid_point: f64)  {
        
        todo!();
        /*
            pimpl->normRange.setSkewForCentre (sliderValueToShowAtMidPoint);
        */
    }
    
    /**
      | Returns the current sensitivity value
      | set by setMouseDragSensitivity().
      |
      */
    pub fn get_mouse_drag_sensitivity(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->pixelsForFullDragExtent;
        */
    }
    
    /**
      | Sets the distance the mouse has to move
      | to drag the slider across the full extent
      | of its range.
      | 
      | This only applies when in modes like
      | RotaryHorizontalDrag, where it's
      | using relative mouse movements to adjust
      | the slider.
      |
      */
    pub fn set_mouse_drag_sensitivity(&mut self, distance_for_full_scale_drag: i32)  {
        
        todo!();
        /*
            jassert (distanceForFullScaleDrag > 0);

        pimpl->pixelsForFullDragExtent = distanceForFullScaleDrag;
        */
    }
    
    /**
      | When the style is IncDecButtons, this
      | lets you turn on a mode where the mouse
      | can be dragged on the buttons to drag
      | the values.
      | 
      | By default this is turned off. When enabled,
      | clicking on the buttons still works
      | them as normal, but by holding down the
      | mouse on a button and dragging it a little
      | distance, it flips into a mode where
      | the value can be dragged. The drag direction
      | can either be set explicitly to be vertical
      | or horizontal, or can be set to incDecButtonsDraggable_AutoDirection
      | so that it depends on whether the buttons
      | are side-by-side or above each other.
      |
      */
    pub fn set_inc_dec_buttons_mode(&mut self, mode: SliderIncDecButtonMode)  {
        
        todo!();
        /*
            pimpl->setIncDecButtonsMode (mode);
        */
    }
    
    /**
      | Returns the status of the text-box.
      | @see setTextBoxStyle
      |
      */
    pub fn get_text_box_position(&self) -> SliderTextEntryBoxPosition {
        
        todo!();
        /*
            return pimpl->textBoxPos;
        */
    }
    
    /**
      | Returns the width used for the text-box.
      | @see setTextBoxStyle
      |
      */
    pub fn get_text_box_width(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->textBoxWidth;
        */
    }
    
    /**
      | Returns the height used for the text-box.
      | @see setTextBoxStyle
      |
      */
    pub fn get_text_box_height(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->textBoxHeight;
        */
    }
    
    /**
      | Changes the location and properties
      | of the text-entry box.
      | 
      | -----------
      | @param newPosition
      | 
      | where it should go (or NoTextBox to not
      | have one at all)
      | ----------
      | @param isReadOnly
      | 
      | if true, it's a read-only display
      | ----------
      | @param textEntryBoxWidth
      | 
      | the width of the text-box in pixels.
      | Make sure this leaves enough room for
      | the slider as well!
      | ----------
      | @param textEntryBoxHeight
      | 
      | the height of the text-box in pixels.
      | Make sure this leaves enough room for
      | the slider as well!
      | 
      | @see setTextBoxIsEditable, getValueFromText,
      | getTextFromValue
      |
      */
    pub fn set_text_box_style(&mut self, 
        new_position:          SliderTextEntryBoxPosition,
        is_read_only:          bool,
        text_entry_box_width:  i32,
        text_entry_box_height: i32)  {
        
        todo!();
        /*
            pimpl->setTextBoxStyle (newPosition, isReadOnly, textEntryBoxWidth, textEntryBoxHeight);
        */
    }
    
    /**
      | Returns true if the text-box is read-only.
      | @see setTextBoxStyle
      |
      */
    pub fn is_text_box_editable(&self) -> bool {
        
        todo!();
        /*
            return pimpl->editableText;
        */
    }
    
    /**
      | Makes the text-box editable.
      | 
      | By default this is true, and the user
      | can enter values into the textbox, but
      | it can be turned off if that's not suitable.
      | 
      | @see setTextBoxStyle, getValueFromText,
      | getTextFromValue
      |
      */
    pub fn set_text_box_is_editable(&mut self, should_be_editable: bool)  {
        
        todo!();
        /*
            pimpl->setTextBoxIsEditable (shouldBeEditable);
        */
    }
    
    /**
      | If the text-box is editable, this will
      | give it the focus so that the user can
      | type directly into it.
      | 
      | This is basically the effect as the user
      | clicking on it.
      |
      */
    pub fn show_text_box(&mut self)  {
        
        todo!();
        /*
            pimpl->showTextBox();
        */
    }
    
    /**
      | If the text-box currently has focus
      | and is being edited, this resets it and
      | takes keyboard focus away from it.
      | 
      | -----------
      | @param discardCurrentEditorContents
      | 
      | if true, the slider's value will be left
      | unchanged; if false, the current contents
      | of the text editor will be used to set
      | the slider position before it is hidden.
      |
      */
    pub fn hide_text_box(&mut self, discard_current_editor_contents: bool)  {
        
        todo!();
        /*
            pimpl->hideTextBox (discardCurrentEditorContents);
        */
    }
    
    /**
      | Tells the slider whether to keep sending
      | change messages while the user is dragging
      | the slider.
      | 
      | If set to true, a change message will
      | only be sent when the user has dragged
      | the slider and let go. If set to false
      | (the default), then messages will be
      | continuously sent as they drag it while
      | the mouse button is still held down.
      |
      */
    pub fn set_change_notification_only_on_release(&mut self, only_notify_on_release: bool)  {
        
        todo!();
        /*
            pimpl->sendChangeOnlyOnRelease = onlyNotifyOnRelease;
        */
    }
    
    /**
      | Returns true if setSliderSnapsToMousePosition()
      | has been enabled.
      |
      */
    pub fn get_slider_snaps_to_mouse_position(&self) -> bool {
        
        todo!();
        /*
            return pimpl->snapsToMousePos;
        */
    }
    
    /**
      | This lets you change whether the slider
      | thumb jumps to the mouse position when
      | you click.
      | 
      | By default, this is true. If it's false,
      | then the slider moves with relative
      | motion when you drag it.
      | 
      | This only applies to linear bars, and
      | won't affect two- or three- value sliders.
      |
      */
    pub fn set_slider_snaps_to_mouse_position(&mut self, should_snap_to_mouse: bool)  {
        
        todo!();
        /*
            pimpl->snapsToMousePos = shouldSnapToMouse;
        */
    }
    
    /**
      | If enabled, this gives the slider a pop-up
      | bubble which appears while the slider
      | is being dragged or hovered-over.
      | 
      | This can be handy if your slider doesn't
      | have a text-box, so that users can see
      | the value just when they're changing
      | it.
      | 
      | If you pass a component as the parentComponentToUse
      | parameter, the pop-up bubble will be
      | added as a child of that component when
      | it's needed. If you pass nullptr, the
      | pop-up will be placed on the desktop
      | instead (note that it's a transparent
      | window, so if you're using an OS that
      | can't do transparent windows you'll
      | have to add it to a parent component instead).
      | 
      | By default the popup display shown when
      | hovering will remain visible for 2 seconds,
      | but it is possible to change this by passing
      | a different hoverTimeout value. A value
      | of -1 will cause the popup to remain until
      | a mouseExit() occurs on the slider.
      |
      */
    pub fn set_popup_display_enabled(
        &mut self, 
        show_on_drag:  bool,
        show_on_hover: bool,
        parent:        *mut Component,
        hover_timeout: Option<i32>

    ) {

        let hover_timeout: i32 = hover_timeout.unwrap_or(2000);
        
        todo!();
        /*
            pimpl->showPopupOnDrag = showOnDrag;
        pimpl->showPopupOnHover = showOnHover;
        pimpl->parentForPopupDisplay = parent;
        pimpl->popupHoverTimeout = hoverTimeout;
        */
    }
    
    /**
      | If a popup display is enabled and is currently
      | visible, this returns the component
      | that is being shown, or nullptr if none
      | is currently in use. @see setPopupDisplayEnabled
      |
      */
    pub fn get_current_popup_display(&self) -> *mut Component {
        
        todo!();
        /*
            return pimpl->popupDisplay.get();
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            pimpl->lookAndFeelChanged (getLookAndFeel());
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            repaint(); pimpl->updateTextBoxEnablement();
        */
    }
    
    /**
      | Returns the slider's range.
      |
      */
    pub fn get_range(&self) -> Range<f64> {
        
        todo!();
        /*
            return { pimpl->normRange.start, pimpl->normRange.end };
        */
    }
    
    /**
      | Returns the current maximum value.
      | @see setRange, getRange
      |
      */
    pub fn get_maximum(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->normRange.end;
        */
    }
    
    /**
      | Returns the current minimum value.
      | @see setRange, getRange
      |
      */
    pub fn get_minimum(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->normRange.start;
        */
    }
    
    /**
      | Returns the current step-size for values.
      | @see setRange, getRange
      |
      */
    pub fn get_interval(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->normRange.interval;
        */
    }
    
    /**
      | Sets the limits that the slider's value
      | can take.
      | 
      | -----------
      | @param newMinimum
      | 
      | the lowest value allowed
      | ----------
      | @param newMaximum
      | 
      | the highest value allowed
      | ----------
      | @param newInterval
      | 
      | the steps in which the value is allowed
      | to increase - if this is not zero, the
      | value will always be (newMinimum + (newInterval
      | * an integer)).
      |
      */
    pub fn set_range(
        &mut self, 
        new_min: f64,
        new_max: f64,
        new_int: Option<f64>

    ) {

        let new_int: f64 = new_int.unwrap_or(0.0);
        
        todo!();
        /*
            pimpl->setRange (newMin, newMax, newInt);
        */
    }
    
    /**
      | Sets the limits that the slider's value
      | can take.
      | 
      | -----------
      | @param newRange
      | 
      | the range to allow
      | ----------
      | @param newInterval
      | 
      | the steps in which the value is allowed
      | to increase - if this is not zero, the
      | value will always be (newMinimum + (newInterval
      | * an integer)).
      |
      */
    pub fn set_range_with_range(
        &mut self, 
        new_range: Range<f64>,
        new_int:   f64

    ) {
        
        todo!();
        /*
            pimpl->setRange (newRange.getStart(), newRange.getEnd(), newInt);
        */
    }
    
    /**
      | Sets a NormalisableRange to use for
      | the Slider values.
      | 
      | -----------
      | @param newNormalisableRange
      | 
      | the NormalisableRange to use
      |
      */
    pub fn set_normalisable_range(&mut self, new_range: NormalisableRange<f64>)  {
        
        todo!();
        /*
            pimpl->setNormalisableRange (newRange);
        */
    }
    
    /**
      | Returns the slider's current value.
      |
      */
    pub fn get_value(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->getValue();
        */
    }
    
    /**
      | Returns the Value object that represents
      | the slider's current position.
      | 
      | You can use this Value object to connect
      | the slider's position to external values
      | or setters, either by taking a copy of
      | the Value, or by using Value::referTo()
      | to make it point to your own Value object.
      | @see Value, getMaxValue, getMinValueObject
      |
      */
    pub fn get_value_object(&mut self) -> &mut Value {
        
        todo!();
        /*
            return pimpl->currentValue;
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this returns the lower of its values.
      | 
      | You can use this Value object to connect
      | the slider's position to external values
      | or setters, either by taking a copy of
      | the Value, or by using Value::referTo()
      | to make it point to your own Value object.
      | @see Value, getMinValue, getMaxValueObject
      |
      */
    pub fn get_min_value_object(&mut self) -> &mut Value {
        
        todo!();
        /*
            return pimpl->valueMin;
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this returns the higher of its values.
      | 
      | You can use this Value object to connect
      | the slider's position to external values
      | or setters, either by taking a copy of
      | the Value, or by using Value::referTo()
      | to make it point to your own Value object.
      | @see Value, getMaxValue, getMinValueObject
      |
      */
    pub fn get_max_value_object(&mut self) -> &mut Value {
        
        todo!();
        /*
            return pimpl->valueMax;
        */
    }
    
    /**
      | Changes the slider's current value.
      | 
      | This will trigger a callback to Slider::SliderListener::sliderValueChanged()
      | for any listeners that are registered,
      | and will synchronously call the valueChanged()
      | method in case subclasses want to handle
      | it.
      | 
      | -----------
      | @param newValue
      | 
      | the new value to set - this will be restricted
      | by the minimum and maximum range, and
      | will be snapped to the nearest interval
      | if one has been set
      | ----------
      | @param notification
      | 
      | can be one of the NotificationType values,
      | to request a synchronous or asynchronous
      | call to the valueChanged() method of
      | any Slider::Listeners that are registered.
      | A notification will only be sent if the
      | Slider's value has changed.
      |
      */
    pub fn set_value(
        &mut self, 
        new_value:    f64,
        notification: Option<NotificationType>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            pimpl->setValue (newValue, notification);
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this returns the lower of its values.
      | 
      | For a two-value slider, the values are
      | controlled with getMinValue() and
      | getMaxValue().
      | 
      | A slider with three values also uses
      | the normal getValue() and setValue()
      | methods to control the middle value.
      | 
      | @see setMinValue, getMaxValue, TwoValueHorizontal,
      | TwoValueVertical, ThreeValueHorizontal,
      | ThreeValueVertical
      |
      */
    pub fn get_min_value(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->getMinValue();
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this returns the higher of its values.
      | 
      | For a two-value slider, the values are
      | controlled with getMinValue() and
      | getMaxValue().
      | 
      | A slider with three values also uses
      | the normal getValue() and setValue()
      | methods to control the middle value.
      | 
      | @see getMinValue, TwoValueHorizontal,
      | TwoValueVertical, ThreeValueHorizontal,
      | ThreeValueVertical
      |
      */
    pub fn get_max_value(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->getMaxValue();
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this sets the lower of its values.
      | 
      | This will trigger a callback to Slider::SliderListener::sliderValueChanged()
      | for any listeners that are registered,
      | and will synchronously call the valueChanged()
      | method in case subclasses want to handle
      | it.
      | 
      | -----------
      | @param newValue
      | 
      | the new value to set - this will be restricted
      | by the minimum and maximum range, and
      | will be snapped to the nearest interval
      | if one has been set.
      | ----------
      | @param notification
      | 
      | can be one of the NotificationType values,
      | to request a synchronous or asynchronous
      | call to the valueChanged() method of
      | any Slider::Listeners that are registered.
      | A notification will only be sent if this
      | value has changed.
      | ----------
      | @param allowNudgingOfOtherValues
      | 
      | if false, this value will be restricted
      | to being below the max value (in a two-value
      | slider) or the mid value (in a three-value
      | slider). If true, then if this value
      | goes beyond those values, it will push
      | them along with it. @see getMinValue,
      | setMaxValue, setValue
      |
      */
    pub fn set_min_value(
        &mut self, 
        new_value:                     f64,
        notification:                  Option<NotificationType>,
        allow_nudging_of_other_values: Option<bool>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        let allow_nudging_of_other_values: bool = allow_nudging_of_other_values.unwrap_or(false);
        
        todo!();
        /*
            pimpl->setMinValue (newValue, notification, allowNudgingOfOtherValues);
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this sets the lower of its values.
      | 
      | This will trigger a callback to Slider::SliderListener::sliderValueChanged()
      | for any listeners that are registered,
      | and will synchronously call the valueChanged()
      | method in case subclasses want to handle
      | it.
      | 
      | -----------
      | @param newValue
      | 
      | the new value to set - this will be restricted
      | by the minimum and maximum range, and
      | will be snapped to the nearest interval
      | if one has been set.
      | ----------
      | @param notification
      | 
      | can be one of the NotificationType values,
      | to request a synchronous or asynchronous
      | call to the valueChanged() method of
      | any Slider::Listeners that are registered.
      | A notification will only be sent if this
      | value has changed.
      | ----------
      | @param allowNudgingOfOtherValues
      | 
      | if false, this value will be restricted
      | to being above the min value (in a two-value
      | slider) or the mid value (in a three-value
      | slider). If true, then if this value
      | goes beyond those values, it will push
      | them along with it. @see getMaxValue,
      | setMinValue, setValue
      |
      */
    pub fn set_max_value(
        &mut self, 
        new_value:                     f64,
        notification:                  Option<NotificationType>,
        allow_nudging_of_other_values: Option<bool>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        let allow_nudging_of_other_values: bool = allow_nudging_of_other_values.unwrap_or(false);
        
        todo!();
        /*
            pimpl->setMaxValue (newValue, notification, allowNudgingOfOtherValues);
        */
    }
    
    /**
      | For a slider with two or three thumbs,
      | this sets the minimum and maximum thumb
      | positions.
      | 
      | This will trigger a callback to Slider::SliderListener::sliderValueChanged()
      | for any listeners that are registered,
      | and will synchronously call the valueChanged()
      | method in case subclasses want to handle
      | it.
      | 
      | -----------
      | @param newMinValue
      | 
      | the new minimum value to set - this will
      | be snapped to the nearest interval if
      | one has been set.
      | ----------
      | @param newMaxValue
      | 
      | the new minimum value to set - this will
      | be snapped to the nearest interval if
      | one has been set.
      | ----------
      | @param notification
      | 
      | can be one of the NotificationType values,
      | to request a synchronous or asynchronous
      | call to the valueChanged() method of
      | any Slider::Listeners that are registered.
      | A notification will only be sent if one
      | or more of the values has changed. @see
      | setMaxValue, setMinValue, setValue
      |
      */
    pub fn set_min_and_max_values(
        &mut self, 
        new_min_value: f64,
        new_max_value: f64,
        notification:  Option<NotificationType>

    ) {

        let notification: NotificationType = notification.unwrap_or(NotificationType::sendNotificationAsync);
        
        todo!();
        /*
            pimpl->setMinAndMaxValues (newMinValue, newMaxValue, notification);
        */
    }
    
    /**
      | This lets you choose whether double-clicking
      | or single-clicking with a specified
      | key modifier moves the slider to a given
      | position.
      | 
      | By default this is turned off, but it's
      | handy if you want either of these actions
      | to act as a quick way of resetting a slider.
      | Just pass in the value you want it to go
      | to when double-clicked. By default
      | the key modifier is the alt key but you
      | can pass in another key modifier, or
      | none to disable this behaviour.
      | 
      | @see getDoubleClickReturnValue
      |
      */
    pub fn set_double_click_return_value(
        &mut self, 
        is_double_click_enabled:      bool,
        value_to_set_on_double_click: f64,
        mods:                         Option<ModifierKeysFlags>

    ) {

        let mods: ModifierKeysFlags = mods.unwrap_or(ModifierKeysFlags::altModifier);
        
        todo!();
        /*
            pimpl->doubleClickToValue = isDoubleClickEnabled;
        pimpl->doubleClickReturnValue = valueToSetOnDoubleClick;
        pimpl->singleClickModifiers = mods;
        */
    }
    
    /**
      | Returns the values last set by setDoubleClickReturnValue()
      | method. @see setDoubleClickReturnValue
      |
      */
    pub fn get_double_click_return_value(&self) -> f64 {
        
        todo!();
        /*
            return pimpl->doubleClickReturnValue;
        */
    }
    
    /**
      | Returns true if double-clicking to
      | reset to a default value is enabled.
      | @see setDoubleClickReturnValue
      |
      */
    pub fn is_double_click_return_enabled(&self) -> bool {
        
        todo!();
        /*
            return pimpl->doubleClickToValue;
        */
    }
    
    /**
      | This can be called to force the text box
      | to update its contents. (Not normally
      | needed, as this is done automatically).
      |
      */
    pub fn update_text(&mut self)  {
        
        todo!();
        /*
            pimpl->updateText();
        */
    }
    
    /**
      | Sets a suffix to append to the end of the
      | numeric value when it's displayed as
      | a string.
      | 
      | This is used by the default implementation
      | of getTextFromValue(), and is just
      | appended to the numeric value. For more
      | advanced formatting, you can override
      | getTextFromValue() and do something
      | else.
      |
      */
    pub fn set_text_value_suffix(&mut self, suffix: &String)  {
        
        todo!();
        /*
            pimpl->setTextValueSuffix (suffix);
        */
    }
    
    /**
      | Returns the suffix that was set by setTextValueSuffix().
      |
      */
    pub fn get_text_value_suffix(&self) -> String {
        
        todo!();
        /*
            return pimpl->textSuffix;
        */
    }
    
    pub fn get_text_from_value(&mut self, v: f64) -> String {
        
        todo!();
        /*
            auto getText = [this] (double val)
        {
            if (textFromValueFunction != nullptr)
                return textFromValueFunction (val);

            if (getNumDecimalPlacesToDisplay() > 0)
                return String (val, getNumDecimalPlacesToDisplay());

            return String (roundToInt (val));
        };

        return getText (v) + getTextValueSuffix();
        */
    }
    
    pub fn get_value_from_text(&mut self, text: &String) -> f64 {
        
        todo!();
        /*
            auto t = text.trimStart();

        if (t.endsWith (getTextValueSuffix()))
            t = t.substring (0, t.length() - getTextValueSuffix().length());

        if (valueFromTextFunction != nullptr)
            return valueFromTextFunction (t);

        while (t.startsWithChar ('+'))
            t = t.substring (1).trimStart();

        return t.initialSectionContainingOnly ("0123456789.,-")
                .getDoubleValue();
        */
    }
    
    pub fn proportion_of_length_to_value(&mut self, proportion: f64) -> f64 {
        
        todo!();
        /*
            return pimpl->normRange.convertFrom0to1 (proportion);
        */
    }
    
    pub fn value_to_proportion_of_length(&mut self, value: f64) -> f64 {
        
        todo!();
        /*
            return pimpl->normRange.convertTo0to1 (value);
        */
    }
    
    pub fn snap_value(&mut self, 
        attempted_value: f64,
        _1:              SliderDragMode) -> f64 {
        
        todo!();
        /*
            return attemptedValue;
        */
    }
    
    /**
      | Returns the best number of decimal places
      | to use when displaying this slider's
      | value.
      | 
      | It calculates the fewest decimal places
      | needed to represent numbers with the
      | slider's interval setting.
      | 
      | @see setNumDecimalPlacesToDisplay
      |
      */
    pub fn get_num_decimal_places_to_display(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->numDecimalPlaces;
        */
    }
    
    /**
      | Modifies the best number of decimal
      | places to use when displaying this slider's
      | value.
      | 
      | @see getNumDecimalPlacesToDisplay
      |
      */
    pub fn set_num_decimal_places_to_display(&mut self, decimal_places_to_display: i32)  {
        
        todo!();
        /*
            pimpl->numDecimalPlaces = decimalPlacesToDisplay;
        updateText();
        */
    }
    
    /**
      | Returns a number to indicate which thumb
      | is currently being dragged by the mouse.
      | 
      | This will return 0 for the main thumb,
      | 1 for the minimum-value thumb, 2 for
      | the maximum-value thumb, or -1 if none
      | is currently down.
      |
      */
    pub fn get_thumb_being_dragged(&self) -> i32 {
        
        todo!();
        /*
            return pimpl->sliderBeingDragged;
        */
    }
    
    pub fn started_dragging(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn stopped_dragging(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn value_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | If this is set to true, then right-clicking
      | on the slider will pop-up a menu to let
      | the user change the way it works.
      | 
      | By default this is turned off, but when
      | turned on, the menu will include things
      | like velocity sensitivity, and for
      | rotary sliders, whether they use a linear
      | or rotary mouse-drag to move them.
      |
      */
    pub fn set_popup_menu_enabled(&mut self, menu_enabled: bool)  {
        
        todo!();
        /*
            pimpl->menuEnabled = menuEnabled;
        */
    }
    
    /**
      | This can be used to stop the mouse scroll-wheel
      | from moving the slider.
      | 
      | By default it's enabled.
      |
      */
    pub fn set_scroll_wheel_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
            pimpl->scrollWheelEnabled = enabled;
        */
    }
    
    /**
      | Returns true if the scroll wheel can
      | move the slider.
      |
      */
    pub fn is_scroll_wheel_enabled(&self) -> bool {
        
        todo!();
        /*
            return pimpl->scrollWheelEnabled;
        */
    }
    
    /**
      | True if the slider moves horizontally.
      |
      */
    pub fn is_horizontal(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isHorizontal();
        */
    }
    
    /**
      | True if the slider moves vertically.
      |
      */
    pub fn is_vertical(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isVertical();
        */
    }
    
    /**
      | True if the slider is in a rotary mode.
      |
      */
    pub fn is_rotary(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isRotary();
        */
    }
    
    /**
      | True if the slider is in a linear bar mode.
      |
      */
    pub fn is_bar(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isBar();
        */
    }
    
    /**
      | True if the slider has two thumbs.
      |
      */
    pub fn is_two_value(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isTwoValue();
        */
    }
    
    /**
      | True if the slider has three thumbs.
      |
      */
    pub fn is_three_value(&self) -> bool {
        
        todo!();
        /*
            return pimpl->isThreeValue();
        */
    }
    
    /**
      | Returns the X or Y coordinate of a value
      | along the slider's length.
      | 
      | If the slider is horizontal, this will
      | be the X coordinate of the given value,
      | relative to the left of the slider. If
      | it's vertical, then this will be the
      | Y coordinate, relative to the top of
      | the slider.
      | 
      | If the slider is rotary, this will throw
      | an assertion and return 0. If the value
      | is out-of-range, it will be constrained
      | to the length of the slider.
      |
      */
    pub fn get_position_of_value(&self, value: f64) -> f32 {
        
        todo!();
        /*
            return pimpl->getPositionOfValue (value);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            pimpl->paint (g, getLookAndFeel());
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            pimpl->resized (getLookAndFeel());
        */
    }
    
    pub fn focus_of_child_component_changed(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            pimpl->mouseDown (e);
        */
    }
    
    pub fn mouse_up(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            pimpl->mouseUp();
        */
    }
    
    pub fn mouse_move(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            pimpl->mouseMove();
        */
    }
    
    pub fn mouse_exit(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            pimpl->mouseExit();
        */
    }

    /**
      | If popup display is enabled and set to show
      | on mouse hover, this makes sure it is shown
      | when dragging the mouse over a slider and
      | releasing
      */
    pub fn mouse_enter(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            pimpl->mouseMove();
        */
    }
    
    pub fn modifier_keys_changed(&mut self, modifiers: &ModifierKeys)  {
        
        todo!();
        /*
            if (isEnabled())
            pimpl->modifierKeysChanged (modifiers);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled())
            pimpl->mouseDrag (e);
        */
    }
    
    pub fn mouse_double_click(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            if (isEnabled())
            pimpl->mouseDoubleClick();
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        e:     &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            if (! (isEnabled() && pimpl->mouseWheelMove (e, wheel)))
            Component::mouseWheelMove (e, wheel);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return std::make_unique<SliderAccessibilityHandler> (*this);
        */
    }
}

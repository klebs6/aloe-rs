crate::ix!();

#[no_copy]
#[leak_detector]
pub struct AccessibilityActionComponent<'a> {
    base:                    Component<'a>,
    base2:                   Timer,
    enabled_toggle:          ToggleButton<'a>,
    flash_area:              Rectangle<i32>,
    start_time:              u32, // default = 0
    default_colour:          Colour, // default = Colours::lightgrey
    flash_colour:            Colour, // default = defaultColour
    custom_widget_component: &'a mut CustomWidgetComponent<'a>,
    press:                   Box<AccessibilityActionComponent<'a>>, //{ customWidgetComponent, "Press",     true };
    toggle:                  Box<AccessibilityActionComponent<'a>>, //{ customWidgetComponent, "Toggle",    false };
    focus:                   Box<AccessibilityActionComponent<'a>>, //{ customWidgetComponent, "Focus",     true };
    show_menu:               Box<AccessibilityActionComponent<'a>>, //{ customWidgetComponent, "Show menu", false };
}

impl<'a> AccessibilityActionComponent<'a> {

    pub const flashTimeMs: i32 = 500;

    pub fn new(
        owner:         &mut CustomWidgetComponent,
        action_name:   &String,
        initial_state: bool) -> Self {
    
        todo!();
        /*


            enabledToggle.setButtonText (actionName);
                    enabledToggle.onClick = [&owner] { owner.accessibleComponent.invalidateAccessibilityHandler(); };
                    enabledToggle.setToggleState (initialState, dontSendNotification);

                    addAndMakeVisible (enabledToggle);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();

                    flashArea = bounds.removeFromRight (bounds.getHeight()).reduced (5);
                    bounds.removeFromRight (5);
                    enabledToggle.setBounds (bounds);
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (flashColour);
                    g.fillRoundedRectangle (flashArea.toFloat(), 5.0f);
        */
    }
    
    pub fn on_action(&mut self)  {
        
        todo!();
        /*
            if (isTimerRunning())
                        reset();

                    startTime = Time::getMillisecondCounter();
                    startTimer (5);
        */
    }
    
    pub fn is_action_enabled(&self) -> bool {
        
        todo!();
        /*
            return enabledToggle.getToggleState();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            const auto alpha = [this]
                    {
                        const auto progress =  static_cast<float> (Time::getMillisecondCounter() - startTime) / (flashTimeMs / 2);

                        return progress > 1.0f ? 2.0f - progress
                                               : progress;
                    }();

                    if (alpha < 0.0f)
                    {
                        reset();
                        return;
                    }

                    flashColour = defaultColour.overlaidWith (Colours::yellow.withAlpha (alpha));
                    repaint();
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            stopTimer();
                    flashColour = defaultColour;
                    repaint();
        */
    }
}

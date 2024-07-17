crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ProgressBar.h]

/**
  | A progress bar component.
  | 
  | To use this, just create one and make
  | it visible. It'll run its own timer to
  | keep an eye on a variable that you give
  | it, and will automatically redraw itself
  | when the variable changes.
  | 
  | If using LookAndFeel_V4 a circular
  | spinning progress bar will be drawn
  | if the width and height of the ProgressBar
  | are equal, otherwise the standard,
  | linear ProgressBar will be drawn.
  | 
  | For an easy way of running a background
  | task with a dialog box showing its progress,
  | see the ThreadWithProgressWindow
  | class.
  | 
  | @see ThreadWithProgressWindow
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ProgressBar<'a> {
    base:               Component<'a>,
    base2:              SettableTooltipClient,
    base3:              Timer,
    progress:           &'a mut f64,
    current_value:      f64,
    display_percentage: bool,
    displayed_message:  String,
    current_message:    String,
    last_callback_time: u32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ProgressBar.cpp]
impl<'a> ProgressBar<'a> {
    
    /**
      | Creates a ProgressBar.
      | 
      | -----------
      | @param progress
      | 
      | pass in a reference to a double that you're
      | going to update with your task's progress.
      | The ProgressBar will monitor the value
      | of this variable and will redraw itself
      | when the value changes. The range is
      | from 0 to 1.0 and Aloe
      | 
      | LookAndFeel classes will draw a spinning
      | animation for values outside this range.
      | Obviously you'd better be careful not
      | to delete this variable while the ProgressBar
      | still exists!
      |
      */
    pub fn new(progress: &mut f64) -> Self {
    
        todo!();
        /*
        : progress(progress_),
        : display_percentage(true),
        : last_callback_time(0),

            currentValue = jlimit (0.0, 1.0, progress);
        */
    }
    
    /**
      | Turns the percentage display on or off.
      | 
      | By default this is on, and the progress
      | bar will display a text string showing
      | its current percentage.
      |
      */
    pub fn set_percentage_display(&mut self, should_display_percentage: bool)  {
        
        todo!();
        /*
            displayPercentage = shouldDisplayPercentage;
        repaint();
        */
    }
    
    /**
      | Gives the progress bar a string to display
      | inside it.
      | 
      | If you call this, it will turn off the
      | percentage display. @see setPercentageDisplay
      |
      */
    pub fn set_text_to_display(&mut self, text: &String)  {
        
        todo!();
        /*
            displayPercentage = false;
        displayedMessage = text;
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            setOpaque (getLookAndFeel().isProgressBarOpaque (*this));
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            lookAndFeelChanged();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            String text;

        if (displayPercentage)
        {
            if (currentValue >= 0 && currentValue <= 1.0)
                text << roundToInt (currentValue * 100.0) << '%';
        }
        else
        {
            text = displayedMessage;
        }

        getLookAndFeel().drawProgressBar (g, *this,
                                          getWidth(), getHeight(),
                                          currentValue, text);
        */
    }
    
    pub fn visibility_changed(&mut self)  {
        
        todo!();
        /*
            if (isVisible())
            startTimer (30);
        else
            stopTimer();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            double newProgress = progress;

        const uint32 now = Time::getMillisecondCounter();
        const int timeSinceLastCallback = (int) (now - lastCallbackTime);
        lastCallbackTime = now;

        if (currentValue != newProgress
             || newProgress < 0 || newProgress >= 1.0
             || currentMessage != displayedMessage)
        {
            if (currentValue < newProgress
                 && newProgress >= 0 && newProgress < 1.0
                 && currentValue >= 0 && currentValue < 1.0)
            {
                newProgress = jmin (currentValue + 0.0008 * timeSinceLastCallback,
                                    newProgress);
            }

            currentValue = newProgress;
            currentMessage = displayedMessage;
            repaint();

            if (auto* handler = getAccessibilityHandler())
                handler->notifyAccessibilityEvent (AccessibilityEvent::valueChanged);
        }
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class ProgressBarAccessibilityHandler  : public AccessibilityHandler
        {
        
            explicit ProgressBarAccessibilityHandler (ProgressBar& progressBarToWrap)
                : AccessibilityHandler (progressBarToWrap,
                                        AccessibilityRole::progressBar,
                                        AccessibilityActions{},
                                        AccessibilityHandler::Interfaces { std::make_unique<ValueInterface> (progressBarToWrap) }),
                  progressBar (progressBarToWrap)
            {
            }

            String getHelp() const override   { return progressBar.getTooltip(); }

        
            class ValueInterface  : public AccessibilityRangedNumericValueInterface
            {
            
                explicit ValueInterface (ProgressBar& progressBarToWrap)
                    : progressBar (progressBarToWrap)
                {
                }

                bool isReadOnly() const override                { return true; }
                void setValue (double) override                 { jassertfalse; }
                double getCurrentValue() const override         { return progressBar.progress; }
                AccessibleValueRange getRange() const override  { return { { 0.0, 1.0 }, 0.001 }; }

            
                ProgressBar& progressBar;

                
                ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ValueInterface)
            };

            ProgressBar& progressBar;

            
            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ProgressBarAccessibilityHandler)
        };

        return std::make_unique<ProgressBarAccessibilityHandler> (*this);
        */
    }
}

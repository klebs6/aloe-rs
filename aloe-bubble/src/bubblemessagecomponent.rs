crate::ix!();

pub const bubble_paddingx: f32 = 20.0;
pub const bubble_paddingy: f32 = 14.0;

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_BubbleMessageComponent.h]

/**
  | A speech-bubble component that displays
  | a short message.
  | 
  | This can be used to show a message with
  | the tail of the speech bubble pointing
  | to a particular component or location
  | on the screen.
  | 
  | @see BubbleComponent
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct BubbleMessageComponent<'a> {
    base:                BubbleComponent<'a>,
    base2:               Timer,
    fade_out_length:     i32,
    mouse_click_counter: i32,
    text_layout:         TextLayout,
    expiry_time:         i64,
    delete_after_use:    bool,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_extra/misc/aloe_BubbleMessageComponent.cpp]
impl<'a> BubbleMessageComponent<'a> {
    
    /**
      | Creates a bubble component.
      | 
      | After creating one a BubbleComponent,
      | do the following:
      | 
      | - add it to an appropriate parent component,
      | or put it on the desktop with Component::addToDesktop
      | (0).
      | 
      | - use the showAt() method to show a message.
      | 
      | - it will make itself invisible after
      | it times-out (and can optionally also
      | delete itself), or you can reuse it somewhere
      | else by calling showAt() again.
      |
      */
    pub fn new(fade_out_length_ms: Option<i32>) -> Self {

        let fade_out_length_ms: i32 =
                 fade_out_length_ms.unwrap_or(150);
    
        todo!();
        /*
        : fade_out_length(fadeOutLengthMs),
        : mouse_click_counter(0),
        : expiry_time(0),
        : delete_after_use(false),

        
        */
    }
    
    /**
      | Shows a message bubble at a particular
      | position.
      | 
      | This shows the bubble with its stem pointing
      | to the given location (coordinates
      | being relative to its parent component).
      | 
      | -----------
      | @param position
      | 
      | the coords of the object to point to
      | ----------
      | @param message
      | 
      | the text to display
      | ----------
      | @param numMillisecondsBeforeRemoving
      | 
      | how long to leave it on the screen before
      | removing itself from its parent component.
      | If this is 0 or less, it will stay there
      | until manually removed.
      | ----------
      | @param removeWhenMouseClicked
      | 
      | if this is true, the bubble will disappear
      | as soon as a mouse button is pressed (anywhere
      | on the screen)
      | ----------
      | @param deleteSelfAfterUse
      | 
      | if true, then the component will delete
      | itself after it becomes invisible
      |
      */
    pub fn show_at_particular_position(
        &mut self, 
        pos:                              &Rectangle<i32>,
        text:                             &AttributedString,
        num_milliseconds_before_removing: i32,
        remove_when_mouse_clicked:        Option<bool>,
        delete_self_after_use:            Option<bool>)  {

        let remove_when_mouse_clicked: bool = remove_when_mouse_clicked.unwrap_or(true);
        let delete_self_after_use:     bool = delete_self_after_use.unwrap_or(false);
        
        todo!();
        /*
            createLayout (text);
        setPosition (pos);
        init (numMillisecondsBeforeRemoving, removeWhenMouseClicked, deleteSelfAfterUse);
        */
    }
    
    /**
      | Shows a message bubble next to a particular
      | component.
      | 
      | This shows the bubble with its stem pointing
      | at the given component.
      | 
      | -----------
      | @param component
      | 
      | the component that you want to point
      | at
      | ----------
      | @param message
      | 
      | the text to display
      | ----------
      | @param numMillisecondsBeforeRemoving
      | 
      | how long to leave it on the screen before
      | removing itself from its parent component.
      | If this is 0 or less, it will stay there
      | until manually removed.
      | ----------
      | @param removeWhenMouseClicked
      | 
      | if this is true, the bubble will disappear
      | as soon as a mouse button is pressed (anywhere
      | on the screen)
      | ----------
      | @param deleteSelfAfterUse
      | 
      | if true, then the component will delete
      | itself after it becomes invisible
      |
      */
    pub fn show_at_particular_component(
        &mut self, 
        component:                        *mut Component<'a>,
        text:                             &AttributedString,
        num_milliseconds_before_removing: i32,
        remove_when_mouse_clicked:        Option<bool>,
        delete_self_after_use:            Option<bool>)  {

        let remove_when_mouse_clicked: bool = remove_when_mouse_clicked.unwrap_or(true);
        let delete_self_after_use:     bool = delete_self_after_use.unwrap_or(false);
        
        todo!();
        /*
            createLayout (text);
        setPosition (component);
        init (numMillisecondsBeforeRemoving, removeWhenMouseClicked, deleteSelfAfterUse);
        */
    }
    
    pub fn create_layout(&mut self, text: &AttributedString)  {
        
        todo!();
        /*
            textLayout.createLayoutWithBalancedLineLengths (text, 256);
        */
    }
    
    pub fn init(&mut self, 
        num_milliseconds_before_removing: i32,
        remove_when_mouse_clicked:        bool,
        delete_self_after_use:            bool)  {
        
        todo!();
        /*
            setAlpha (1.0f);
        setVisible (true);
        deleteAfterUse = deleteSelfAfterUse;

        expiryTime = numMillisecondsBeforeRemoving > 0
                        ? (Time::getMillisecondCounter() + (uint32) numMillisecondsBeforeRemoving) : 0;

        mouseClickCounter = Desktop::getInstance().getMouseButtonClickCounter();

        if (! (removeWhenMouseClicked && isShowing()))
            mouseClickCounter += 0xfffff;

        startTimer (77);
        repaint();
        */
    }
    
    pub fn get_content_size(&mut self, 
        w: &mut i32,
        h: &mut i32)  {
        
        todo!();
        /*
            w = (int) (bubblePaddingX + textLayout.getWidth());
        h = (int) (bubblePaddingY + textLayout.getHeight());
        */
    }
    
    pub fn paint_content(&mut self, 
        g: &mut Graphics,
        w: i32,
        h: i32)  {
        
        todo!();
        /*
            g.setColour (findColour (TooltipWindow::textColourId));

        textLayout.draw (g, Rectangle<float> (bubblePaddingX / 2.0f, bubblePaddingY / 2.0f,
                                              (float) w - bubblePaddingX, (float) h - bubblePaddingY));
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (Desktop::getInstance().getMouseButtonClickCounter() > mouseClickCounter)
            hide (false);
        else if (expiryTime != 0 && Time::getMillisecondCounter() > expiryTime)
            hide (true);
        */
    }
    
    pub fn hide(&mut self, fade_out: bool)  {
        
        todo!();
        /*
            stopTimer();

        if (fadeOut)
            Desktop::getInstance().getAnimator().fadeOut (this, fadeOutLength);
        else
            setVisible (false);

        if (deleteAfterUse)
            delete this;
        */
    }
}

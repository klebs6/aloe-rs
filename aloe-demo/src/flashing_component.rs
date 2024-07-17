crate::ix!();

/**
  | Simple component that can be triggered
  | to flash.
  | 
  | The flash will then fade using a Timer
  | to repaint itself and will send a change
  | message once it is finished.
  |
  */
#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct FlashingComponent<'a> {
    base:        Component<'a>,
    base2:       MessageListener,
    base3:       ChangeBroadcaster<'a>,
    base4:       Timer,
    flash_alpha: f32, // default = 0.0f
    colour:      Colour, // default = { Colours::red  }
}

impl<'a> FlashingComponent<'a> {

    pub fn start_flashing(&mut self)  {
        
        todo!();
        /*
            flashAlpha = 1.0f;
            startTimerHz (25);
        */
    }

    /**
      | Stops this component flashing without
      | sending a change message.
      |
      */
    pub fn stop_flashing(&mut self)  {
        
        todo!();
        /*
            flashAlpha = 0.0f;
            stopTimer();
            repaint();
        */
    }

    /**
      | Sets the colour of the component.
      |
      */
    pub fn set_flash_colour(&mut self, new_colour: Colour)  {
        
        todo!();
        /*
            colour = newColour;
            repaint();
        */
    }

    /**
      | Draws our component.
      |
      */
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (colour.overlaidWith (Colours::white.withAlpha (flashAlpha)));
            g.fillEllipse (getLocalBounds().toFloat());
        */
    }

    /**
      | Custom mouse handler to trigger a flash.
      |
      */
    pub fn mouse_down(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            startFlashing();
        */
    }

    /**
      | Message listener callback used to change
      | our colour
      |
      */
    pub fn handle_message(&mut self, message: &Message)  {
        
        todo!();
        /*
            setFlashColour (ColourMessage::getColour (message));
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            // Reduce the alpha level of the flash slightly so it fades out
            flashAlpha -= 0.075f;

            if (flashAlpha < 0.05f)
            {
                stopFlashing();
                sendChangeMessage();
                // Once we've finished flashing send a change message to trigger the next component to flash
            }

            repaint();
        */
    }
}

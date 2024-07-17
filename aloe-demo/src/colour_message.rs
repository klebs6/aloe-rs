crate::ix!();

/**
  | Simple message that holds a Colour.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ColourMessage {
    base:   Message,
    colour: Colour,
}

impl ColourMessage {

    pub fn new(col: Colour) -> Self {
    
        todo!();
        /*
        : colour(col),
        */
    }

    /**
      | Returns the colour of a ColourMessage
      | of white if the message is not a ColourMessage.
      |
      */
    pub fn get_colour(message: &Message) -> Colour {
        
        todo!();
        /*
            if (auto* cm = dynamic_cast<const ColourMessage*> (&message))
                return cm->colour;

            return Colours::white;
        */
    }
}



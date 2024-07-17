crate::ix!();

/** 
  | A set of colour IDs to use to change the
  | colour of various aspects of the bubble
  | component.
  |
  | These constants can be used either via the
  | Component::setColour(), or
  | LookAndFeel::setColour() methods.
  |
  | @see Component::setColour,
  | Component::findColour,
  | LookAndFeel::setColour,
  | LookAndFeel::findColour
  */
pub enum BubbleComponentColourIds
{
    /**
      | A background colour to fill the bubble
      | with
      |
      */
    backgroundColourId = 0x1000af0, 

    /**
      | The colour to use for an outline around
      | the bubble.
      |
      */
    outlineColourId    = 0x1000af1  
}

/**
  | This abstract base class is implemented
  | by LookAndFeel classes.
  |
  */
pub trait BubbleComponentLookAndFeelMethods {

    fn draw_bubble(
        &mut self, 
        _0:              &mut Graphics,
        _1:              &mut BubbleComponent,
        position_of_tip: &Point<f32>,
        body:            &Rectangle<f32>
    );
}

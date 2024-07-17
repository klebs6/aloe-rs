crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/effects/aloe_ImageEffectFilter.h]

/**
  | A graphical effect filter that can be
  | applied to components.
  | 
  | An ImageEffectFilter can be applied
  | to the image that a component paints
  | before it hits the screen.
  | 
  | This is used for adding effects like
  | shadows, blurs, etc.
  | 
  | @see Component::setComponentEffect
  | 
  | @tags{Graphics}
  |
  */
pub trait ImageEffectFilter {
    
    /**
      | Overridden to render the effect.
      | 
      | The implementation of this method must
      | use the image that is passed in as its
      | source, and should render its output
      | to the graphics context passed in.
      | 
      | -----------
      | @param sourceImage
      | 
      | the image that the source component
      | has just rendered with its paint() method.
      | The image may or may not have an alpha
      | channel, depending on whether the component
      | is opaque.
      | ----------
      | @param destContext
      | 
      | the graphics context to use to draw the
      | resultant image.
      | ----------
      | @param scaleFactor
      | 
      | a scale factor that has been applied
      | to the image - e.g. if this is 2, then the
      | image is actually scaled-up to twice
      | the original resolution
      | ----------
      | @param alpha
      | 
      | the alpha with which to draw the resultant
      | image to the target context
      |
      */
    fn apply_effect(
        &mut self, 
        source_image: &mut Image,
        dest_context: &mut Graphics,
        scale_factor: f32,
        alpha:        f32);
}

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_LowLevelGraphicsSoftwareRenderer.h]

/**
  | A lowest-common-denominator implementation
  | of LowLevelGraphicsContext that does
  | all its rendering in memory.
  | 
  | User code is not supposed to create instances
  | of this class directly - do all your rendering
  | via the Graphics class instead.
  | 
  | @tags{Graphics}
  |
  */
#[no_copy]
#[leak_detector]
pub struct LowLevelGraphicsSoftwareRenderer {
    base: StackBasedLowLevelGraphicsContext<SoftwareRendererSavedState>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_LowLevelGraphicsSoftwareRenderer.cpp]
impl LowLevelGraphicsSoftwareRenderer {
    
    /**
      | Creates a context to render into an image.
      |
      */
    pub fn new_from_image(image: &Image) -> Self {
    
        todo!();
        /*


            : RenderingHelpers::StackBasedLowLevelGraphicsContext<RenderingHelpers::SoftwareRendererSavedState>
            (new RenderingHelpers::SoftwareRendererSavedState (image, image.getBounds()))
        */
    }
    
    /**
      | Creates a context to render into a clipped
      | subsection of an image.
      |
      */
    pub fn new(
        image:        &Image,
        origin:       Point<i32>,
        initial_clip: &RectangleList<i32>) -> Self {
    
        todo!();
        /*


            : RenderingHelpers::StackBasedLowLevelGraphicsContext<RenderingHelpers::SoftwareRendererSavedState>
            (new RenderingHelpers::SoftwareRendererSavedState (image, initialClip, origin))
        */
    }
}

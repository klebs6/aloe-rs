crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ImageComponent.h]

/**
  | A component that simply displays an
  | image.
  | 
  | Use setImage to give it an image, and
  | it'll display it - simple as that!
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ImageComponent<'a> {
    base:      Component<'a>,
    base2:     SettableTooltipClient,
    image:     Image,
    placement: RectanglePlacement,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/widgets/aloe_ImageComponent.cpp]
impl<'a> ImageComponent<'a> {

    /**
      | Creates an ImageComponent.
      |
      */
    pub fn new(name: &String) -> Self {
    
        todo!();
        /*
        : component(name),
        : placement(RectanglePlacement::centred),

        
        */
    }
    
    /**
      | Sets the image that should be displayed.
      |
      */
    pub fn set_image(&mut self, new_image: &Image)  {
        
        todo!();
        /*
            if (image != newImage)
        {
            image = newImage;
            repaint();
        }
        */
    }
    
    /**
      | Sets the image that should be displayed,
      | and its placement within the component.
      |
      */
    pub fn set_image_with_placement(
        &mut self, 
        new_image:        &Image,
        placement_to_use: RectanglePlacement

    ) {
        
        todo!();
        /*
            if (image != newImage || placement != placementToUse)
        {
            image = newImage;
            placement = placementToUse;
            repaint();
        }
        */
    }
    
    /**
      | Sets the method of positioning that
      | will be used to fit the image within the
      | component's bounds.
      | 
      | By default the positioning is centred,
      | and will fit the image inside the component's
      | bounds whilst keeping its aspect ratio
      | correct, but you can change it to whatever
      | layout you need.
      |
      */
    pub fn set_image_placement(&mut self, new_placement: RectanglePlacement)  {
        
        todo!();
        /*
            if (placement != newPlacement)
        {
            placement = newPlacement;
            repaint();
        }
        */
    }
    
    /**
      | Returns the current image.
      |
      */
    pub fn get_image(&self) -> &Image {
        
        todo!();
        /*
            return image;
        */
    }
    
    /**
      | Returns the current image placement.
      |
      */
    pub fn get_image_placement(&self) -> RectanglePlacement {
        
        todo!();
        /*
            return placement;
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setOpacity (1.0f);
        g.drawImage (image, getLocalBounds().toFloat(), placement);
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            class ImageComponentAccessibilityHandler  : public AccessibilityHandler
        {
        
            explicit ImageComponentAccessibilityHandler (ImageComponent& imageComponentToWrap)
                : AccessibilityHandler (imageComponentToWrap, AccessibilityRole::image),
                  imageComponent (imageComponentToWrap)
            {
            }

            String getHelp() const override   { return imageComponent.getTooltip(); }

        
            ImageComponent& imageComponent;

            
            ALOE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR (ImageComponentAccessibilityHandler)
        };

        return std::make_unique<ImageComponentAccessibilityHandler> (*this);
        */
    }
}

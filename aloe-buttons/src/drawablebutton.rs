crate::ix!();

pub fn copy_drawable_if_not_null(d: *const Drawable) -> Box<Drawable> {
    
    todo!();
    /*
        if (d != nullptr)
            return d->createCopy();

        return {};
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_DrawableButton.h]

/**
  | A button that displays a Drawable.
  | 
  | Up to three Drawable objects can be given
  | to this button, to represent the 'normal',
  | 'over' and 'down' states.
  | 
  | @see Button
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct DrawableButton<'a> {
    base:              Button<'a>,
    style:             DrawableButtonStyle,
    normal_image:      Box<Drawable<'a>>,
    over_image:        Box<Drawable<'a>>,
    down_image:        Box<Drawable<'a>>,
    disabled_image:    Box<Drawable<'a>>,
    normal_image_on:   Box<Drawable<'a>>,
    over_image_on:     Box<Drawable<'a>>,
    down_image_on:     Box<Drawable<'a>>,
    disabled_image_on: Box<Drawable<'a>>,
    current_image:     *mut Drawable<'a>, // default = nullptr
    edge_indent:       i32, // default = 3
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_DrawableButton.cpp]
impl<'a> DrawableButton<'a> {
    
    /**
      | Returns the current style.
      |
      */
    pub fn get_style(&self) -> DrawableButtonStyle {
        
        todo!();
        /*
            return style;
        */
    }
    
    /**
      | Returns the current edge indent size.
      |
      */
    pub fn get_edge_indent(&self) -> i32 {
        
        todo!();
        /*
            return edgeIndent;
        */
    }
    
    pub fn should_draw_button_background(&self) -> bool {
        
        todo!();
        /*
            return style == ImageOnButtonBackground || style == ImageOnButtonBackgroundOriginalSize;
        */
    }

    /**
      | Creates a DrawableButton.
      | 
      | After creating one of these, use setImages()
      | to specify the drawables to use.
      | 
      | -----------
      | @param buttonName
      | 
      | the name to give the component
      | ----------
      | @param buttonStyle
      | 
      | the layout to use
      | 
      | @see DrawableButtonStyle, setButtonStyle,
      | setImages
      |
      */
    pub fn new(
        name:         &String,
        button_style: DrawableButtonStyle) -> Self {
    
        todo!();
        /*
        : button(name),
        : style(buttonStyle),

        
        */
    }

    /**
      | Sets up the images to draw for the various
      | button states.
      | 
      | The button will keep its own internal
      | copies of these drawables.
      | 
      | -----------
      | @param normalImage
      | 
      | the thing to draw for the button's 'normal'
      | state. An internal copy will be made
      | of the object passed-in if it is non-null.
      | ----------
      | @param overImage
      | 
      | the thing to draw for the button's 'over'
      | state - if this is null, the button's
      | normal image will be used when the mouse
      | is over it. An internal copy will be made
      | of the object passed-in if it is non-null.
      | ----------
      | @param downImage
      | 
      | the thing to draw for the button's 'down'
      | state - if this is null, the 'over' image
      | will be used instead (or the normal image
      | as a last resort). An internal copy will
      | be made of the object passed-in if it
      | is non-null.
      | ----------
      | @param disabledImage
      | 
      | an image to draw when the button is disabled.
      | If this is null, the normal image will
      | be drawn with a reduced opacity instead.
      | An internal copy will be made of the object
      | passed-in if it is non-null.
      | ----------
      | @param normalImageOn
      | 
      | same as the normalImage, but this is
      | used when the button's toggle state
      | is 'on'. If this is nullptr, the normal
      | image is used instead
      | ----------
      | @param overImageOn
      | 
      | same as the overImage, but this is used
      | when the button's toggle state is 'on'.
      | If this is nullptr, the normalImageOn
      | is drawn instead
      | ----------
      | @param downImageOn
      | 
      | same as the downImage, but this is used
      | when the button's toggle state is 'on'.
      | If this is nullptr, the overImageOn
      | is drawn instead
      | ----------
      | @param disabledImageOn
      | 
      | same as the disabledImage, but this
      | is used when the button's toggle state
      | is 'on'. If this is nullptr, the normal
      | image will be drawn instead with a reduced
      | opacity
      |
      */
    pub fn set_images(&mut self, 
        normal:      *const Drawable,
        over:        *const Drawable,
        down:        *const Drawable,
        disabled:    *const Drawable,
        normal_on:   *const Drawable,
        over_on:     *const Drawable,
        down_on:     *const Drawable,
        disabled_on: *const Drawable)  {
        
        todo!();
        /*
            jassert (normal != nullptr); // you really need to give it at least a normal image..

        normalImage     = copyDrawableIfNotNull (normal);
        overImage       = copyDrawableIfNotNull (over);
        downImage       = copyDrawableIfNotNull (down);
        disabledImage   = copyDrawableIfNotNull (disabled);
        normalImageOn   = copyDrawableIfNotNull (normalOn);
        overImageOn     = copyDrawableIfNotNull (overOn);
        downImageOn     = copyDrawableIfNotNull (downOn);
        disabledImageOn = copyDrawableIfNotNull (disabledOn);

        currentImage = nullptr;

        buttonStateChanged();
        */
    }
    
    /**
      | Changes the button's style. @see DrawableButtonStyle
      |
      */
    pub fn set_button_style(&mut self, new_style: DrawableButtonStyle)  {
        
        todo!();
        /*
            if (style != newStyle)
        {
            style = newStyle;
            buttonStateChanged();
        }
        */
    }
    
    /**
      | Gives the button an optional amount
      | of space around the edge of the drawable.
      | 
      | By default there's a gap of about 3 pixels.
      |
      */
    pub fn set_edge_indent(&mut self, num_pixels_indent: i32)  {
        
        todo!();
        /*
            edgeIndent = numPixelsIndent;
        repaint();
        resized();
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            Button::resized();

        if (currentImage != nullptr)
        {
            if (style != ImageRaw)
            {
                int transformFlags = 0;

                if (style == ImageStretched)
                {
                    transformFlags |= RectanglePlacement::stretchToFit;
                }
                else
                {
                    transformFlags |= RectanglePlacement::centred;

                    if (style == ImageOnButtonBackgroundOriginalSize)
                        transformFlags |= RectanglePlacement::doNotResize;
                }

                currentImage->setTransformToFit (getImageBounds(), transformFlags);
            }
        }
        */
    }
    
    pub fn button_state_changed(&mut self)  {
        
        todo!();
        /*
            repaint();

        Drawable* imageToDraw = nullptr;
        float opacity = 1.0f;

        if (isEnabled())
        {
            imageToDraw = getCurrentImage();
        }
        else
        {
            imageToDraw = getToggleState() ? disabledImageOn.get()
                                           : disabledImage.get();

            if (imageToDraw == nullptr)
            {
                opacity = 0.4f;
                imageToDraw = getNormalImage();
            }
        }

        if (imageToDraw != currentImage)
        {
            removeChildComponent (currentImage);
            currentImage = imageToDraw;

            if (currentImage != nullptr)
            {
                currentImage->setInterceptsMouseClicks (false, false);
                addAndMakeVisible (currentImage);
                resized();
            }
        }

        if (currentImage != nullptr)
            currentImage->setAlpha (opacity);
        */
    }
    
    pub fn enablement_changed(&mut self)  {
        
        todo!();
        /*
            Button::enablementChanged();
        buttonStateChanged();
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            auto& lf = getLookAndFeel();

        if (shouldDrawButtonBackground())
            lf.drawButtonBackground (g, *this,
                                     findColour (getToggleState() ? TextButton::buttonOnColourId
                                                                  : TextButton::buttonColourId),
                                     shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);
        else
            lf.drawDrawableButton (g, *this, shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown);
        */
    }
    
    /**
      | Returns the image that the button is
      | currently displaying.
      |
      */
    pub fn get_current_image(&self) -> *mut Drawable {
        
        todo!();
        /*
            if (isDown())  return getDownImage();
        if (isOver())  return getOverImage();

        return getNormalImage();
        */
    }
    
    /**
      | Returns the image that the button will
      | use for its normal state.
      |
      */
    pub fn get_normal_image(&self) -> *mut Drawable {
        
        todo!();
        /*
            return (getToggleState() && normalImageOn != nullptr) ? normalImageOn.get()
                                                              : normalImage.get();
        */
    }
    
    /**
      | Returns the image that the button will
      | use when the mouse is over it.
      |
      */
    pub fn get_over_image(&self) -> *mut Drawable {
        
        todo!();
        /*
            if (getToggleState())
        {
            if (overImageOn   != nullptr)   return overImageOn.get();
            if (normalImageOn != nullptr)   return normalImageOn.get();
        }

        return overImage != nullptr ? overImage.get() : normalImage.get();
        */
    }
    
    /**
      | Returns the image that the button will
      | use when the mouse is held down on it.
      |
      */
    pub fn get_down_image(&self) -> *mut Drawable {
        
        todo!();
        /*
            if (auto* d = getToggleState() ? downImageOn.get() : downImage.get())
            return d;

        return getOverImage();
        */
    }
}

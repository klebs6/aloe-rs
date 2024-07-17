crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ImageButton.h]

/**
  | As the title suggests, this is a button
  | containing an image.
  | 
  | The colour and transparency of the image
  | can be set to vary when the button state
  | changes.
  | 
  | @see Button, ShapeButton, TextButton
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ImageButton<'a> {
    base:                 Button<'a>,
    scale_image_to_fit:   bool,
    preserve_proportions: bool,
    alpha_threshold:      u8,
    image_bounds:         Rectangle<i32>,
    normal_image:         Image,
    over_image:           Image,
    down_image:           Image,
    normal_opacity:       f32,
    over_opacity:         f32,
    down_opacity:         f32,
    normal_overlay:       Colour,
    over_overlay:         Colour,
    down_overlay:         Colour,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/buttons/aloe_ImageButton.cpp]
impl<'a> ImageButton<'a> {
    
    /**
      | Creates an ImageButton.
      | 
      | Use setImage() to specify the image
      | to use. The colours and opacities that
      | are specified here can be changed later
      | using setImages().
      | 
      | -----------
      | @param name
      | 
      | the name to give the component
      |
      */
    pub fn new(text: Option<&String>) -> Self {

        let text: &String = text.unwrap_or(&String::new());
    
        todo!();
        /*
        : button(text_),
        : scale_image_to_fit(true),
        : preserve_proportions(true),
        : alpha_threshold(0),

        
        */
    }

    /**
      | Sets up the images to draw in various
      | states.
      | 
      | -----------
      | @param resizeButtonNowToFitThisImage
      | 
      | if true, the button will be immediately
      | resized to the same dimensions as the
      | normal image
      | ----------
      | @param rescaleImagesWhenButtonSizeChanges
      | 
      | if true, the image will be rescaled to
      | fit the button when the button's size
      | changes
      | ----------
      | @param preserveImageProportions
      | 
      | if true then any rescaling of the image
      | to fit the button will keep the image's
      | x and y proportions correct - i.e. it
      | won't distort its shape, although this
      | might create gaps around the edges
      | ----------
      | @param normalImage
      | 
      | the image to use when the button is in
      | its normal state. button no longer needs
      | it.
      | ----------
      | @param imageOpacityWhenNormal
      | 
      | the opacity to use when drawing the normal
      | image.
      | ----------
      | @param overlayColourWhenNormal
      | 
      | an overlay colour to use to fill the alpha
      | channel of the normal image - if this
      | colour is transparent, no overlay will
      | be drawn. The overlay will be drawn over
      | the top of the image, so you can basically
      | add a solid or semi-transparent colour
      | to the image to brighten or darken it
      | ----------
      | @param overImage
      | 
      | the image to use when the mouse is over
      | the button. If you want to use the same
      | image as was set in the normalImage parameter,
      | this value can be a null image.
      | ----------
      | @param imageOpacityWhenOver
      | 
      | the opacity to use when drawing the image
      | when the mouse is over the button
      | ----------
      | @param overlayColourWhenOver
      | 
      | an overlay colour to use to fill the alpha
      | channel of the image when the mouse is
      | over - if this colour is transparent,
      | no overlay will be drawn
      | ----------
      | @param downImage
      | 
      | an image to use when the button is pressed
      | down. If set to a null image, the 'over'
      | image will be drawn instead (or the normal
      | image if there isn't an 'over' image
      | either).
      | ----------
      | @param imageOpacityWhenDown
      | 
      | the opacity to use when drawing the image
      | when the button is pressed
      | ----------
      | @param overlayColourWhenDown
      | 
      | an overlay colour to use to fill the alpha
      | channel of the image when the button
      | is pressed down - if this colour is transparent,
      | no overlay will be drawn
      | ----------
      | @param hitTestAlphaThreshold
      | 
      | if set to zero, the mouse is considered
      | to be over the button whenever it's inside
      | the button's bounding rectangle. If
      | set to values higher than 0, the mouse
      | will only be considered to be over the
      | image when the value of the image's alpha
      | channel at that position is greater
      | than this level.
      |
      */
      pub fn set_images(&mut self, 
          resize_button_now_to_fit_this_image:     bool,
          rescale_images_when_button_size_changes: bool,
          preserve_image_proportions:              bool,
          normal_image:                            &Image,
          image_opacity_when_normal:               f32,
          overlay_colour_when_normal:              Colour,
          over_image:                              &Image,
          image_opacity_when_over:                 f32,
          overlay_colour_when_over:                Colour,
          down_image:                              &Image,
          image_opacity_when_down:                 f32,
          overlay_colour_when_down:                Colour,
          hit_test_alpha_threshold:                Option<f32>
      )  {

        let hit_test_alpha_threshold: f32 = hit_test_alpha_threshold.unwrap_or(0.0);
        
        todo!();
        /*
            normalImage = normalImage_;
        overImage = overImage_;
        downImage = downImage_;

        if (resizeButtonNowToFitThisImage && normalImage.isValid())
        {
            imageBounds.setSize (normalImage.getWidth(),
                                 normalImage.getHeight());

            setSize (imageBounds.getWidth(), imageBounds.getHeight());
        }

        scaleImageToFit = rescaleImagesWhenButtonSizeChanges;
        preserveProportions = preserveImageProportions;

        normalOpacity = imageOpacityWhenNormal;
        normalOverlay = overlayColourWhenNormal;
        overOpacity   = imageOpacityWhenOver;
        overOverlay   = overlayColourWhenOver;
        downOpacity   = imageOpacityWhenDown;
        downOverlay   = overlayColourWhenDown;

        alphaThreshold = (uint8) jlimit (0, 0xff, roundToInt (255.0f * hitTestAlphaThreshold));

        repaint();
        */
    }
    
    pub fn get_current_image(&self) -> Image {
        
        todo!();
        /*
            if (isDown() || getToggleState())
            return getDownImage();

        if (isOver())
            return getOverImage();

        return getNormalImage();
        */
    }
    
    /**
      | Returns the currently set 'normal'
      | image.
      |
      */
    pub fn get_normal_image(&self) -> Image {
        
        todo!();
        /*
            return normalImage;
        */
    }
    
    /**
      | Returns the image that's drawn when
      | the mouse is over the button.
      | 
      | If a valid 'over' image has been set,
      | this will return it; otherwise it'll
      | just return the normal image.
      |
      */
    pub fn get_over_image(&self) -> Image {
        
        todo!();
        /*
            return overImage.isValid() ? overImage
                                   : normalImage;
        */
    }
    
    /**
      | Returns the image that's drawn when
      | the button is held down.
      | 
      | If a valid 'down' image has been set,
      | this will return it; otherwise it'll
      | return the 'over' image or normal image,
      | depending on what's available.
      |
      */
    pub fn get_down_image(&self) -> Image {
        
        todo!();
        /*
            return downImage.isValid() ? downImage
                                   : getOverImage();
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            if (! isEnabled())
        {
            shouldDrawButtonAsHighlighted = false;
            shouldDrawButtonAsDown = false;
        }

        Image im (getCurrentImage());

        if (im.isValid())
        {
            const int iw = im.getWidth();
            const int ih = im.getHeight();
            int w = getWidth();
            int h = getHeight();
            int x = (w - iw) / 2;
            int y = (h - ih) / 2;

            if (scaleImageToFit)
            {
                if (preserveProportions)
                {
                    int newW, newH;
                    const float imRatio = (float) ih / (float) iw;
                    const float destRatio = (float) h / (float) w;

                    if (imRatio > destRatio)
                    {
                        newW = roundToInt ((float) h / imRatio);
                        newH = h;
                    }
                    else
                    {
                        newW = w;
                        newH = roundToInt ((float) w * imRatio);
                    }

                    x = (w - newW) / 2;
                    y = (h - newH) / 2;
                    w = newW;
                    h = newH;
                }
                else
                {
                    x = 0;
                    y = 0;
                }
            }

            if (! scaleImageToFit)
            {
                w = iw;
                h = ih;
            }

            imageBounds.setBounds (x, y, w, h);

            const bool useDownImage = shouldDrawButtonAsDown || getToggleState();

            getLookAndFeel().drawImageButton (g, &im, x, y, w, h,
                                              useDownImage ? downOverlay
                                                           : (shouldDrawButtonAsHighlighted ? overOverlay
                                                                                : normalOverlay),
                                              useDownImage ? downOpacity
                                                           : (shouldDrawButtonAsHighlighted ? overOpacity
                                                                                : normalOpacity),
                                              *this);
        }
        */
    }
    
    pub fn hit_test(&mut self, x: i32, y: i32) -> bool {
        
        todo!();
        /*
            if (! Component::hitTest (x, y)) // handle setInterceptsMouseClicks
            return false;

        if (alphaThreshold == 0)
            return true;

        Image im (getCurrentImage());

        return im.isNull() || ((! imageBounds.isEmpty())
                                && alphaThreshold < im.getPixelAt (((x - imageBounds.getX()) * im.getWidth()) / imageBounds.getWidth(),
                                                                   ((y - imageBounds.getY()) * im.getHeight()) / imageBounds.getHeight()).getAlpha());
        */
    }
}

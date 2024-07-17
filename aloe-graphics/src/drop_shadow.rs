crate::ix!();

pub fn blur_data_triplets(
    d:     *mut u8,
    num:   i32,
    delta: i32)  {

    todo!();
    /*
        uint32 last = d[0];
        d[0] = (uint8) ((d[0] + d[delta] + 1) / 3);
        d += delta;

        num -= 2;

        do
        {
            const uint32 newLast = d[0];
            d[0] = (uint8) ((last + d[0] + d[delta] + 1) / 3);
            d += delta;
            last = newLast;
        }
        while (--num > 0);

        d[0] = (uint8) ((last + d[0] + 1) / 3);
    */
}

pub fn blur_single_channel_image(
    data:        *mut u8,
    width:       i32,
    height:      i32,
    line_stride: i32,
    repetitions: i32)  {
    
    todo!();
    /*
        jassert (width > 2 && height > 2);

        for (int y = 0; y < height; ++y)
            for (int i = repetitions; --i >= 0;)
                blurDataTriplets (data + lineStride * y, width, 1);

        for (int x = 0; x < width; ++x)
            for (int i = repetitions; --i >= 0;)
                blurDataTriplets (data + x, height, lineStride);
    */
}

pub fn blur_single_channel_image_with_radius(
    image:  &mut Image,
    radius: i32)  {
    
    todo!();
    /*
        const ImageBitmapData bm (image, ImageBitmapData::readWrite);
        blurSingleChannelImage (bm.data, bm.width, bm.height, bm.lineStride, 2 * radius);
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/effects/aloe_DropShadowEffect.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/effects/aloe_DropShadowEffect.cpp]

/**
  | Defines a drop-shadow effect.
  | 
  | @tags{Graphics}
  |
  */
pub struct DropShadow {

    /**
      | The colour with which to render the shadow. In
      | most cases you'll probably want to leave this as
      | black with an alpha value of around 0.5
      */
    colour: Colour, // default = { 0x90000000  }

    /**
      | The approximate spread of the shadow.
      |
      */
    radius: i32, // default = { 4  }

    /**
      | The offset of the shadow.
      |
      */
    offset: Point<i32>,
}

impl Default for DropShadow {
    
    /**
      | Creates a default drop-shadow effect.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl DropShadow {

    /**
      | Creates a drop-shadow object with the
      | given parameters.
      |
      */
    pub fn new(
        shadow_colour: Colour,
        r:             i32,
        o:             Point<i32>) -> Self {
    
        todo!();
        /*
        : colour(shadowColour),
        : radius(r),
        : offset(o),

            jassert (radius > 0);
        */
    }
}

/**
  | An effect filter that adds a drop-shadow
  | behind the image's content.
  | 
  | (This will only work on images/components
  | that aren't opaque, of course).
  | 
  | When added to a component, this effect
  | will draw a soft-edged shadow based
  | on what gets drawn inside it. The shadow
  | will also be applied to the component's
  | children.
  | 
  | For speed, this doesn't use a proper
  | gaussian blur, but cheats by using a
  | simple bilinear filter. If you need
  | a really high-quality shadow, check
  | out ImageConvolutionKernel::createGaussianBlur()
  | 
  | @see Component::setComponentEffect
  | 
  | @tags{Graphics}
  |
  */
#[leak_detector]
pub struct DropShadowEffect {
    shadow: DropShadow,
}

impl ImageEffectFilter for DropShadowEffect {

    /**
      | @internal
      |
      */
    fn apply_effect(&mut self, 
        image:        &mut Image,
        g:            &mut Graphics,
        scale_factor: f32,
        alpha:        f32)  {
        
        todo!();
        /*
            DropShadow s (shadow);
        s.radius = roundToInt ((float) s.radius * scaleFactor);
        s.colour = s.colour.withMultipliedAlpha (alpha);
        s.offset.x = roundToInt ((float) s.offset.x * scaleFactor);
        s.offset.y = roundToInt ((float) s.offset.y * scaleFactor);

        s.drawForImage (g, image);

        g.setOpacity (alpha);
        g.drawImageAt (image, 0, 0);
        */
    }
}

impl Default for DropShadowEffect {
    
    /**
      | Creates a default drop-shadow effect.
      | 
      | To customise the shadow's appearance,
      | use the setShadowProperties() method.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl DropShadowEffect {
    
    /**
      | Sets up parameters affecting the shadow's
      | appearance.
      |
      */
    pub fn set_shadow_properties(&mut self, new_shadow: &DropShadow)  {
        
        todo!();
        /*
            shadow = newShadow;
        */
    }
}

pub trait DrawForImage {

    fn draw_for_image(
        &self, 
        g:         &mut Graphics,
        src_image: &Image);
}

impl DrawForImage for DropShadow {

    /**
      | Renders a drop-shadow based on the alpha-channel
      | of the given image.
      |
      */
    fn draw_for_image(
        &self, 
        g:         &mut Graphics,
        src_image: &Image)  {
        
        todo!();
        /*
            jassert (radius > 0);

        if (srcImage.isValid())
        {
            Image shadowImage (srcImage.convertedToFormat (Image::SingleChannel));
            shadowImage.duplicateIfShared();

            blurSingleChannelImage (shadowImage, radius);

            g.setColour (colour);
            g.drawImageAt (shadowImage, offset.x, offset.y, true);
        }
        */
    }
}

pub trait DrawForPath {

    fn draw_for_path(
        &self, 
        g:    &mut Graphics,
        path: &Path);
}

impl DrawForPath for DropShadow {

    /**
      | Renders a drop-shadow based on the shape
      | of a path.
      |
      */
    fn draw_for_path(
        &self, 
        g:    &mut Graphics,
        path: &Path)  {
        
        todo!();
        /*
            jassert (radius > 0);

        auto area = (path.getBounds().getSmallestIntegerContainer() + offset)
                      .expanded (radius + 1)
                      .getIntersection (g.getClipBounds().expanded (radius + 1));

        if (area.getWidth() > 2 && area.getHeight() > 2)
        {
            Image renderedPath (Image::SingleChannel, area.getWidth(), area.getHeight(), true);

            {
                Graphics g2 (renderedPath);
                g2.setColour (Colours::white);
                g2.fillPath (path, AffineTransform::translation ((float) (offset.x - area.getX()),
                                                                 (float) (offset.y - area.getY())));
            }

            blurSingleChannelImage (renderedPath, radius);

            g.setColour (colour);
            g.drawImageAt (renderedPath, area.getX(), area.getY(), true);
        }
        */
    }
}

pub trait DrawForRectangle {

    fn draw_for_rectangle(
        &self, 
        g:           &mut Graphics,
        target_area: &Rectangle<i32>);
}

impl DrawForRectangle for DropShadow {

    /**
      | Renders a drop-shadow for a rectangle.
      | 
      | -----------
      | @note
      | 
      | for speed, this approximates the shadow
      | using gradients.
      |
      */
    fn draw_for_rectangle(
        &self, 
        g:           &mut Graphics,
        target_area: &Rectangle<i32>)  {
        
        todo!();
        /*
            ColourGradient cg (colour, 0, 0, colour.withAlpha (0.0f), 0, 0, false);

        for (float i = 0.05f; i < 1.0f; i += 0.1f)
            cg.addColour (1.0 - i, colour.withMultipliedAlpha (i * i));

        const float radiusInset = (float) radius / 2.0f;
        const float expandedRadius = (float) radius + radiusInset;

        auto area = targetArea.toFloat().reduced (radiusInset) + offset.toFloat();

        auto r = area.expanded (expandedRadius);
        auto top = r.removeFromTop (expandedRadius);
        auto bottom = r.removeFromBottom (expandedRadius);

        drawShadowSection (g, cg, top.removeFromLeft  (expandedRadius), true, 1.0f, 1.0f, 0, 1.0f);
        drawShadowSection (g, cg, top.removeFromRight (expandedRadius), true, 0, 1.0f, 1.0f, 1.0f);
        drawShadowSection (g, cg, top, false, 0, 1.0f, 0, 0);

        drawShadowSection (g, cg, bottom.removeFromLeft  (expandedRadius), true, 1.0f, 0, 0, 0);
        drawShadowSection (g, cg, bottom.removeFromRight (expandedRadius), true, 0, 0, 1.0f, 0);
        drawShadowSection (g, cg, bottom, false, 0, 0, 0, 1.0f);

        drawShadowSection (g, cg, r.removeFromLeft  (expandedRadius), false, 1.0f, 0, 0, 0);
        drawShadowSection (g, cg, r.removeFromRight (expandedRadius), false, 0, 0, 1.0f, 0);

        g.setColour (colour);
        g.fillRect (area);
        */
    }
}


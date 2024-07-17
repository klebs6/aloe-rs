crate::ix!();

pub fn draw_shadow_section(
    g:         &mut Graphics,
    cg:        &mut ColourGradient,
    area:      Rectangle<f32>,
    is_corner: bool,
    centrex:   f32,
    centrey:   f32,
    edgex:     f32,
    edgey:     f32)  {

    todo!();
    /*
        cg.point1 = area.getRelativePoint (centreX, centreY);
        cg.point2 = area.getRelativePoint (edgeX, edgeY);
        cg.isRadial = isCorner;

        g.setGradientFill (cg);
        g.fillRect (area);
    */
}

pub trait Rescaled {

    fn rescaled(
        &self, 
        new_width:  i32,
        new_height: i32,
        quality:    Option<GraphicsResamplingQuality>

    ) -> Self;
}

impl Rescaled for Image {

    /**
      | Returns a rescaled version of this image.
      | 
      | A new image is returned which is a copy
      | of this one, rescaled to the given size.
      | 
      | -----------
      | @note
      | 
      | if the new size is identical to the existing
      | image, this will just return a reference
      | to the original image, and won't actually
      | create a duplicate.
      |
      */
    fn rescaled(
        &self, 
        new_width:  i32,
        new_height: i32,
        quality:    Option<GraphicsResamplingQuality>

    ) -> Image {

        let quality: GraphicsResamplingQuality = quality.unwrap_or(GraphicsResamplingQuality::mediumResamplingQuality);
        
        todo!();
        /*
            if (image == nullptr || (image->width == newWidth && image->height == newHeight))
            return *this;

        auto type = image->createType();
        Image newImage (type->create (image->pixelFormat, newWidth, newHeight, hasAlphaChannel()));

        Graphics g (newImage);
        g.setImageResamplingQuality (quality);
        g.drawImageTransformed (*this, AffineTransform::scale ((float) newWidth  / (float) image->width,
                                                               (float) newHeight / (float) image->height), false);
        return newImage;
        */
    }
}

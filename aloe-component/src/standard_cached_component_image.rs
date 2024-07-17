crate::ix!();

///-------------------
#[no_copy]
#[leak_detector]
pub struct StandardCachedComponentImage<'a> {
    image:      Image,
    valid_area: RectangleList<i32>,
    owner:      &'a mut Component<'a>,
    scale:      f32, // default = 1.0f
}

impl<'a> CachedComponentImage for StandardCachedComponentImage<'a> {

}

impl<'a> PaintGraphics for StandardCachedComponentImage<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            scale = g.getInternalContext().getPhysicalPixelScaleFactor();
            auto compBounds = owner.getLocalBounds();
            auto imageBounds = compBounds * scale;

            if (image.isNull() || image.getBounds() != imageBounds)
            {
                image = Image (owner.isOpaque() ? Image::RGB
                                                : Image::ARGB,
                               jmax (1, imageBounds.getWidth()),
                               jmax (1, imageBounds.getHeight()),
                               ! owner.isOpaque());

                validArea.clear();
            }

            if (! validArea.containsRectangle (compBounds))
            {
                Graphics imG (image);
                auto& lg = imG.getInternalContext();

                lg.addTransform (AffineTransform::scale (scale));

                for (auto& i : validArea)
                    lg.excludeClipRectangle (i);

                if (! owner.isOpaque())
                {
                    lg.setFill (Colours::transparentBlack);
                    lg.fillRect (compBounds, true);
                    lg.setFill (Colours::black);
                }

                owner.paintEntireComponent (imG, true);
            }

            validArea = compBounds;

            g.setColour (Colours::black.withAlpha (owner.getAlpha()));
            g.drawImageTransformed (image, AffineTransform::scale ((float) compBounds.getWidth()  / (float) imageBounds.getWidth(),
                                                                   (float) compBounds.getHeight() / (float) imageBounds.getHeight()), false);
        */
    }
}

impl<'a> ReleaseResources for StandardCachedComponentImage<'a> {

    fn release_resources(&mut self)  {
        
        todo!();
        /*
            image = Image();
        */
    }
}

impl<'a> Invalidate for StandardCachedComponentImage<'a> {

    fn invalidate(&mut self, area: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            validArea.subtract (area); return true;
        */
    }
}

impl<'a> InvalidateAll for StandardCachedComponentImage<'a> {

    fn invalidate_all(&mut self) -> bool {
        
        todo!();
        /*
            validArea.clear(); return true;
        */
    }
}

impl<'a> StandardCachedComponentImage<'a> {

    pub fn new(c: &mut Component<'a>) -> Self {
    
        todo!();
        /*
        : owner(c),
        */
    }
}

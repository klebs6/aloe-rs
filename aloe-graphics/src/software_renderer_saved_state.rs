crate::ix!();

pub struct SoftwareRendererSavedState {
    base:  SavedStateBase<SoftwareRendererSavedState>,
    image: Image,
    font:  Font,
}

pub type SoftwareRendererSavedStateBaseClass      = SavedStateBase<SoftwareRendererSavedState>;
pub type SoftwareRendererSavedStateGlyphCacheType = GlyphCache<CachedGlyphEdgeTable<SoftwareRendererSavedState>,SoftwareRendererSavedState>;

impl SoftwareRendererSavedState {

    pub fn new_with_clip_bounds(
        im:          &Image,
        clip_bounds: Rectangle<i32>) -> Self {
    
        todo!();
        /*
        : base_class(clipBounds),
        : image(im),

        
        */
    }
    
    pub fn new(
        im:        &Image,
        clip_list: &RectangleList<i32>,
        origin:    Point<i32>) -> Self {
    
        todo!();
        /*
        : base_class(clipList, origin),
        : image(im),

        
        */
    }
    
    pub fn begin_transparency_layer(&mut self, opacity: f32) -> *mut SoftwareRendererSavedState {
        
        todo!();
        /*
            auto* s = new SoftwareRendererSavedState (*this);

            if (clip != nullptr)
            {
                auto layerBounds = clip->getClipBounds();

                s->image = Image (typename Image::ARGB, layerBounds.getWidth(), layerBounds.getHeight(), true);
                s->transparencyLayerAlpha = opacity;
                s->transform.moveOriginInDeviceSpace (-layerBounds.getPosition());
                s->cloneClipIfMultiplyReferenced();
                s->clip->translate (-layerBounds.getPosition());
            }

            return s;
        */
    }
    
    pub fn end_transparency_layer(&mut self, finished_layer_state: &mut SoftwareRendererSavedState)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                auto layerBounds = clip->getClipBounds();

                auto g = image.createLowLevelContext();
                g->setOpacity (finishedLayerState.transparencyLayerAlpha);
                g->drawImage (finishedLayerState.image, AffineTransform::translation (layerBounds.getPosition()));
            }
        */
    }
    
    pub fn clear_glyph_cache()  {
        
        todo!();
        /*
            GlyphCacheType::getInstance().reset();
        */
    }
    
    pub fn draw_glyph(&mut self, 
        glyph_number: i32,
        trans:        &AffineTransform)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (trans.isOnlyTranslation() && ! transform.isRotated)
                {
                    auto& cache = GlyphCacheType::getInstance();
                    Point<float> pos (trans.getTranslationX(), trans.getTranslationY());

                    if (transform.isOnlyTranslated)
                    {
                        cache.drawGlyph (*this, font, glyphNumber, pos + transform.offset.toFloat());
                    }
                    else
                    {
                        pos = transform.transformed (pos);

                        Font f (font);
                        f.setHeight (font.getHeight() * transform.complexTransform.mat11);

                        auto xScale = transform.complexTransform.mat00 / transform.complexTransform.mat11;

                        if (std::abs (xScale - 1.0f) > 0.01f)
                            f.setHorizontalScale (xScale);

                        cache.drawGlyph (*this, f, glyphNumber, pos);
                    }
                }
                else
                {
                    auto fontHeight = font.getHeight();

                    auto t = transform.getTransformWith (AffineTransform::scale (fontHeight * font.getHorizontalScale(), fontHeight)
                                                                         .followedBy (trans));

                    std::unique_ptr<EdgeTable> et (font.getTypeface()->getEdgeTableForGlyph (glyphNumber, t, fontHeight));

                    if (et != nullptr)
                        fillShape (*new EdgeTableRegionType (*et), false);
                }
            }
        */
    }
    
    pub fn get_maximum_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return image.getBounds();
        */
    }
    
    pub fn render_image_transformed<IteratorType>(&self, 
        iter:       &mut IteratorType,
        src:        &Image,
        alpha:      i32,
        trans:      &AffineTransform,
        quality:    GraphicsResamplingQuality,
        tiled_fill: bool)  {
    
        todo!();
        /*
            typename ImageBitmapData destData (image, typename ImageBitmapData::readWrite);
            const typename ImageBitmapData srcData (src, typename ImageBitmapData::readOnly);
            EdgeTableFillers::renderImageTransformed (iter, destData, srcData, alpha, trans, quality, tiledFill);
        */
    }
    
    pub fn render_image_untransformed<IteratorType>(&self, 
        iter:       &mut IteratorType,
        src:        &Image,
        alpha:      i32,
        x:          i32,
        y:          i32,
        tiled_fill: bool)  {
    
        todo!();
        /*
            typename ImageBitmapData destData (image, typename ImageBitmapData::readWrite);
            const typename ImageBitmapData srcData (src, typename ImageBitmapData::readOnly);
            EdgeTableFillers::renderImageUntransformed (iter, destData, srcData, alpha, x, y, tiledFill);
        */
    }
    
    pub fn fill_with_solid_colour<IteratorType>(&self, 
        iter:             &mut IteratorType,
        colour:           PixelARGB,
        replace_contents: bool)  {
    
        todo!();
        /*
            typename ImageBitmapData destData (image, typename ImageBitmapData::readWrite);

            switch (destData.pixelFormat)
            {
                case typename Image::ARGB:   EdgeTableFillers::renderSolidFill (iter, destData, colour, replaceContents, (PixelARGB*) nullptr); break;
                case typename Image::RGB:    EdgeTableFillers::renderSolidFill (iter, destData, colour, replaceContents, (PixelRGB*) nullptr); break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:            EdgeTableFillers::renderSolidFill (iter, destData, colour, replaceContents, (PixelAlpha*) nullptr); break;
            }
        */
    }
    
    pub fn fill_with_gradient<IteratorType>(&self, 
        iter:        &mut IteratorType,
        gradient:    &mut ColourGradient,
        trans:       &AffineTransform,
        is_identity: bool)  {
    
        todo!();
        /*
            HeapBlock<PixelARGB> lookupTable;
            auto numLookupEntries = gradient.createLookupTable (trans, lookupTable);
            jassert (numLookupEntries > 0);

            typename ImageBitmapData destData (image, typename ImageBitmapData::readWrite);

            switch (destData.pixelFormat)
            {
                case typename Image::ARGB:   EdgeTableFillers::renderGradient (iter, destData, gradient, trans, lookupTable, numLookupEntries, isIdentity, (PixelARGB*) nullptr); break;
                case typename Image::RGB:    EdgeTableFillers::renderGradient (iter, destData, gradient, trans, lookupTable, numLookupEntries, isIdentity, (PixelRGB*) nullptr); break;
                case typename Image::SingleChannel:
                case typename Image::UnknownFormat:
                default:            EdgeTableFillers::renderGradient (iter, destData, gradient, trans, lookupTable, numLookupEntries, isIdentity, (PixelAlpha*) nullptr); break;
            }
        */
    }
}

crate::ix!();

pub struct SavedState<'a> {
    base:                   SavedStateBase<SavedState<'a>>,
    font:                   Font,
    state:                  *mut GLState<'a>,
    is_using_custom_shader: bool, // default = false
    transparency_layer:     Image,
    previous_target:        Box<RenderingTarget<'a>>,
}

pub type SavedStateBaseClass<'a>      = SavedStateBase<SavedState<'a>>;
pub type SavedStateGlyphCacheType<'a> = GlyphCache<CachedGlyphEdgeTable<SavedState<'a>>, SavedState<'a>>;

impl<'a> SavedState<'a> {

    pub fn new_from_glstate(s: *mut GLState) -> Self {
    
        todo!();
        /*
            : BaseClass (s->target.bounds), state (s)
        */
    }
    
    pub fn new_from_savedstate(other: &SavedState) -> Self {
    
        todo!();
        /*


            : BaseClass (other), font (other.font), state (other.state),
              transparencyLayer (other.transparencyLayer),
              previousTarget (createCopyIfNotNull (other.previousTarget.get()))
        */
    }
    
    pub fn begin_transparency_layer(&mut self, opacity: f32) -> *mut SavedState {
        
        todo!();
        /*
            auto* s = new SavedState (*this);

            if (clip != nullptr)
            {
                auto clipBounds = clip->getClipBounds();

                state->flush();
                s->transparencyLayer = Image (OpenGLImageType().create (Image::ARGB, clipBounds.getWidth(), clipBounds.getHeight(), true));
                s->previousTarget.reset (new Target (state->target));
                state->target = Target (state->target.context, *OpenGLImageType::getFrameBufferFrom (s->transparencyLayer), clipBounds.getPosition());
                s->transparencyLayerAlpha = opacity;
                s->cloneClipIfMultiplyReferenced();

                s->state->target.makeActive();
            }

            return s;
        */
    }
    
    pub fn end_transparency_layer(&mut self, finished_layer_state: &mut SavedState)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                jassert (finishedLayerState.previousTarget != nullptr);

                state->flush();
                state->target = *finishedLayerState.previousTarget;
                finishedLayerState.previousTarget.reset();

                state->target.makeActive();
                auto clipBounds = clip->getClipBounds();

                clip->renderImageUntransformed (*this, finishedLayerState.transparencyLayer,
                                                (int) (finishedLayerState.transparencyLayerAlpha * 255.0f),
                                                clipBounds.getX(), clipBounds.getY(), false);
            }
        */
    }
    
    pub fn draw_glyph(
        &mut self, 
        glyph_number: i32,
        trans:        &AffineTransform

    ) {
        
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

                    const std::unique_ptr<EdgeTable> et (font.getTypeface()->getEdgeTableForGlyph (glyphNumber, t, fontHeight));

                    if (et != nullptr)
                        fillShape (*new EdgeTableRegionType (*et), false);
                }
            }
        */
    }
    
    pub fn get_maximum_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return state->target.bounds;
        */
    }
    
    pub fn set_fill_type(&mut self, new_fill: &FillType)  {
        
        todo!();
        /*
            BaseClass::setFillType (newFill);
            state->textureCache.resetGradient();
        */
    }
    
    pub fn render_image_transformed<IteratorType>(
        &self, 
        iter:       &mut IteratorType,
        src:        &Image,
        alpha:      i32,
        trans:      &AffineTransform,
        _4:         GraphicsResamplingQuality,
        tiled_fill: bool

    ) {
    
        todo!();
        /*
            state->shaderQuadQueue.flush();
            state->setShaderForTiledImageFill (state->cachedImageList->getTextureFor (src), trans, 0, nullptr, tiledFill);

            state->shaderQuadQueue.add (iter, PixelARGB ((uint8) alpha, (uint8) alpha, (uint8) alpha, (uint8) alpha));
            state->shaderQuadQueue.flush();

            state->currentShader.clearShader (state->shaderQuadQueue);
        */
    }
    
    pub fn render_image_untransformed<IteratorType>(
        &self, 
        iter:       &mut IteratorType,
        src:        &Image,
        alpha:      i32,
        x:          i32,
        y:          i32,
        tiled_fill: bool

    ) {
    
        todo!();
        /*
            renderImageTransformed (iter, src, alpha, AffineTransform::translation ((float) x, (float) y),
                                    Graphics::lowResamplingQuality, tiledFill);
        */
    }
    
    
    pub fn fill_with_solid_colour<IteratorType>(
        &self, 
        iter:             &mut IteratorType,
        colour:           PixelARGB,
        replace_contents: bool

    ) {
    
        todo!();
        /*
            if (! isUsingCustomShader)
            {
                state->activeTextures.disableTextures (state->shaderQuadQueue);
                state->blendMode.setBlendMode (state->shaderQuadQueue, replaceContents);
                state->setShader (state->currentShader.programs->solidColourProgram);
            }

            state->shaderQuadQueue.add (iter, colour);
        */
    }
    
    pub fn fill_with_gradient<IteratorType>(
        &self, 
        iter:        &mut IteratorType,
        gradient:    &mut ColourGradient,
        trans:       &AffineTransform,
        is_identity: bool

    ) {
    
        todo!();
        /*
            state->setShaderForGradientFill (gradient, trans, 0, nullptr);
            state->shaderQuadQueue.add (iter, fillType.colour.getPixelARGB());
        */
    }
    
    pub fn fill_rect_with_custom_shader(
        &mut self, 
        shader: &mut ShaderBase,
        area:   Rectangle<i32>

    ) {
        
        todo!();
        /*
            state->setShader (shader);
            isUsingCustomShader = true;

            fillRect (area, true);

            isUsingCustomShader = false;
            state->currentShader.clearShader (state->shaderQuadQueue);
        */
    }
    
    pub fn assign_from(&mut self, _0: &SavedState) -> &mut SavedState {
        
        todo!();
        /*
        
        */
    }
}

crate::ix!();

pub struct SavedStateBase<SavedStateType> {
    clip:                     SavedStateBaseRegionType,
    transform:                TranslationOrTransform,
    fill_type:                FillType,
    interpolation_quality:    GraphicsResamplingQuality,
    transparency_layer_alpha: f32,

    phantom:                  PhantomData<SavedStateType>,
}

pub type SavedStateBaseRegionType              = ClipRegionsBase;
pub type SavedStateBaseRegionTypePtr           = ClipRegionsBasePtr;
pub type SavedStateBaseEdgeTableRegionType     = EdgeTableRegion;
pub type SavedStateBaseRectangleListRegionType = RectangleListRegion;

impl<SavedStateType> SavedStateBase<SavedStateType> {

    pub fn new_from_rect_i32(initial_clip: Rectangle<i32>) -> Self {
    
        todo!();
        /*


            : clip (new RectangleListRegionType (initialClip)),
              interpolationQuality (typename Graphics::mediumResamplingQuality), transparencyLayerAlpha (1.0f)
        */
    }
    
    pub fn new(
        clip_list: &RectangleList<i32>,
        origin:    Point<i32>) -> Self {
    
        todo!();
        /*


            : clip (new RectangleListRegionType (clipList)), transform (origin),
              interpolationQuality (typename Graphics::mediumResamplingQuality), transparencyLayerAlpha (1.0f)
        */
    }
    
    pub fn new_from_other(other: &SavedStateBase<SavedStateType>) -> Self {
    
        todo!();
        /*


            : clip (other.clip), transform (other.transform), fillType (other.fillType),
              interpolationQuality (other.interpolationQuality),
              transparencyLayerAlpha (other.transparencyLayerAlpha)
        */
    }
    
    pub fn get_this(&mut self) -> &mut SavedStateType {
        
        todo!();
        /*
            return *static_cast<SavedStateType*> (this);
        */
    }
    
    pub fn clip_to_rectangle(&mut self, r: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (transform.isOnlyTranslated)
                {
                    cloneClipIfMultiplyReferenced();
                    clip = clip->clipToRectangle (transform.translated (r));
                }
                else if (! transform.isRotated)
                {
                    cloneClipIfMultiplyReferenced();
                    clip = clip->clipToRectangle (transform.transformed (r));
                }
                else
                {
                    Path p;
                    p.addRectangle (r);
                    clipToPath (p, {});
                }
            }

            return clip != nullptr;
        */
    }
    
    pub fn clip_to_rectangle_list(&mut self, r: &RectangleList<i32>) -> bool {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (transform.isOnlyTranslated)
                {
                    cloneClipIfMultiplyReferenced();

                    if (transform.isIdentity())
                    {
                        clip = clip->clipToRectangleList (r);
                    }
                    else
                    {
                        RectangleList<int> offsetList (r);
                        offsetList.offsetAll (transform.offset);
                        clip = clip->clipToRectangleList (offsetList);
                    }
                }
                else if (! transform.isRotated)
                {
                    cloneClipIfMultiplyReferenced();
                    RectangleList<int> scaledList;

                    for (auto& i : r)
                        scaledList.add (transform.transformed (i));

                    clip = clip->clipToRectangleList (scaledList);
                }
                else
                {
                    clipToPath (r.toPath(), {});
                }
            }

            return clip != nullptr;
        */
    }
    
    pub fn get_largest_integer_within(r: Rectangle<f32>) -> Rectangle<i32> {
        
        todo!();
        /*
            auto x1 = (int) std::ceil (r.getX());
            auto y1 = (int) std::ceil (r.getY());
            auto x2 = (int) std::floor (r.getRight());
            auto y2 = (int) std::floor (r.getBottom());

            return { x1, y1, x2 - x1, y2 - y1 };
        */
    }
    
    pub fn exclude_clip_rectangle(&mut self, r: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                cloneClipIfMultiplyReferenced();

                if (transform.isOnlyTranslated)
                {
                    clip = clip->excludeClipRectangle (getLargestIntegerWithin (transform.translated (r.toFloat())));
                }
                else if (! transform.isRotated)
                {
                    clip = clip->excludeClipRectangle (getLargestIntegerWithin (transform.transformed (r.toFloat())));
                }
                else
                {
                    Path p;
                    p.addRectangle (r.toFloat());
                    p.applyTransform (transform.complexTransform);
                    p.addRectangle (clip->getClipBounds().toFloat());
                    p.setUsingNonZeroWinding (false);
                    clip = clip->clipToPath (p, {});
                }
            }

            return clip != nullptr;
        */
    }
    
    pub fn clip_to_path(&mut self, 
        p: &Path,
        t: &AffineTransform)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                cloneClipIfMultiplyReferenced();
                clip = clip->clipToPath (p, transform.getTransformWith (t));
            }
        */
    }
    
    pub fn clip_to_image_alpha(&mut self, 
        source_image: &Image,
        t:            &AffineTransform)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (sourceImage.hasAlphaChannel())
                {
                    cloneClipIfMultiplyReferenced();
                    clip = clip->clipToImageAlpha (sourceImage, transform.getTransformWith (t), interpolationQuality);
                }
                else
                {
                    Path p;
                    p.addRectangle (sourceImage.getBounds());
                    clipToPath (p, t);
                }
            }
        */
    }
    
    pub fn clip_region_intersects(&self, r: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (transform.isOnlyTranslated)
                    return clip->clipRegionIntersects (transform.translated (r));

                return getClipBounds().intersects (r);
            }

            return false;
        */
    }
    
    pub fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return clip != nullptr ? transform.deviceSpaceToUserSpace (clip->getClipBounds())
                                   : Rectangle<int>();
        */
    }
    
    pub fn set_fill_type(&mut self, new_fill: &FillType)  {
        
        todo!();
        /*
            fillType = newFill;
        */
    }
    
    pub fn fill_target_rect(
        &mut self, 
        r:                Rectangle<i32>,
        replace_contents: bool)  {
        
        todo!();
        /*
            if (fillType.isColour())
            {
                clip->fillRectWithColour (getThis(), r, fillType.colour.getPixelARGB(), replaceContents);
            }
            else
            {
                auto clipped = clip->getClipBounds().getIntersection (r);

                if (! clipped.isEmpty())
                    fillShape (*new RectangleListRegionType (clipped), false);
            }
        */
    }
    
    pub fn fill_target_rect_f32(&mut self, r: Rectangle<f32>)  {
        
        todo!();
        /*
            if (fillType.isColour())
            {
                clip->fillRectWithColour (getThis(), r, fillType.colour.getPixelARGB());
            }
            else
            {
                auto clipped = clip->getClipBounds().toFloat().getIntersection (r);

                if (! clipped.isEmpty())
                    fillShape (*new EdgeTableRegionType (clipped), false);
            }
        */
    }
    
    pub fn fill_rect_as_path<CoordType: Copy>(&mut self, r: Rectangle<CoordType>)  {
    
        todo!();
        /*
            Path p;
            p.addRectangle (r);
            fillPath (p, {});
        */
    }
    
    pub fn fill_rect(
        &mut self, 
        r:                Rectangle<i32>,
        replace_contents: bool

    ) {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (transform.isOnlyTranslated)
                {
                    fillTargetRect (transform.translated (r), replaceContents);
                }
                else if (! transform.isRotated)
                {
                    fillTargetRect (transform.transformed (r), replaceContents);
                }
                else
                {
                    jassert (! replaceContents); // not implemented..
                    fillRectAsPath (r);
                }
            }
        */
    }
    
    pub fn fill_rect_f32(&mut self, r: Rectangle<f32>)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (transform.isOnlyTranslated)
                    fillTargetRect (transform.translated (r));
                else if (! transform.isRotated)
                    fillTargetRect (transform.transformed (r));
                else
                    fillRectAsPath (r);
            }
        */
    }
    
    pub fn fill_rect_list(&mut self, list: &RectangleList<f32>)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                if (list.getNumRectangles() == 1)
                    return fillRect (*list.begin());

                if (transform.isIdentity())
                {
                    fillShape (*new EdgeTableRegionType (list), false);
                }
                else if (! transform.isRotated)
                {
                    RectangleList<float> transformed (list);

                    if (transform.isOnlyTranslated)
                        transformed.offsetAll (transform.offset.toFloat());
                    else
                        transformed.transformAll (transform.getTransform());

                    fillShape (*new EdgeTableRegionType (transformed), false);
                }
                else
                {
                    fillPath (list.toPath(), {});
                }
            }
        */
    }
    
    pub fn fill_path(&mut self, 
        path: &Path,
        t:    &AffineTransform)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                auto trans = transform.getTransformWith (t);
                auto clipRect = clip->getClipBounds();

                if (path.getBoundsTransformed (trans).getSmallestIntegerContainer().intersects (clipRect))
                    fillShape (*new EdgeTableRegionType (clipRect, path, trans), false);
            }
        */
    }
    
    pub fn fill_edge_table(&mut self, 
        edge_table: &EdgeTable,
        x:          f32,
        y:          i32)  {
        
        todo!();
        /*
            if (clip != nullptr)
            {
                auto* edgeTableClip = new EdgeTableRegionType (edgeTable);
                edgeTableClip->edgeTable.translate (x, y);

                if (fillType.isColour())
                {
                    auto brightness = fillType.colour.getBrightness() - 0.5f;

                    if (brightness > 0.0f)
                        edgeTableClip->edgeTable.multiplyLevels (1.0f + 1.6f * brightness);
                }

                fillShape (*edgeTableClip, false);
            }
        */
    }
    
    pub fn draw_line(&mut self, line: Line<f32>)  {
        
        todo!();
        /*
            Path p;
            p.addLineSegment (line, 1.0f);
            fillPath (p, {});
        */
    }
    
    pub fn draw_image(&mut self, 
        source_image: &Image,
        trans:        &AffineTransform)  {
        
        todo!();
        /*
            if (clip != nullptr && ! fillType.colour.isTransparent())
                renderImage (sourceImage, trans, {});
        */
    }
    
    pub fn is_only_translation_allowing_error(
        t:         &AffineTransform,
        tolerance: f32) -> bool {
        
        todo!();
        /*
            return std::abs (t.mat01) < tolerance
                && std::abs (t.mat10) < tolerance
                && std::abs (t.mat00 - 1.0f) < tolerance
                && std::abs (t.mat11 - 1.0f) < tolerance;
        */
    }
    
    pub fn render_image(
        &mut self, 
        source_image:           &Image,
        trans:                  &AffineTransform,
        tiled_fill_clip_region: *const SavedStateBaseRegionType

    ) {
        
        todo!();
        /*
            auto t = transform.getTransformWith (trans);
            auto alpha = fillType.colour.getAlpha();

            if (isOnlyTranslationAllowingError (t, 0.002f))
            {
                // If our translation doesn't involve any distortion, just use a simple blit..
                auto tx = (int) (t.getTranslationX() * 256.0f);
                auto ty = (int) (t.getTranslationY() * 256.0f);

                if (interpolationQuality == typename Graphics::lowResamplingQuality || ((tx | ty) & 224) == 0)
                {
                    tx = ((tx + 128) >> 8);
                    ty = ((ty + 128) >> 8);

                    if (tiledFillClipRegion != nullptr)
                    {
                        tiledFillClipRegion->renderImageUntransformed (getThis(), sourceImage, alpha, tx, ty, true);
                    }
                    else
                    {
                        Rectangle<int> area (tx, ty, sourceImage.getWidth(), sourceImage.getHeight());
                        area = area.getIntersection (getThis().getMaximumBounds());

                        if (! area.isEmpty())
                            if (auto c = clip->applyClipTo (*new EdgeTableRegionType (area)))
                                c->renderImageUntransformed (getThis(), sourceImage, alpha, tx, ty, false);
                    }

                    return;
                }
            }

            if (! t.isSingularity())
            {
                if (tiledFillClipRegion != nullptr)
                {
                    tiledFillClipRegion->renderImageTransformed (getThis(), sourceImage, alpha,
                                                                 t, interpolationQuality, true);
                }
                else
                {
                    Path p;
                    p.addRectangle (sourceImage.getBounds());

                    if (auto c = clip->clone()->clipToPath (p, t))
                        c->renderImageTransformed (getThis(), sourceImage, alpha,
                                                   t, interpolationQuality, false);
                }
            }
        */
    }
    
    pub fn fill_shape(
        &mut self, 
        shape_to_fill:    SavedStateBaseRegionType,
        replace_contents: bool

    ) {
        
        todo!();
        /*
            jassert (clip != nullptr);
            shapeToFill = clip->applyClipTo (shapeToFill);

            if (shapeToFill != nullptr)
            {
                if (fillType.isGradient())
                {
                    jassert (! replaceContents); // that option is just for solid colours

                    auto g2 = *(fillType.gradient);
                    g2.multiplyOpacity (fillType.getOpacity());
                    auto t = transform.getTransformWith (fillType.transform).translated (-0.5f, -0.5f);

                    bool isIdentity = t.isOnlyTranslation();

                    if (isIdentity)
                    {
                        // If our translation doesn't involve any distortion, we can speed it up..
                        g2.point1.applyTransform (t);
                        g2.point2.applyTransform (t);
                        t = {};
                    }

                    shapeToFill->fillAllWithGradient (getThis(), g2, t, isIdentity);
                }
                else if (fillType.isTiledImage())
                {
                    renderImage (fillType.image, fillType.transform, shapeToFill.get());
                }
                else
                {
                    shapeToFill->fillAllWithColour (getThis(), fillType.colour.getPixelARGB(), replaceContents);
                }
            }
        */
    }
    
    pub fn clone_clip_if_multiply_referenced(&mut self)  {
        
        todo!();
        /*
            if (clip->getReferenceCount() > 1)
                clip = clip->clone();
        */
    }
}

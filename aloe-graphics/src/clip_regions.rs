crate::ix!();

pub struct ClipRegions<SavedStateType> {
    phantom: PhantomData<SavedStateType>,
}

pub struct ClipRegionsBase {
    base: SingleThreadedReferenceCountedObject,
}

pub type ClipRegionsBasePtr = ReferenceCountedObjectPtr<ClipRegionsBase>;

pub trait ClipRegionsBaseInterface<SavedStateType> {

    fn clone(&self) -> ClipRegionsBasePtr;

    fn apply_clip_to(&self, target: &ClipRegionsBasePtr) -> ClipRegionsBasePtr;

    fn clip_to_rectangle(&mut self, _0: Rectangle<i32>) -> ClipRegionsBasePtr;

    fn clip_to_rectangle_list(&mut self, _0: &RectangleList<i32>) -> ClipRegionsBasePtr;

    fn exclude_clip_rectangle(&mut self, _0: Rectangle<i32>) -> ClipRegionsBasePtr;

    fn clip_to_path(&mut self, 
        _0: &Path,
        _1: &AffineTransform) -> ClipRegionsBasePtr;

    fn clip_to_edge_table(&mut self, _0: &EdgeTable) -> ClipRegionsBasePtr;

    fn clip_to_image_alpha(&mut self, 
        _0: &Image,
        _1: &AffineTransform,
        _2: GraphicsResamplingQuality) -> ClipRegionsBasePtr;

    fn translate(&mut self, delta: Point<i32>);

    fn clip_region_intersects(&self, _0: Rectangle<i32>) -> bool;

    fn get_clip_bounds(&self) -> Rectangle<i32>;

    fn fill_rect_with_colour_maybe_replace_contents(&self, 
        _0:               &mut SavedStateType,
        _1:               Rectangle<i32>,
        colour:           PixelARGB,
        replace_contents: bool);

    fn fill_rect_with_colour(&self, 
        _0:     &mut SavedStateType,
        _1:     Rectangle<f32>,
        colour: PixelARGB);

    fn fill_all_with_colour_maybe_replace_contents(&self, 
        _0:               &mut SavedStateType,
        colour:           PixelARGB,
        replace_contents: bool);

    fn fill_all_with_gradient(&self, 
        _0:          &mut SavedStateType,
        _1:          &mut ColourGradient,
        _2:          &AffineTransform,
        is_identity: bool);

    fn render_image_transformed(&self, 
        _0:         &mut SavedStateType,
        _1:         &Image,
        alpha:      i32,
        _3:         &AffineTransform,
        _4:         GraphicsResamplingQuality,
        tiled_fill: bool);

    fn render_image_untransformed(&self, 
        _0:         &mut SavedStateType,
        _1:         &Image,
        alpha:      i32,
        x:          i32,
        y:          i32,
        tiled_fill: bool);
}

//---------------------------------------------
pub struct EdgeTableRegion {
    base:       ClipRegionsBase,
    edge_table: EdgeTable,
}

pub type EdgeTableRegionPtr = ClipRegionsBasePtr;

impl EdgeTableRegion {
    
    pub fn new_from_edge_table(e: &EdgeTable) -> Self {
    
        todo!();
        /*
        : edge_table(e),

        
        */
    }
    
    pub fn new_from_rect_i32(r: Rectangle<i32>) -> Self {
    
        todo!();
        /*
        : edge_table(r),

        
        */
    }
    
    pub fn new_from_rect_f32(r: Rectangle<f32>) -> Self {
    
        todo!();
        /*
        : edge_table(r),

        
        */
    }
    
    pub fn new_from_rectangle_list_i32(r: &RectangleList<i32>) -> Self {
    
        todo!();
        /*
        : edge_table(r),

        
        */
    }
    
    pub fn new_from_rectangle_list_f32(r: &RectangleList<f32>) -> Self {
    
        todo!();
        /*
        : edge_table(r),

        
        */
    }
    
    pub fn new_from_bounds_path_and_affine_transform(
        bounds: Rectangle<i32>,
        p:      &Path,
        t:      &AffineTransform) -> Self {
    
        todo!();
        /*
        : edge_table(bounds, p, t),

        
        */
    }
    
    pub fn new_from_edge_table_region(other: &EdgeTableRegion) -> Self {
    
        todo!();
        /*
        : base(),
        : edge_table(other.edgeTable),

        
        */
    }
    
    pub fn clone(&self) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            return *new EdgeTableRegion (*this);
        */
    }
    
    pub fn apply_clip_to(&self, target: &EdgeTableRegionPtr) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            return target->clipToEdgeTable (edgeTable);
        */
    }
    
    pub fn clip_to_rectangle(&mut self, r: Rectangle<i32>) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            edgeTable.clipToRectangle (r);
                return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
        */
    }
    
    pub fn clip_to_rectangle_list(&mut self, r: &RectangleList<i32>) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            RectangleList<int> inverse (edgeTable.getMaximumBounds());

                if (inverse.subtract (r))
                    for (auto& i : inverse)
                        edgeTable.excludeRectangle (i);

                return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
        */
    }
    
    pub fn exclude_clip_rectangle(&mut self, r: Rectangle<i32>) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            edgeTable.excludeRectangle (r);
                return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
        */
    }
    
    pub fn clip_to_path(&mut self, 
        p:         &Path,
        transform: &AffineTransform) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            EdgeTable et (edgeTable.getMaximumBounds(), p, transform);
                edgeTable.clipToEdgeTable (et);
                return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
        */
    }
    
    pub fn clip_to_edge_table(&mut self, et: &EdgeTable) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            edgeTable.clipToEdgeTable (et);
                return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
        */
    }
    
    pub fn clip_to_image_alpha(&mut self, 
        image:     &Image,
        transform: &AffineTransform,
        quality:   GraphicsResamplingQuality) -> EdgeTableRegionPtr {
        
        todo!();
        /*
            const typename ImageBitmapData srcData (image, typename ImageBitmapData::readOnly);

                if (transform.isOnlyTranslation())
                {
                    // If our translation doesn't involve any distortion, just use a simple blit..
                    auto tx = (int) (transform.getTranslationX() * 256.0f);
                    auto ty = (int) (transform.getTranslationY() * 256.0f);

                    if (quality == typename Graphics::lowResamplingQuality || ((tx | ty) & 224) == 0)
                    {
                        auto imageX = ((tx + 128) >> 8);
                        auto imageY = ((ty + 128) >> 8);

                        if (image.getFormat() == typename Image::ARGB)
                            straightClipImage (srcData, imageX, imageY, (PixelARGB*) nullptr);
                        else
                            straightClipImage (srcData, imageX, imageY, (PixelAlpha*) nullptr);

                        return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
                    }
                }

                if (transform.isSingularity())
                    return EdgeTableRegionPtr();

                {
                    Path p;
                    p.addRectangle (0, 0, (float) srcData.width, (float) srcData.height);
                    EdgeTable et2 (edgeTable.getMaximumBounds(), p, transform);
                    edgeTable.clipToEdgeTable (et2);
                }

                if (! edgeTable.isEmpty())
                {
                    if (image.getFormat() == typename Image::ARGB)
                        transformedClipImage (srcData, transform, quality, (PixelARGB*) nullptr);
                    else
                        transformedClipImage (srcData, transform, quality, (PixelAlpha*) nullptr);
                }

                return edgeTable.isEmpty() ? EdgeTableRegionPtr() : EdgeTableRegionPtr (*this);
        */
    }
    
    pub fn translate(&mut self, delta: Point<i32>)  {
        
        todo!();
        /*
            edgeTable.translate ((float) delta.x, delta.y);
        */
    }
    
    pub fn clip_region_intersects(&self, r: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            return edgeTable.getMaximumBounds().intersects (r);
        */
    }
    
    pub fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return edgeTable.getMaximumBounds();
        */
    }
    
    pub fn fill_rect_with_colour_maybe_replace_contents<SavedStateType>(
        &self, 
        state:            &mut SavedStateType,
        area:             Rectangle<i32>,
        colour:           PixelARGB,
        replace_contents: bool)  {
        
        todo!();
        /*
            auto totalClip = edgeTable.getMaximumBounds();
                auto clipped = totalClip.getIntersection (area);

                if (! clipped.isEmpty())
                {
                    EdgeTableRegion et (clipped);
                    et.edgeTable.clipToEdgeTable (edgeTable);
                    state.fillWithSolidColour (et.edgeTable, colour, replaceContents);
                }
        */
    }
    
    pub fn fill_rect_with_colour<SavedStateType>(
        &self, 
        state:  &mut SavedStateType,
        area:   Rectangle<f32>,
        colour: PixelARGB)  {
        
        todo!();
        /*
            auto totalClip = edgeTable.getMaximumBounds().toFloat();
                auto clipped = totalClip.getIntersection (area);

                if (! clipped.isEmpty())
                {
                    EdgeTableRegion et (clipped);
                    et.edgeTable.clipToEdgeTable (edgeTable);
                    state.fillWithSolidColour (et.edgeTable, colour, false);
                }
        */
    }
    
    pub fn fill_all_with_colour<SavedStateType>(
        &self, 
        state:            &mut SavedStateType,
        colour:           PixelARGB,
        replace_contents: bool)  {
        
        todo!();
        /*
            state.fillWithSolidColour (edgeTable, colour, replaceContents);
        */
    }
    
    pub fn fill_all_with_gradient<SavedStateType>(
        &self, 
        state:       &mut SavedStateType,
        gradient:    &mut ColourGradient,
        transform:   &AffineTransform,
        is_identity: bool)  {
        
        todo!();
        /*
            state.fillWithGradient (edgeTable, gradient, transform, isIdentity);
        */
    }
    
    pub fn render_image_transformed<SavedStateType>(
        &self, 
        state:      &mut SavedStateType,
        src:        &Image,
        alpha:      i32,
        transform:  &AffineTransform,
        quality:    GraphicsResamplingQuality,
        tiled_fill: bool)  {
        
        todo!();
        /*
            state.renderImageTransformed (edgeTable, src, alpha, transform, quality, tiledFill);
        */
    }
    
    pub fn render_image_untransformed<SavedStateType>(
        &self, 
        state:      &mut SavedStateType,
        src:        &Image,
        alpha:      i32,
        x:          i32,
        y:          i32,
        tiled_fill: bool)  {
        
        todo!();
        /*
            state.renderImageUntransformed (edgeTable, src, alpha, x, y, tiledFill);
        */
    }
    
    pub fn transformed_clip_image<SrcPixelType>(&mut self, 
        src_data:  &ImageBitmapData,
        transform: &AffineTransform,
        quality:   GraphicsResamplingQuality,
        _3:        *const SrcPixelType)  {
    
        todo!();
        /*
            EdgeTableFillers::TransformedImageFill<SrcPixelType, SrcPixelType, false> renderer (srcData, srcData, transform, 255, quality);

                for (int y = 0; y < edgeTable.getMaximumBounds().getHeight(); ++y)
                    renderer.clipEdgeTableLine (edgeTable, edgeTable.getMaximumBounds().getX(), y + edgeTable.getMaximumBounds().getY(),
                                                edgeTable.getMaximumBounds().getWidth());
        */
    }
    
    pub fn straight_clip_image<SrcPixelType>(&mut self, 
        src_data: &ImageBitmapData,
        imagex:   i32,
        imagey:   i32,
        _3:       *const SrcPixelType)  {
    
        todo!();
        /*
            Rectangle<int> r (imageX, imageY, srcData.width, srcData.height);
                edgeTable.clipToRectangle (r);

                EdgeTableFillers::ImageFill<SrcPixelType, SrcPixelType, false> renderer (srcData, srcData, 255, imageX, imageY);

                for (int y = 0; y < r.getHeight(); ++y)
                    renderer.clipEdgeTableLine (edgeTable, r.getX(), y + r.getY(), r.getWidth());
        */
    }
}

//----------------------------------------------------
pub struct RectangleListRegion {
    base: ClipRegionsBase,
    clip: RectangleList<i32>,
}

pub type RectangleListRegionPtr = ClipRegionsBasePtr;

#[no_copy]
pub struct RectangleListRegionSubRectangleIterator<'a> {
    clip: &'a RectangleList<i32>,
    area: Rectangle<i32>,
}

impl<'a> RectangleListRegionSubRectangleIterator<'a> {

    pub fn new(
        clip_list:   &RectangleList<i32>,
        clip_bounds: Rectangle<i32>) -> Self {
    
        todo!();
        /*
        : clip(clipList),
        : area(clipBounds),

        
        */
    }
    
    
    pub fn iterate<Renderer>(&self, r: &mut Renderer)  {
    
        todo!();
        /*
            for (auto& i : clip)
                    {
                        auto rect = i.getIntersection (area);

                        if (! rect.isEmpty())
                            r.handleEdgeTableRectangleFull (rect.getX(), rect.getY(), rect.getWidth(), rect.getHeight());
                    }
        */
    }
}

///-----------------------
#[no_copy]
pub struct RectangleListRegionSubRectangleIteratorFloat<'a> {
    clip: &'a RectangleList<i32>,
    area: Rectangle<f32>,
}

impl<'a> RectangleListRegionSubRectangleIteratorFloat<'a> {

    pub fn new(
        clip_list:   &RectangleList<i32>,
        clip_bounds: Rectangle<f32>) -> Self {
    
        todo!();
        /*
        : clip(clipList),
        : area(clipBounds),

        
        */
    }
    
    pub fn iterate<Renderer>(&self, r: &mut Renderer)  {
    
        todo!();
        /*
            const RenderingHelpers::FloatRectangleRasterisingInfo f (area);

                    for (auto& i : clip)
                    {
                        auto clipLeft   = i.getX();
                        auto clipRight  = i.getRight();
                        auto clipTop    = i.getY();
                        auto clipBottom = i.getBottom();

                        if (f.totalBottom > clipTop && f.totalTop < clipBottom
                             && f.totalRight > clipLeft && f.totalLeft < clipRight)
                        {
                            if (f.isOnePixelWide())
                            {
                                if (f.topAlpha != 0 && f.totalTop >= clipTop)
                                {
                                    r.setEdgeTableYPos (f.totalTop);
                                    r.handleEdgeTablePixel (f.left, f.topAlpha);
                                }

                                auto y1 = jmax (clipTop, f.top);
                                auto y2 = jmin (f.bottom, clipBottom);
                                auto h = y2 - y1;

                                if (h > 0)
                                    r.handleEdgeTableRectangleFull (f.left, y1, 1, h);

                                if (f.bottomAlpha != 0 && f.bottom < clipBottom)
                                {
                                    r.setEdgeTableYPos (f.bottom);
                                    r.handleEdgeTablePixel (f.left, f.bottomAlpha);
                                }
                            }
                            else
                            {
                                auto clippedLeft   = jmax (f.left, clipLeft);
                                auto clippedWidth  = jmin (f.right, clipRight) - clippedLeft;
                                bool doLeftAlpha  = f.leftAlpha != 0 && f.totalLeft >= clipLeft;
                                bool doRightAlpha = f.rightAlpha != 0 && f.right < clipRight;

                                if (f.topAlpha != 0 && f.totalTop >= clipTop)
                                {
                                    r.setEdgeTableYPos (f.totalTop);

                                    if (doLeftAlpha)        r.handleEdgeTablePixel (f.totalLeft, f.getTopLeftCornerAlpha());
                                    if (clippedWidth > 0)   r.handleEdgeTableLine (clippedLeft, clippedWidth, f.topAlpha);
                                    if (doRightAlpha)       r.handleEdgeTablePixel (f.right, f.getTopRightCornerAlpha());
                                }

                                auto y1 = jmax (clipTop, f.top);
                                auto y2 = jmin (f.bottom, clipBottom);
                                auto h = y2 - y1;

                                if (h > 0)
                                {
                                    if (h == 1)
                                    {
                                        r.setEdgeTableYPos (y1);

                                        if (doLeftAlpha)        r.handleEdgeTablePixel (f.totalLeft, f.leftAlpha);
                                        if (clippedWidth > 0)   r.handleEdgeTableLineFull (clippedLeft, clippedWidth);
                                        if (doRightAlpha)       r.handleEdgeTablePixel (f.right, f.rightAlpha);
                                    }
                                    else
                                    {
                                        if (doLeftAlpha)        r.handleEdgeTableRectangle (f.totalLeft, y1, 1, h, f.leftAlpha);
                                        if (clippedWidth > 0)   r.handleEdgeTableRectangleFull (clippedLeft, y1, clippedWidth, h);
                                        if (doRightAlpha)       r.handleEdgeTableRectangle (f.right, y1, 1, h, f.rightAlpha);
                                    }
                                }

                                if (f.bottomAlpha != 0 && f.bottom < clipBottom)
                                {
                                    r.setEdgeTableYPos (f.bottom);

                                    if (doLeftAlpha)        r.handleEdgeTablePixel (f.totalLeft, f.getBottomLeftCornerAlpha());
                                    if (clippedWidth > 0)   r.handleEdgeTableLine (clippedLeft, clippedWidth, f.bottomAlpha);
                                    if (doRightAlpha)       r.handleEdgeTablePixel (f.right, f.getBottomRightCornerAlpha());
                                }
                            }
                        }
                    }
        */
    }
}

impl RectangleListRegion {

    pub fn new_from_rect_i32(r: Rectangle<i32>) -> Self {
    
        todo!();
        /*
        : clip(r),

        
        */
    }
    
    pub fn new_from_rect_list_i32(r: &RectangleList<i32>) -> Self {
    
        todo!();
        /*
        : clip(r),

        
        */
    }
    
    pub fn new_from_rectangle_list_region(other: &RectangleListRegion) -> Self {
    
        todo!();
        /*
        : base(),
        : clip(other.clip),

        
        */
    }
    
    pub fn clone(&self) -> RectangleListRegionPtr {
        
        todo!();
        /*
            return *new RectangleListRegion (*this);
        */
    }
    
    pub fn apply_clip_to(&self, target: &RectangleListRegionPtr) -> RectangleListRegionPtr {
        
        todo!();
        /*
            return target->clipToRectangleList (clip);
        */
    }
    
    pub fn clip_to_rectangle(&mut self, r: Rectangle<i32>) -> RectangleListRegionPtr {
        
        todo!();
        /*
            clip.clipTo (r);
                return clip.isEmpty() ? Ptr() : Ptr (*this);
        */
    }
    
    pub fn clip_to_rectangle_list(&mut self, r: &RectangleList<i32>) -> RectangleListRegionPtr {
        
        todo!();
        /*
            clip.clipTo (r);
                return clip.isEmpty() ? Ptr() : Ptr (*this);
        */
    }
    
    pub fn exclude_clip_rectangle(&mut self, r: Rectangle<i32>) -> RectangleListRegionPtr {
        
        todo!();
        /*
            clip.subtract (r);
                return clip.isEmpty() ? Ptr() : Ptr (*this);
        */
    }
    
    pub fn clip_to_path(&mut self, 
        p:         &Path,
        transform: &AffineTransform) -> RectangleListRegionPtr {
        
        todo!();
        /*
            return toEdgeTable()->clipToPath (p, transform);
        */
    }
    
    pub fn clip_to_edge_table(&mut self, et: &EdgeTable) -> RectangleListRegionPtr {
        
        todo!();
        /*
            return toEdgeTable()->clipToEdgeTable (et);
        */
    }
    
    pub fn clip_to_image_alpha(&mut self, 
        image:     &Image,
        transform: &AffineTransform,
        quality:   GraphicsResamplingQuality) -> RectangleListRegionPtr {
        
        todo!();
        /*
            return toEdgeTable()->clipToImageAlpha (image, transform, quality);
        */
    }
    
    pub fn translate(&mut self, delta: Point<i32>)  {
        
        todo!();
        /*
            clip.offsetAll (delta);
        */
    }
    
    pub fn clip_region_intersects(&self, r: Rectangle<i32>) -> bool {
        
        todo!();
        /*
            return clip.intersects (r);
        */
    }
    
    pub fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return clip.getBounds();
        */
    }
    
    pub fn fill_rect_with_colour_maybe_replace_contents<SavedStateType>(
        &self, 
        state:            &mut SavedStateType,
        area:             Rectangle<i32>,
        colour:           PixelARGB,
        replace_contents: bool)  {
        
        todo!();
        /*
            SubRectangleIterator iter (clip, area);
                state.fillWithSolidColour (iter, colour, replaceContents);
        */
    }
    
    pub fn fill_rect_with_colour<SavedStateType>(
        &self, 
        state:  &mut SavedStateType,
        area:   Rectangle<f32>,
        colour: PixelARGB)  {
        
        todo!();
        /*
            SubRectangleIteratorFloat iter (clip, area);
                state.fillWithSolidColour (iter, colour, false);
        */
    }
    
    pub fn fill_all_with_colour<SavedStateType>(
        &self, 
        state:            &mut SavedStateType,
        colour:           PixelARGB,
        replace_contents: bool)  {
        
        todo!();
        /*
            state.fillWithSolidColour (*this, colour, replaceContents);
        */
    }
    
    pub fn fill_all_with_gradient<SavedStateType>(
        &self, 
        state:       &mut SavedStateType,
        gradient:    &mut ColourGradient,
        transform:   &AffineTransform,
        is_identity: bool)  {
        
        todo!();
        /*
            state.fillWithGradient (*this, gradient, transform, isIdentity);
        */
    }
    
    pub fn render_image_transformed<SavedStateType>(
        &self, 
        state:      &mut SavedStateType,
        src:        &Image,
        alpha:      i32,
        transform:  &AffineTransform,
        quality:    GraphicsResamplingQuality,
        tiled_fill: bool)  {
        
        todo!();
        /*
            state.renderImageTransformed (*this, src, alpha, transform, quality, tiledFill);
        */
    }
    
    pub fn render_image_untransformed<SavedStateType>(
        &self, 
        state:      &mut SavedStateType,
        src:        &Image,
        alpha:      i32,
        x:          i32,
        y:          i32,
        tiled_fill: bool)  {
        
        todo!();
        /*
            state.renderImageUntransformed (*this, src, alpha, x, y, tiledFill);
        */
    }
    
    pub fn iterate<Renderer>(&self, r: &mut Renderer)  {
    
        todo!();
        /*
            for (auto& i : clip)
                {
                    auto x = i.getX();
                    auto w = i.getWidth();
                    jassert (w > 0);
                    auto bottom = i.getBottom();

                    for (int y = i.getY(); y < bottom; ++y)
                    {
                        r.setEdgeTableYPos (y);
                        r.handleEdgeTableLineFull (x, w);
                    }
                }
        */
    }
    
    pub fn to_edge_table(&self) -> RectangleListRegionPtr {
        
        todo!();
        /*
            return *new EdgeTableRegion (clip);
        */
    }
}

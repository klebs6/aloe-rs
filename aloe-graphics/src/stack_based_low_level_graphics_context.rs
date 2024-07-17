crate::ix!();

#[derive(Default)]
pub struct StackBasedLowLevelGraphicsContext<SavedStateType> {
    stack: SavedStateStack<SavedStateType>,
}

impl<SavedStateType> LowLevelGraphicsContext for StackBasedLowLevelGraphicsContext<SavedStateType> {}

impl<SavedStateType> DrawTextLayout for StackBasedLowLevelGraphicsContext<SavedStateType> {}

impl<SavedStateType> IsVectorDevice for StackBasedLowLevelGraphicsContext<SavedStateType> {
    
    fn is_vector_device(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
impl<SavedStateType> SetOrigin for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn set_origin(&mut self, o: Point<i32>)  {
        
        todo!();
        /*
            stack->transform.setOrigin (o);
        */
    }
}
    
impl<SavedStateType> AddTransform for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn add_transform(&mut self, t: &AffineTransform)  {
        
        todo!();
        /*
            stack->transform.addTransform (t);
        */
    }
}
    
impl<SavedStateType> GetPhysicalPixelScaleFactor for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn get_physical_pixel_scale_factor(&mut self) -> f32 {
        
        todo!();
        /*
            return stack->transform.getPhysicalPixelScaleFactor();
        */
    }
}
    
impl<SavedStateType> GetClipBounds for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return stack->getClipBounds();
        */
    }
}
    
impl<SavedStateType> IsClipEmpty for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn is_clip_empty(&self) -> bool {
        
        todo!();
        /*
            return stack->clip == nullptr;
        */
    }
}
    
impl<SavedStateType> ClipRegionIntersects for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn clip_region_intersects(&mut self, r: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            return stack->clipRegionIntersects (r);
        */
    }
}
    
impl<SavedStateType> ClipToRectangle for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn clip_to_rectangle(&mut self, r: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
            return stack->clipToRectangle (r);
        */
    }
}
    
impl<SavedStateType> ClipToRectangleList for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn clip_to_rectangle_list(&mut self, r: &RectangleList<i32>) -> bool {
        
        todo!();
        /*
            return stack->clipToRectangleList (r);
        */
    }
}
    
impl<SavedStateType> ExcludeClipRectangle for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn exclude_clip_rectangle(&mut self, r: &Rectangle<i32>)  {
        
        todo!();
        /*
            stack->excludeClipRectangle (r);
        */
    }
}
    
impl<SavedStateType> ClipToPath for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn clip_to_path(&mut self, 
        path: &Path,
        t:    &AffineTransform)  {
        
        todo!();
        /*
            stack->clipToPath (path, t);
        */
    }
}
    
impl<SavedStateType> ClipToImageAlpha for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn clip_to_image_alpha(&mut self, 
        im: &Image,
        t:  &AffineTransform)  {
        
        todo!();
        /*
            stack->clipToImageAlpha (im, t);
        */
    }
}
    
impl<SavedStateType> SaveState for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn save_state(&mut self)  {
        
        todo!();
        /*
            stack.save();
        */
    }
}
    
impl<SavedStateType> RestoreState for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn restore_state(&mut self)  {
        
        todo!();
        /*
            stack.restore();
        */
    }
}
    
impl<SavedStateType> BeginTransparencyLayer for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn begin_transparency_layer(&mut self, opacity: f32)  {
        
        todo!();
        /*
            stack.beginTransparencyLayer (opacity);
        */
    }
}
    
impl<SavedStateType> EndTransparencyLayer for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn end_transparency_layer(&mut self)  {
        
        todo!();
        /*
            stack.endTransparencyLayer();
        */
    }
}
    
impl<SavedStateType> SetFill for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn set_fill(&mut self, fill_type: &FillType)  {
        
        todo!();
        /*
            stack->setFillType (fillType);
        */
    }
}
    
impl<SavedStateType> SetOpacity for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn set_opacity(&mut self, new_opacity: f32)  {
        
        todo!();
        /*
            stack->fillType.setOpacity (newOpacity);
        */
    }
}
    
impl<SavedStateType> SetInterpolationQuality for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn set_interpolation_quality(&mut self, quality: GraphicsResamplingQuality)  {
        
        todo!();
        /*
            stack->interpolationQuality = quality;
        */
    }
}
    
impl<SavedStateType> FillRectMaybeReplace for StackBasedLowLevelGraphicsContext<SavedStateType> {

    fn fill_rect_maybe_replace(
        &mut self, 
        r:       &Rectangle<i32>,
        replace: bool
    )  {
        
        todo!();
        /*
            stack->fillRect (r, replace);
        */
    }
}
    
impl<SavedStateType> FillRect for StackBasedLowLevelGraphicsContext<SavedStateType> {

    fn fill_rect(&mut self, r: &Rectangle<f32>)  {
        
        todo!();
        /*
            stack->fillRect (r);
        */
    }
}
    
impl<SavedStateType> FillRectList for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn fill_rect_list(&mut self, list: &RectangleList<f32>)  {
        
        todo!();
        /*
            stack->fillRectList (list);
        */
    }
}
    
impl<SavedStateType> FillPath for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn fill_path(&mut self, 
        path: &Path,
        t:    &AffineTransform)  {
        
        todo!();
        /*
            stack->fillPath (path, t);
        */
    }
}
    
impl<SavedStateType> DrawImage for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn draw_image(&mut self, 
        im: &Image,
        t:  &AffineTransform)  {
        
        todo!();
        /*
            stack->drawImage (im, t);
        */
    }
}
    
impl<SavedStateType> DrawGlyph for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn draw_glyph(&mut self, 
        glyph_number: i32,
        t:            &AffineTransform)  {
        
        todo!();
        /*
            stack->drawGlyph (glyphNumber, t);
        */
    }
}
    
impl<SavedStateType> DrawLine for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn draw_line(&mut self, line: &Line<f32>)  {
        
        todo!();
        /*
            stack->drawLine (line);
        */
    }
}
    
impl<SavedStateType> SetFont for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn set_font(&mut self, new_font: &Font)  {
        
        todo!();
        /*
            stack->font = newFont;
        */
    }
}
    
impl<SavedStateType> GetFont for StackBasedLowLevelGraphicsContext<SavedStateType> {
    fn get_font(&mut self) -> &Font {
        
        todo!();
        /*
            return stack->font;
        */
    }
}
    
impl<SavedStateType> StackBasedLowLevelGraphicsContext<SavedStateType> {

    fn new(initial_state: *mut SavedStateType) -> Self {
    
        todo!();
        /*
        : stack(initialState),
        */
    }
}

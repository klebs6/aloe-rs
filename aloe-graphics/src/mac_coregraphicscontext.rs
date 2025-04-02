crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_mac_CoreGraphicsContext.h]

// Assume the extern "C" functions have been defined or imported appropriately
unsafe extern "C" {

    fn CGColorSpaceRelease(space: *mut libc::c_void);
    fn CGContextRelease(context: *mut libc::c_void);
    fn CGDataProviderRelease(provider: *mut libc::c_void);
    fn CGImageRelease(image: *mut libc::c_void);
    fn CGGradientRelease(gradient: *mut libc::c_void);
}

// Type aliases for the raw Core Graphics types
pub type CGColorSpaceRef = *mut libc::c_void;
pub type CGContextRef = *mut libc::c_void;
pub type CGDataProviderRef = *mut libc::c_void;
pub type CGImageRef = *mut libc::c_void;
pub type CGGradientRef = *mut libc::c_void;

// Custom smart pointer for ColorSpace
pub struct ColorSpacePtr {
    ptr: CGColorSpaceRef,
}

impl Drop for ColorSpacePtr {
    fn drop(&mut self) {
        // Call the appropriate Core Graphics function to release the ColorSpace
        unsafe {
            CGColorSpaceRelease(self.ptr);
        }
    }
}

// Custom smart pointer for Context
pub struct ContextPtr {
    ptr: CGContextRef,
}

impl Drop for ContextPtr {
    fn drop(&mut self) {
        // Call the appropriate Core Graphics function to release the Context
        unsafe {
            CGContextRelease(self.ptr);
        }
    }
}

// Custom smart pointer for DataProvider
pub struct DataProviderPtr {
    ptr: CGDataProviderRef,
}

impl Drop for DataProviderPtr {
    fn drop(&mut self) {
        // Call the appropriate Core Graphics function to release the DataProvider
        unsafe {
            CGDataProviderRelease(self.ptr);
        }
    }
}

// Custom smart pointer for Image
pub struct ImagePtr {
    ptr: CGImageRef,
}

impl Drop for ImagePtr {
    fn drop(&mut self) {
        // Call the appropriate Core Graphics function to release the Image
        unsafe {
            CGImageRelease(self.ptr);
        }
    }
}

// Custom smart pointer for Gradient
pub struct GradientPtr {
    ptr: CGGradientRef,
}

impl Drop for GradientPtr {
    fn drop(&mut self) {
        // Call the appropriate Core Graphics function to release the Gradient
        unsafe {
            CGGradientRelease(self.ptr);
        }
    }
}

//-------------------------------
pub struct CoreGraphicsContextSavedState {
    fill_type:           FillType,
    font:                Font,
    font_ref:            CGFontRef,
    text_matrix:         CGAffineTransform, // default = CGAffineTransformIdentity
    inverse_text_matrix: CGAffineTransform, // default = CGAffineTransformIdentity
    gradient:            GradientPtr,
}

#[no_copy]
#[leak_detector]
pub struct CoreGraphicsContext {
    context:                 ContextPtr,
    flip_height:             CGFloat,
    rgb_colour_space:        ColorSpacePtr,
    grey_colour_space:       ColorSpacePtr,
    last_clip_rect:          RefCell<Rectangle<i32>>,
    last_clip_rect_is_valid: RefCell<bool>, // default = false
    state:                   Box<CoreGraphicsContextSavedState>,
    state_stack:             Vec<CoreGraphicsContextSavedState>,
}

impl LowLevelGraphicsContext for CoreGraphicsContext {}

impl IsVectorDevice for CoreGraphicsContext {
    
    fn is_vector_device(&self) -> bool {
        false
    }
}

impl CoreGraphicsContext {
    
    fn is_vector_device(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
impl SetOrigin for CoreGraphicsContext {
    fn set_origin(&mut self, _0: Point<i32>)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl AddTransform for CoreGraphicsContext {
    fn add_transform(&mut self, _0: &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl GetPhysicalPixelScaleFactor for CoreGraphicsContext {
    fn get_physical_pixel_scale_factor(&mut self) -> f32 {
        
        todo!();
        /*
        
        */
    }
}
    
impl ClipToRectangle for CoreGraphicsContext {
    fn clip_to_rectangle(&mut self, _0: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
        
        */
    }
}
    
impl ClipToRectangleList for CoreGraphicsContext {
    fn clip_to_rectangle_list(&mut self, _0: &RectangleList<i32>) -> bool {
        
        todo!();
        /*
        
        */
    }
}
    
impl ExcludeClipRectangle for CoreGraphicsContext {
    fn exclude_clip_rectangle(&mut self, _0: &Rectangle<i32>)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl ClipToPath for CoreGraphicsContext {
    fn clip_to_path(&mut self, 
        _0: &Path,
        _1: &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl ClipToImageAlpha for CoreGraphicsContext {
    fn clip_to_image_alpha(&mut self, 
        _0: &Image,
        _1: &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl ClipRegionIntersects for CoreGraphicsContext {
    fn clip_region_intersects(&mut self, _0: &Rectangle<i32>) -> bool {
        
        todo!();
        /*
        
        */
    }
}
    
impl GetClipBounds for CoreGraphicsContext {
    fn get_clip_bounds(&self) -> Rectangle<i32> {
        
        todo!();
        /*
        
        */
    }
}
    
impl IsClipEmpty for CoreGraphicsContext {
    fn is_clip_empty(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}
    
impl SaveState for CoreGraphicsContext {
    fn save_state(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl RestoreState for CoreGraphicsContext {
    fn restore_state(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl BeginTransparencyLayer for CoreGraphicsContext {
    fn begin_transparency_layer(&mut self, opacity: f32)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl EndTransparencyLayer for CoreGraphicsContext {
    fn end_transparency_layer(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl SetFill for CoreGraphicsContext {
    fn set_fill(&mut self, _0: &FillType)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl SetOpacity for CoreGraphicsContext {
    fn set_opacity(&mut self, _0: f32)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl SetInterpolationQuality for CoreGraphicsContext {
    fn set_interpolation_quality(&mut self, _0: GraphicsResamplingQuality)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl FillRectMaybeReplace for CoreGraphicsContext {

    fn fill_rect_maybe_replace(
        &mut self, 
        _0:                        &Rectangle<i32>,
        replace_existing_contents: bool)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl FillRect for CoreGraphicsContext {
    fn fill_rect(&mut self, _0: &Rectangle<f32>)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl FillRectList for CoreGraphicsContext {
    fn fill_rect_list(&mut self, _0: &RectangleList<f32>)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl FillPath for CoreGraphicsContext {
    fn fill_path(&mut self, 
        _0: &Path,
        _1: &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl DrawLine for CoreGraphicsContext {
    fn draw_line(&mut self, _0: &Line<f32>)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl SetFont for CoreGraphicsContext {
    fn set_font(&mut self, _0: &Font)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl GetFont for CoreGraphicsContext {
    fn get_font(&mut self) -> &Font {
        
        todo!();
        /*
        
        */
    }
}
    
impl DrawGlyph for CoreGraphicsContext {
    fn draw_glyph(&mut self, 
        glyph_number: i32,
        _1:           &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl DrawTextLayout for CoreGraphicsContext {
    fn draw_text_layout(&mut self, 
        _0: &AttributedString,
        _1: &Rectangle<f32>) -> bool {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait DrawGradient {

    fn draw_gradient(&mut self);
}

impl DrawGradient for CoreGraphicsContext {
    fn draw_gradient(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait CreatePath {

    fn create_path(&self, _0: &Path);
}

impl CreatePath for CoreGraphicsContext {
    fn create_path(&self, _0: &Path)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait CreatePathWithTransform {

    fn create_path(&self, 
        _0: &Path,
        _1: &AffineTransform);
}

impl CreatePathWithTransform for CoreGraphicsContext {

    fn create_path(&self, 
        _0: &Path,
        _1: &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait Flip {

    fn flip(&self);
}

impl Flip for CoreGraphicsContext {

    fn flip(&self)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait ApplyTransform {

    fn apply_transform(&self, _0: &AffineTransform);
}

impl ApplyTransform for CoreGraphicsContext {

    fn apply_transform(&self, _0: &AffineTransform)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl DrawImage for CoreGraphicsContext {

    fn draw_image(
        &mut self, 
        _0:                        &Image,
        _1:                        &AffineTransform,

        /*fill_entire_clip_as_tiles: bool*/

    )  {
        
        todo!();
        /*
        
        */
    }
}
    
impl CoreGraphicsContext {

    pub fn clip_to_rectangle_list_without_test(&mut self, _0: &RectangleList<i32>) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn fill_cg_rect(&mut self, 
        _0:                        &CGRect,
        replace_existing_contents: bool)  {
        
        todo!();
        /*
        
        */
    }
}

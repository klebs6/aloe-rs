crate::ix!();

/**
  | Interface class for graphics context
  | objects, used internally by the Graphics
  | class.
  | 
  | Users are not supposed to create instances
  | of this class directly - do your drawing
  | via the Graphics object instead.
  | 
  | It's a base class for different types
  | of graphics context, that may perform
  | software-based or OS-accelerated
  | rendering.
  | 
  | E.g. the LowLevelGraphicsSoftwareRenderer
  | renders onto an image in memory, but
  | other subclasses could render directly
  | to a windows HDC, a Quartz context, or
  | an OpenGL context.
  | 
  | @tags{Graphics}
  |
  */
pub trait LowLevelGraphicsContext:
IsVectorDevice
+ SetOrigin
+ AddTransform
+ GetPhysicalPixelScaleFactor
+ ClipToRectangle
+ ClipToRectangleList
+ ExcludeClipRectangle
+ ClipToPath
+ ClipToImageAlpha
+ ClipRegionIntersects
+ GetClipBounds
+ IsClipEmpty
+ SaveState
+ RestoreState
+ BeginTransparencyLayer
+ EndTransparencyLayer
+ SetFill
+ SetOpacity
+ SetInterpolationQuality
+ FillRectMaybeReplace
+ FillRect
+ FillRectList
+ FillPath
+ DrawImage
+ DrawLine
+ SetFont
+ GetFont
+ DrawGlyph
+ DrawTextLayout
{

}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/contexts/aloe_LowLevelGraphicsContext.h]

pub trait IsVectorDevice {

    /**
      | Returns true if this device is vector-based,
      | e.g. a printer.
      |
      */
    fn is_vector_device(&self) -> bool;
}

pub trait SetOrigin {

    /**
      | Moves the origin to a new position.
      | 
      | The coordinates are relative to the
      | current origin, and indicate the new
      | position of (0, 0).
      |
      */
    fn set_origin(&mut self, _0: Point<i32>);
}

pub trait AddTransform {

    fn add_transform(&mut self, _0: &AffineTransform);
}

pub trait GetPhysicalPixelScaleFactor {

    fn get_physical_pixel_scale_factor(&mut self) -> f32;
}

pub trait ClipToRectangle {

    fn clip_to_rectangle(&mut self, _0: &Rectangle<i32>) -> bool;
}

pub trait ClipToRectangleList {

    fn clip_to_rectangle_list(&mut self, _0: &RectangleList<i32>) -> bool;
}

pub trait ExcludeClipRectangle {

    fn exclude_clip_rectangle(&mut self, _0: &Rectangle<i32>);
}

pub trait ClipToPath {

    fn clip_to_path(
        &mut self, 
        _0: &Path,
        _1: &AffineTransform
    );
}

pub trait ClipToImageAlpha {

    fn clip_to_image_alpha(&mut self, 
            _0: &Image,
            _1: &AffineTransform);
}

pub trait ClipRegionIntersects {

    fn clip_region_intersects(&mut self, _0: &Rectangle<i32>) -> bool;
}

pub trait GetClipBounds {

    fn get_clip_bounds(&self) -> Rectangle<i32>;
}

pub trait IsClipEmpty {

    fn is_clip_empty(&self) -> bool;
}

pub trait SaveState {

    fn save_state(&mut self);
}

pub trait RestoreState {

    fn restore_state(&mut self);
}

pub trait BeginTransparencyLayer {

    fn begin_transparency_layer(&mut self, opacity: f32);
}

pub trait EndTransparencyLayer {

    fn end_transparency_layer(&mut self);
}
    
pub trait SetFill {

    fn set_fill(&mut self, _0: &FillType);
}

pub trait SetOpacity {

    fn set_opacity(&mut self, _0: f32);
}

pub trait SetInterpolationQuality {

    fn set_interpolation_quality(&mut self, _0: GraphicsResamplingQuality);
}

pub trait FillRectMaybeReplace {

    fn fill_rect_maybe_replace(&mut self, 
            _0:                        &Rectangle<i32>,
            replace_existing_contents: bool);
}

pub trait FillRect {

    fn fill_rect(&mut self, _0: &Rectangle<f32>);
}

pub trait FillRectList {

    fn fill_rect_list(&mut self, _0: &RectangleList<f32>);
}

pub trait FillPath {

    fn fill_path(&mut self, 
            _0: &Path,
            _1: &AffineTransform);
}

pub trait DrawImage {

    fn draw_image(&mut self, 
            _0: &Image,
            _1: &AffineTransform);
}

pub trait DrawLine {

    fn draw_line(&mut self, _0: &Line<f32>);
}

pub trait SetFont {

    fn set_font(&mut self, _0: &Font);
}

pub trait GetFont {

    fn get_font(&mut self) -> &Font;
}

pub trait DrawGlyph {

    fn draw_glyph(
        &mut self, 
        glyph_number: i32,
        _1:           &AffineTransform
    );
}

pub trait DrawTextLayout {

    fn draw_text_layout(&mut self, 
        _0: &AttributedString,
        _1: &Rectangle<f32>) -> bool {
        
        false
    }
}

//----------------------------------------
pub trait CreateLowLevelContext {

    /**
      | Creates a context that will draw into
      | this image.
      |
      */
    fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext>;
}

impl CreateLowLevelContext for Image {

    /**
      | Creates a context suitable for drawing
      | onto this image.
      | 
      | Don't call this method directly! It's
      | used internally by the Graphics class.
      |
      */
    fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            if (image != nullptr)
            return image->createLowLevelContext();

        return {};
        */
    }
}

impl CreateLowLevelContext for SubsectionPixelData {

    fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            auto g = sourceImage->createLowLevelContext();
            g->clipToRectangle (area);
            g->setOrigin (area.getPosition());
            return g;
        */
    }
}

impl CreateLowLevelContext for SoftwarePixelData {

    fn create_low_level_context(&mut self) -> Box<dyn LowLevelGraphicsContext> {
        
        todo!();
        /*
            sendDataChangeMessage();
            return std::make_unique<LowLevelGraphicsSoftwareRenderer> (Image (*this));
        */
    }
}

pub trait ImagePixelDataInterface: 

CreateLowLevelContext {

    /**
      | Creates a copy of this image.
      |
      */
    fn clone(&mut self) -> ImagePixelDataPtr;

    /**
      | Creates an instance of the type of this
      | image.
      |
      */
    fn create_type(&self) -> Box<dyn ImageType>;

    /**
      | Initialises a BitmapData object.
      |
      */
    fn initialise_bitmap_data(&mut self, 
        _0: &mut ImageBitmapData,
        x:  i32,
        y:  i32,
        _3: ImageBitmapDataReadWriteMode);

    /**
      | Returns the number of Image objects
      | which are currently referring to the
      | same internal shared image data. This
      | is different to the reference count
      | as an instance of ImagePixelData can
      | internally depend on another ImagePixelData
      | via it's member variables.
      |
      */
    fn get_shared_count(&self) -> i32;
}

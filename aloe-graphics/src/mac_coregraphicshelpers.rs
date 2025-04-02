crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_mac_CoreGraphicsHelpers.h]

pub fn convert_to_rect_int<RectType>(r: RectType) -> Rectangle<i32> {

    todo!();
        /*
            return { (int) r.origin.x,
                     (int) r.origin.y,
                     (int) r.size.width,
                     (int) r.size.height };
        */
}

pub fn convert_to_rect_float<RectType>(r: RectType) -> Rectangle<f32> {

    todo!();
        /*
            return { (float) r.origin.x,
                     (float) r.origin.y,
                     (float) r.size.width,
                     (float) r.size.height };
        */
}

pub fn convert_to_cg_rect<RectType>(r: RectType) -> CGRect {

    todo!();
        /*
            return CGRectMake ((CGFloat) r.getX(), (CGFloat) r.getY(), (CGFloat) r.getWidth(), (CGFloat) r.getHeight());
        */
}

pub fn convert_to_point_float<PointType>(p: PointType) -> Point<f32> {

    todo!();
        /*
            return { (float) p.x, (float) p.y };
        */
}

pub fn convert_to_cg_point<PointType>(p: PointType) -> CGPoint {

    todo!();
        /*
            return CGPointMake ((CGFloat) p.x, (CGFloat) p.y);
        */
}

pub fn round_to_int_point<PointType>(p: PointType) -> Point<i32> {

    todo!();
        /*
            return { roundToInt (p.x), roundToInt (p.y) };
        */
}

#[cfg(target_os="macos")]
#[inline] pub fn get_main_screen_height() -> CGFloat {
    
    todo!();
        /*
            if ([[NSScreen screens] count] == 0)
                return 0.0f;

            return [[[NSScreen screens] objectAtIndex: 0] frame].size.height;
        */
}

#[cfg(target_os="macos")]
#[inline] pub fn flipped_screen_rect(r: NSRect) -> NSRect {
    
    todo!();
        /*
            r.origin.y = getMainScreenHeight() - (r.origin.y + r.size.height);
            return r;
        */
}

#[cfg(target_os="macos")]
#[inline] pub fn flipped_screen_point(p: NSPoint) -> NSPoint {
    
    todo!();
        /*
            p.y = getMainScreenHeight() - p.y;
            return p;
        */
}

#[cfg(target_os="macos")]
pub fn aloe_create_core_graphics_image(
        _0:                  &Image,
        _1:                  CGColorSpaceRef,
        must_outlive_source: bool) -> CGImageRef {
    
    todo!();
        /*
        
        */
}

pub fn aloe_get_image_context(_0: &Image) -> CGContextRef {
    
    todo!();
        /*
        
        */
}

#[cfg(target_os="ios")]
pub fn aloe_create_image_from_ui_image(_0: *mut UIImage) -> Image {
    
    todo!();
        /*
        
        */
}

#[cfg(target_os="macos")]
pub fn image_to_ns_image(
    image:        &Image,
    scale_factor: Option<f32>

) -> *mut objc::runtime::Object /* NSImage */ {

    let scale_factor: f32 = scale_factor.unwrap_or(1.0);

    todo!();
        /*
        
        */
}

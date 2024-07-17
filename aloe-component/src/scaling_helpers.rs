crate::ix!();

pub fn unscaled_screen_pos_to_scaled_with_point_or_rect<PointOrRect>(
        scale: f32,
        pos:   PointOrRect) -> PointOrRect {

    todo!();
    /*
        return scale != 1.0f ? pos / scale : pos;
    */
}

pub fn scaled_screen_pos_to_unscaled_with_point_or_rect_with_scale<PointOrRect>(
        scale: f32,
        pos:   PointOrRect) -> PointOrRect {

    todo!();
    /*
        return scale != 1.0f ? pos * scale : pos;
    */
}

pub fn scaled_screen_pos_to_unscaled<T: Num + Copy>(
    scale: f32,
    pos:   Rectangle<T>

) -> Rectangle<T> {
    
    todo!();
    /*
        return scale != 1.0f ? Rectangle<int> (roundToInt ((float) pos.getX() * scale),
                                                   roundToInt ((float) pos.getY() * scale),
                                                   roundToInt ((float) pos.getWidth() * scale),
                                                   roundToInt ((float) pos.getHeight() * scale)) : pos;
    */
    /*
        return scale != 1.0f ? Rectangle<float> (pos.getX() * scale,
                                                     pos.getY() * scale,
                                                     pos.getWidth() * scale,
                                                     pos.getHeight() * scale) : pos;
    */
}

/**
  | For these, we need to avoid getSmallestIntegerContainer
  | being used, which causes judder when
  | moving windows
  |
  */
pub fn unscaled_screen_pos_to_scaled<T: Num + Copy>(
    scale: f32,
    pos:   Rectangle<T>

) -> Rectangle<T> {
    
    todo!();
    /*
        return scale != 1.0f ? Rectangle<float> (pos.getX() / scale,
                                                     pos.getY() / scale,
                                                     pos.getWidth() / scale,
                                                     pos.getHeight() / scale) : pos;
    */
    /*
        return scale != 1.0f ? Rectangle<int> (roundToInt ((float) pos.getX() / scale),
                                                   roundToInt ((float) pos.getY() / scale),
                                                   roundToInt ((float) pos.getWidth() / scale),
                                                   roundToInt ((float) pos.getHeight() / scale)) : pos;
    */
}

pub fn unscaled_screen_pos_to_scaled_no_scale<PointOrRect>(pos: PointOrRect) -> PointOrRect {

    todo!();
    /*
        return unscaledScreenPosToScaled (Desktop::getInstance().getGlobalScaleFactor(), pos);
    */
}

pub fn scaled_screen_pos_to_unscaled_with_point_or_rect<PointOrRect>(pos: PointOrRect) -> PointOrRect {

    todo!();
    /*
        return scaledScreenPosToUnscaled (Desktop::getInstance().getGlobalScaleFactor(), pos);
    */
}

pub fn unscaled_screen_pos_to_scaled_with_component<'a, PointOrRect>(
        comp: &Component<'a>,
        pos:  PointOrRect) -> PointOrRect {

    todo!();
    /*
        return unscaledScreenPosToScaled (comp.getDesktopScaleFactor(), pos);
    */
}

pub fn scaled_screen_pos_to_unscaled_with_component<'a, PointOrRect>(
    comp: &Component<'a>,
    pos:  PointOrRect) -> PointOrRect {

    todo!();
    /*
        return scaledScreenPosToUnscaled (comp.getDesktopScaleFactor(), pos);
    */
}

pub fn add_position_to_point<'a, T: Num>(
        p: Point<T>,
        c: &Component<'a>) -> Point<T> {
    
    todo!();
    /*
        return p + c.getPosition();
    */
    /*
        return p + c.getPosition().toFloat();
    */
}

pub fn add_position_to_rectangle<'a, T: Num>(
        p: Rectangle<i32>,
        c: &Component<'a>) -> Rectangle<i32> {
    
    todo!();
    /*
        return p + c.getPosition();
    */
    /*
        return p + c.getPosition().toFloat();
    */
}

pub fn subtract_position_from_point<'a, T: Num>(
        p: Point<T>,
        c: &Component<'a>) -> Point<T> {
    
    todo!();
    /*
        return p - c.getPosition();
    */
    /*
        return p - c.getPosition().toFloat();
    */
}

pub fn subtract_position_from_rectangle<'a, T: Num + Copy>(
    p: Rectangle<T>,
    c: &Component<'a>

) -> Rectangle<T> {
    
    todo!();
    /*
        return p - c.getPosition();
    */
    /*
        return p - c.getPosition().toFloat();
    */
}
